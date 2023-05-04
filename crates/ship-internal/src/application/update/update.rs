use crate::application::app::AppManager;
use crate::application::common::get_data_path_by_app_server_id;
use crate::application::settings::SettingsManager;
use crate::application::update::sync::{SyncTask, SyncTaskType};
use crate::application::update::update_manage::UpdateManager;
use crate::application::update::{
    Error, Progress, TaskControlMessage, UpdateTaskControlMessage, UpdateTaskStatus,
    UpdateTaskTraceMessage,
};
use crate::application::{scan, update};
use crate::request;
use crate::types::common::{ClientFileInfo, DataNode, FileInfo, ServerFileInfo};
use log::{debug, info, warn};
use std::path::Path;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

pub fn handle_update_control(
    message: UpdateTaskControlMessage,
    update_manager: Arc<Mutex<UpdateManager>>,
    app_manager: Arc<Mutex<AppManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
) {
    debug!(
        "start handle_update, UpdateTaskControlMessage: {:?}",
        message
    );

    match message {
        UpdateTaskControlMessage::Start { app_server_id } => {
            // remove last UpdateTask by app_server_id, if exist
            let mut update_manager_g = update_manager.lock().unwrap();
            let update_task_o = update_manager_g.get_update_task_by_app_server_id(app_server_id);
            match update_task_o {
                None => {}
                Some(update_task) => {
                    let id = update_task.id;
                    update_manager_g.update_tasks.remove(&id);
                }
            }
            drop(update_manager_g);

            if let Err(e) =
                handle_task(app_server_id, update_manager, app_manager, settings_manager)
            {
                warn!("handle update_task failed, err: {:?}", e);
            }
        }
        UpdateTaskControlMessage::Stop { app_server_id } => {
            let mut update_manager_g = update_manager.lock().unwrap();
            let update_task_o =
                update_manager_g.get_mut_update_task_by_app_server_id(app_server_id);
            match update_task_o {
                None => {}
                Some(update_task) => {
                    if let Err(e) = update_task.tx.send(TaskControlMessage::Stop) {
                        warn!(
                            "send TaskControlMessage::Stop to channel failed, err: {}",
                            e
                        );
                    }
                }
            }
            drop(update_manager_g);
        }
    }
}

fn handle_task(
    app_server_id: u64,
    update_manager: Arc<Mutex<UpdateManager>>,
    app_manager: Arc<Mutex<AppManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
) -> Result<(), Error> {
    let mut update_manager_g = update_manager.lock().unwrap();
    let task = update_manager_g.create_task(app_server_id);
    let task_id = task.id;
    task.tx
        .send(TaskControlMessage::Start)
        .map_err(|_| Error::SendControlMessageFailed)?;
    if let Err(e) = update_manager_g.add_task(task) {
        warn!("add update task failed, task_id: {}", task_id);
        return Err(e);
    }
    drop(update_manager_g);
    let (trace_tx, trace_rx) = mpsc::channel::<UpdateTaskTraceMessage>();
    let update_manager_2 = Arc::clone(&update_manager);
    thread::spawn(move || {
        watch_trace(trace_rx, app_server_id, update_manager_2);
    });
    trace_tx
        .send(UpdateTaskTraceMessage::Wait)
        .map_err(|_| Error::SendTraceMessageFailed)?;
    let r = do_handle_task(
        app_server_id,
        update_manager,
        app_manager,
        settings_manager,
        task_id,
        trace_tx.clone(),
    );
    if let Err(_) = &r {
        trace_tx
            .send(UpdateTaskTraceMessage::Failed)
            .map_err(|_| Error::SendTraceMessageFailed)?;
    };
    r
}
fn do_handle_task(
    app_server_id: u64,
    update_manager: Arc<Mutex<UpdateManager>>,
    app_manager: Arc<Mutex<AppManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
    task_id: u64,
    trace_tx: Sender<UpdateTaskTraceMessage>,
) -> Result<(), Error> {
    // check start
    loop {
        thread::sleep(Duration::from_millis(100));
        let m_o = get_control_message(task_id, Arc::clone(&update_manager))?;
        if let Some(message) = m_o {
            match message {
                TaskControlMessage::Start => {
                    debug!("get message: {:?}", message);
                    break;
                }
                _ => {}
            }
        }
    }

    let data_path_r = get_data_path_by_app_server_id(
        app_server_id,
        Arc::clone(&app_manager),
        Arc::clone(&settings_manager),
    );
    let data_path = match data_path_r {
        Ok(p) => p,
        Err(_) => {
            warn!("get_data_path_by_app_server_id failed");
            return Err(Error::GetDataPathFailed);
        }
    };
    debug!("app_server_id: {}, data_path: {}", app_server_id, data_path);
    if let Err(_) = fs::create_dir_all(&data_path) {
        warn!("create dir failed, path: {}", data_path);
        return Err(Error::CreateDirFailed);
    }

    let mut app_id = 0;
    let mut address = "".to_string();
    let app_manager_g = app_manager.lock().unwrap();
    let mut is_found = false;
    'outer: for (_, app) in &app_manager_g.apps {
        for (_, app_server) in &app.app_server_info.servers {
            if app_server.id == app_server_id {
                is_found = true;
                app_id = app.id;
                address = app_server.address.to_address_string();
                break 'outer;
            }
        }
    }

    drop(app_manager_g);
    if !is_found {
        warn!(
            "get app_server info failed, app_server_id: {}",
            app_server_id
        );
        return Err(Error::GetAppServerFailed);
    }

    // get server files
    trace_tx
        .send(UpdateTaskTraceMessage::GetServerUpdateInfo)
        .map_err(|_| Error::SendTraceMessageFailed)?;
    let sfi_r = request::app_server::file_info::get_file_info(&address);
    let sfi = match sfi_r {
        Ok(x) => ServerFileInfo::from(&x),
        Err(e) => {
            warn!(
                "get ServerFileInfo failed, app_server_id: {}, err: {:?}",
                app_server_id, e
            );
            return Err(Error::GetServerFileInfoFailed);
        }
    };

    // scan local files
    trace_tx
        .send(UpdateTaskTraceMessage::GetClientFileInfo)
        .map_err(|_| Error::SendTraceMessageFailed)?;
    let cfi_r = scan::scan(&data_path);
    let cfi = match cfi_r {
        Ok(x) => x,
        Err(e) => {
            warn!(
                "get ClientFileInfo failed, app_server_id: {}, err: {:?}",
                app_server_id, e
            );
            return Err(Error::GetClientFileInfoFailed);
        }
    };

    // diff files
    let (added_files, changed_files, deleted_files) = diff_files(&cfi, &sfi);
    debug!(
        "sfi: {:?}, cfi: {:?}, added_files: {:?}, changed_files: {:?}, deleted_files: {:?}",
        sfi, cfi, added_files, changed_files, deleted_files
    );
    print_diff_detail(&sfi, &cfi, &added_files, &changed_files, &deleted_files);

    let app_server_info_r = request::app_server::app_server_info::get_app_server_info(&address);
    let data_nodes = match app_server_info_r {
        Ok(app_server_info) => app_server_info
            .data_nodes
            .iter()
            .map(|x| DataNode::from(x))
            .collect(),
        Err(e) => {
            warn!(
                "get data_nodes failed, app_server_id: {}, err: {:?}",
                app_server_id, e
            );
            return Err(Error::GetDataNodesFailed);
        }
    };

    let sync_tasks = generate_sync_tasks(
        app_id,
        &added_files,
        &changed_files,
        &deleted_files,
        &data_path,
        &data_nodes,
    );

    let total = added_files.len() + changed_files.len();

    let (sync_task_tx, sync_task_rx) = mpsc::channel::<SyncTask>();
    debug!("add SyncTask to channel");
    for x in sync_tasks {
        let r = sync_task_tx.send(x);
        if let Err(e) = r {
            warn!("send SyncTask to channel failed, err: {}", e);
            return Err(Error::AddSyncTaskFailed);
        }
    }
    // close and rx will recv disconnect err when channel empty
    drop(sync_task_tx);

    let mut count = 0; // exclude deleted SyncTask (added + changed)
    loop {
        thread::sleep(Duration::from_millis(10));

        let m_o = get_control_message(task_id, Arc::clone(&update_manager))?;
        if let Some(message) = m_o {
            match message {
                TaskControlMessage::Stop => {
                    debug!("get message: {:?}", message);
                    trace_tx
                        .send(UpdateTaskTraceMessage::Canceled)
                        .map_err(|_| Error::SendTraceMessageFailed)?;
                    break;
                }
                _ => {}
            }
        }

        // handle
        debug!("get SyncTask");
        let sync_task_r = sync_task_rx.recv_timeout(Duration::from_millis(100));
        match sync_task_r {
            Ok(sync_task) => {
                match sync_task.sync_type {
                    SyncTaskType::Create | SyncTaskType::Update => {
                        count += 1;
                    }
                    _ => {}
                }
                trace_tx
                    .send(UpdateTaskTraceMessage::Processing {
                        progress: Progress::new(count, total as u64),
                        sync_task: sync_task.clone(),
                    })
                    .map_err(|_| Error::SendTraceMessageFailed)?;
                if let Err(e) = update::sync::handle_task(sync_task) {
                    warn!("handle SyncTask failed, err: {:?}", e);
                    return Err(Error::HandleSyncTaskFailed);
                }
            }
            Err(e) => match e {
                RecvTimeoutError::Timeout => {
                    // debug!("timeout");
                }
                RecvTimeoutError::Disconnected => {
                    info!("all sync task finished, app_server_id: {}", app_server_id);
                    trace_tx
                        .send(UpdateTaskTraceMessage::Finished {
                            finish_time: chrono::Utc::now().timestamp(),
                        })
                        .map_err(|_| Error::SendTraceMessageFailed)?;
                    return Ok(());
                }
            },
        }
    }

    Ok(())
}

fn get_control_message(
    task_id: u64,
    update_manager: Arc<Mutex<UpdateManager>>,
) -> Result<Option<TaskControlMessage>, Error> {
    let update_manager_g = update_manager.lock().unwrap();
    let task_o = update_manager_g.get_update_task_by_id(task_id);
    match task_o {
        None => {
            drop(update_manager_g);
            warn!("task not exist, id: {}", task_id);
            return Err(Error::TaskNotExist);
        }
        Some(task) => {
            let m_r = task.rx.try_recv();
            drop(update_manager_g);
            if let Ok(m) = m_r {
                return Ok(Some(m));
            }
        }
    }
    Ok(None)
}

fn watch_trace(
    rx: Receiver<UpdateTaskTraceMessage>,
    app_server_id: u64,
    update_manager: Arc<Mutex<UpdateManager>>,
) {
    loop {
        thread::sleep(Duration::from_millis(1));
        let message_r = rx.recv();
        match message_r {
            Ok(message) => {
                debug!("trace: {:?}", message);

                let mut update_manager_g = update_manager.lock().unwrap();
                let task_o = update_manager_g.get_mut_update_task_by_app_server_id(app_server_id);
                match task_o {
                    None => {
                        drop(update_manager_g);
                        warn!("UpdateTask not exist, app_server_id: {}", app_server_id);
                        break;
                    }
                    Some(task) => {
                        match message {
                            UpdateTaskTraceMessage::Wait => {
                                task.status = UpdateTaskStatus::Wait;
                            }
                            UpdateTaskTraceMessage::GetServerUpdateInfo => {
                                task.status = UpdateTaskStatus::GetServerUpdateInfo;
                            }
                            UpdateTaskTraceMessage::GetClientFileInfo => {
                                task.status = UpdateTaskStatus::GetClientFileInfo;
                            }
                            UpdateTaskTraceMessage::Processing {
                                progress,
                                sync_task,
                            } => {
                                task.status = UpdateTaskStatus::Processing {
                                    progress,
                                    sync_task,
                                };
                            }
                            UpdateTaskTraceMessage::Canceled => {
                                task.status = UpdateTaskStatus::Canceled;
                            }
                            UpdateTaskTraceMessage::Failed => {
                                task.status = UpdateTaskStatus::Failed;
                            }
                            UpdateTaskTraceMessage::Finished { finish_time } => {
                                task.status = UpdateTaskStatus::Finished { finish_time };
                            }
                        }
                        drop(update_manager_g);
                    }
                }
            }
            Err(_) => {
                debug!("watch trace exit");
                break;
            }
        }
    }
}

fn diff_files(
    cfi: &ClientFileInfo,
    sfi: &ServerFileInfo,
) -> (Vec<FileInfo>, Vec<FileInfo>, Vec<FileInfo>) {
    let mut added_files: Vec<FileInfo> = vec![];
    let mut changed_files: Vec<FileInfo> = vec![];
    let mut deleted_files: Vec<FileInfo> = vec![];

    for cf in &cfi.files {
        if !is_in(cf, &sfi.files) {
            deleted_files.push(cf.clone());
        }
    }
    for sf in &sfi.files {
        if !is_in(sf, &cfi.files) {
            added_files.push(sf.clone());
        }
    }
    for sf in &sfi.files {
        let sf_path = Path::new(&sf.relative_path);
        for cf in &cfi.files {
            let cf_path = Path::new(&cf.relative_path);
            if cf_path.eq(sf_path) {
                if cf.file_type != sf.file_type || cf.size != sf.size || cf.hash != sf.hash {
                    changed_files.push(sf.clone());
                }
                break;
            }
        }
    }
    (added_files, changed_files, deleted_files)
}

fn is_in(f: &FileInfo, files: &Vec<FileInfo>) -> bool {
    let f_path = Path::new(&f.relative_path);
    let mut flag = false;
    for x in files {
        let x_path = Path::new(&x.relative_path);
        if x_path.eq(f_path) {
            flag = true;
            break;
        }
    }
    if flag {
        return true;
    }
    false
}

fn print_diff_detail(
    sfi: &ServerFileInfo,
    cfi: &ClientFileInfo,
    added_files: &Vec<FileInfo>,
    changed_files: &Vec<FileInfo>,
    deleted_files: &Vec<FileInfo>,
) {
    print_file_info(&sfi.files, "server");
    print_file_info(&cfi.files, "client");
    print_file_info(&added_files, "added_files");
    print_file_info(&changed_files, "changed_files");
    print_file_info(&deleted_files, "deleted_files");
}

fn print_file_info(fi: &Vec<FileInfo>, s: &str) {
    debug!("------- {} -------", s);
    for f in fi {
        debug!(
            "type: {}, hash: {:32}, size: {:10}, rel_path: {}",
            f.file_type.to_formatted_string(),
            f.hash,
            f.size,
            f.relative_path
        );
    }
    debug!("------- {} -------", s);
}

fn generate_sync_tasks(
    app_id: u64,
    added_files: &Vec<FileInfo>,
    changed_files: &Vec<FileInfo>,
    deleted_files: &Vec<FileInfo>,
    base_path: &str,
    data_nodes: &Vec<DataNode>,
) -> Vec<SyncTask> {
    let mut tasks = vec![];
    for fi in added_files {
        tasks.push(SyncTask::new(
            app_id,
            SyncTaskType::Create,
            fi.clone(),
            base_path.to_string(),
            data_nodes.clone(),
        ));
    }
    for fi in changed_files {
        tasks.push(SyncTask::new(
            app_id,
            SyncTaskType::Update,
            fi.clone(),
            base_path.to_string(),
            data_nodes.clone(),
        ));
    }
    for fi in deleted_files {
        tasks.push(SyncTask::new(
            app_id,
            SyncTaskType::Delete,
            fi.clone(),
            base_path.to_string(),
            data_nodes.clone(),
        ));
    }
    tasks
}
