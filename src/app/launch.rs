mod start;
mod update;

use crate::data::apps::App;
use crate::data::common;
use crate::data::common::{
    AppServer, ClientFileInfo, DataNode, FileInfo, FileType, ServerFileInfo, StartStatus, SyncTask,
    SyncTaskType,
};
use crate::data::core::AppDataPtr;
use crate::error::SyncError;
use crate::utils::filepath;
use crate::{error, requests, scan};
use image::Progress;
use log::{debug, trace, warn};
use std::error::Error;
use std::io::{Read, Write};
use std::os::unix;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, RecvError, Sender, TryRecvError};
use std::sync::{mpsc, Arc};
use std::time::Duration;
use std::{fs, io, thread};

enum LaunchMessage {
    UpdateCompleted,
    StartCompleted,
}

enum LaunchControlMessage {
    Start,
    Stop,
}

enum TaskEventMessage {
    Start,
    Progress(common::Progress),
    Stopped,
    Failed,
    Done,
}

pub fn launch(app_data_ptr: AppDataPtr, app: &App, app_server: &AppServer) {
    set_launch_status(
        Arc::clone(&app_data_ptr),
        app,
        app_server,
        StartStatus::StartHandle,
    );
    let data_dir_r = filepath::get_exe_dir();
    let mut data_dir;
    match data_dir_r {
        Ok(dir) => {
            data_dir = dir;
        }
        Err(_) => {
            // TODO
            panic!("");
        }
    }

    let dir;
    let path_buf = Path::new(&data_dir)
        .join(&app.uid)
        .join(&app_server.uid)
        .join(&app.uid);
    let p_o = path_buf.to_str();
    match p_o {
        None => {
            // TODO
            panic!("");
        }
        Some(p) => {
            dir = p;
        }
    }
    debug!("Launch app, dir: {}, app_server: {:?}", dir, app_server);

    // scan client file info
    let cfi_r = scan::scan(dir);
    let cfi;
    match cfi_r {
        Ok(v) => cfi = v,
        Err(e) => {
            debug!("scan failed, err: {:?}", e);
            match e {
                error::Error::ScanPathNotExitError => cfi = ClientFileInfo::default(),
                _ => {
                    set_launch_status(
                        Arc::clone(&app_data_ptr),
                        app,
                        app_server,
                        StartStatus::Failed,
                    );
                    return;
                }
            }
        }
    }

    // get server file info
    let sfi_r = requests::get_file_info(app_server);
    let sfi = match sfi_r {
        Ok(sfi) => sfi,
        Err(_) => {
            set_launch_status(
                Arc::clone(&app_data_ptr),
                app,
                app_server,
                StartStatus::Failed,
            );
            return;
        }
    };

    // diff file info
    let (added_files, changed_files, deleted_files) = diff_server_client(&sfi, &cfi);
    debug!(
        "sfi: {:?}, cfi: {:?}, added_files: {:?}, changed_files: {:?}, deleted_files: {:?}",
        sfi, cfi, added_files, changed_files, deleted_files
    );
    print_diff_detail(&sfi, &cfi, &added_files, &changed_files, &deleted_files);

    let base_path = dir.to_string();
    let data_nodes: Vec<DataNode>;
    let app_server_info_r = requests::get_info(app_server);
    match app_server_info_r {
        Ok(app_server_info) => {
            data_nodes = app_server_info
                .data_nodes
                .iter()
                .map(|x| DataNode::from(x))
                .collect();
        }
        Err(_) => {
            set_launch_status(
                Arc::clone(&app_data_ptr),
                app,
                app_server,
                StartStatus::Failed,
            );
            return;
        }
    }
    // use task channel
    let mut tasks = vec![];
    for fi in added_files {
        tasks.push(SyncTask::new(
            fi,
            SyncTaskType::Create,
            base_path.clone(),
            data_nodes.clone(),
        ));
    }
    for fi in changed_files {
        tasks.push(SyncTask::new(
            fi,
            SyncTaskType::Update,
            base_path.clone(),
            data_nodes.clone(),
        ));
    }
    for fi in deleted_files {
        tasks.push(SyncTask::new(
            fi,
            SyncTaskType::Delete,
            base_path.clone(),
            data_nodes.clone(),
        ));
    }

    let (launch_tx, launch_rx) = mpsc::channel::<LaunchMessage>();
    let (launch_control_tx, launch_control_rx) = mpsc::channel::<LaunchControlMessage>();
    let (task_event_tx, task_event_rx) = mpsc::channel::<TaskEventMessage>();

    thread::spawn(move || {
        start_tasks(tasks, launch_control_rx, task_event_tx);
    });

    let app_copy = app.clone();
    let app_server_copy = app_server.clone();
    let app_data_ptr_copy = Arc::clone(&app_data_ptr);
    let launch_tx_1 = launch_tx.clone();
    thread::spawn(move || {
        watch_task(
            app_data_ptr_copy,
            &app_copy,
            &app_server_copy,
            task_event_rx,
            launch_tx_1,
        );
    });

    let app_copy_2 = app.clone();
    let app_server_copy_2 = app_server.clone();
    let app_data_ptr_copy_2 = Arc::clone(&app_data_ptr);
    let launch_tx_2 = launch_tx.clone();
    thread::spawn(move || {
        watch_launch(
            app_data_ptr_copy_2,
            &app_copy_2,
            &app_server_copy_2,
            launch_rx,
            launch_tx_2,
        );
    });

    launch_control_tx.send(LaunchControlMessage::Start).unwrap();
}

fn start_tasks(
    tasks: Vec<SyncTask>,
    launch_control_rx: Receiver<LaunchControlMessage>,
    task_event_tx: Sender<TaskEventMessage>,
) {
    loop {
        let m_r = launch_control_rx.recv();
        match m_r {
            Ok(m) => match m {
                LaunchControlMessage::Start => {
                    debug!("recv ControlMessage::Start");
                    break;
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
    for (index, task) in tasks.iter().enumerate() {
        let m_r = launch_control_rx.try_recv();
        match m_r {
            Ok(m) => match m {
                LaunchControlMessage::Stop => {
                    debug!("recv ControlMessage::Stop");

                    // stop task
                    thread::sleep(Duration::from_secs(1));

                    task_event_tx.send(TaskEventMessage::Stopped).unwrap();
                    return;
                }
                _ => {}
            },
            Err(_) => {
                // debug!("recv ControlMessage Err");
            }
        }

        let r = task_event_tx.send(TaskEventMessage::Progress(common::Progress {
            v: index,
            total: tasks.len(),
            task: task.clone(),
        }));
        if let Err(e) = r {
            warn!("send TaskEventMessage::Progress failed, err: {}", e);
            return;
        }
        // handle task
        let r = handle_task(task);
        if let Err(e) = r {
            task_event_tx.send(TaskEventMessage::Failed).unwrap();
            return;
        }
    }

    task_event_tx.send(TaskEventMessage::Done).unwrap();
}

fn watch_task(
    app_data_ptr: AppDataPtr,
    app: &App,
    app_server: &AppServer,
    task_event_rx: Receiver<TaskEventMessage>,
    launch_tx: Sender<LaunchMessage>,
) {
    loop {
        let task_message_r = task_event_rx.recv();
        match task_message_r {
            Ok(m) => match m {
                TaskEventMessage::Start => {
                    debug!("TaskMessage::Start");
                }
                TaskEventMessage::Progress(p) => {
                    trace!("TaskMessage::Progress, {:?}", p);
                    set_launch_status(
                        Arc::clone(&app_data_ptr),
                        app,
                        app_server,
                        StartStatus::Updating(p),
                    );
                }
                TaskEventMessage::Stopped => {
                    debug!("TaskMessage::Stopped ");
                    set_launch_status(
                        Arc::clone(&app_data_ptr),
                        app,
                        app_server,
                        StartStatus::Cancelled,
                    );
                    break;
                }
                TaskEventMessage::Failed => {
                    debug!("TaskMessage::Failed ");
                    set_launch_status(
                        Arc::clone(&app_data_ptr),
                        app,
                        app_server,
                        StartStatus::Failed,
                    );

                    break;
                }
                TaskEventMessage::Done => {
                    debug!("TaskMessage::Done ");
                    set_launch_status(
                        Arc::clone(&app_data_ptr),
                        app,
                        app_server,
                        StartStatus::UpdateCompleted,
                    );
                    launch_tx.send(LaunchMessage::UpdateCompleted);
                    break;
                }
            },
            Err(_) => {
                debug!("recv TaskMessage Err");
            }
        }
        // thread::sleep(Duration::from_secs(1));
    }
}

fn watch_launch(
    app_data_ptr: AppDataPtr,
    app: &App,
    app_server: &AppServer,
    launch_rx: Receiver<LaunchMessage>,
    launch_tx: Sender<LaunchMessage>,
) {
    loop {
        let launch_message_r = launch_rx.recv();
        match launch_message_r {
            Ok(launch_message) => match launch_message {
                LaunchMessage::UpdateCompleted => {
                    launch_app(
                        Arc::clone(&app_data_ptr),
                        app,
                        app_server,
                        launch_tx.clone(),
                    );
                }
                LaunchMessage::StartCompleted => {
                    debug!("Start completed");
                    return;
                }
            },
            Err(_) => {
                // debug!("recv LaunchMessage Err");
            }
        }
    }
}

fn handle_task(task: &SyncTask) -> Result<(), SyncError> {
    debug!("handel task: {:?}", task);
    thread::sleep(Duration::from_millis(1000));
    match task.sync_type {
        SyncTaskType::Create | SyncTaskType::Update => {
            debug!("will sync, file_info: {:?}", task.file_info);

            let full_file_path = Path::new(&task.base_path).join(&task.file_info.relative_path);
            let r = delete_file(&full_file_path);
            if let Err(e) = r {
                return Err(e);
            }

            match task.file_info.file_type {
                FileType::Unknown => {
                    return Err(SyncError::UnknownFileType);
                }
                FileType::File => {
                    let url = format!(
                        "{}/api/v1/download/{}",
                        task.data_nodes.get(0).unwrap().address.to_address_string(),
                        task.file_info.relative_path
                    );
                    let resp_r = reqwest::blocking::get(url);
                    match resp_r {
                        Ok(mut resp) => {
                            let f_r = fs::File::create(&full_file_path);
                            match f_r {
                                Ok(f) => {
                                    let mut writer = io::BufWriter::new(f);
                                    let mut buf = [0; 1024];
                                    while true {
                                        let r = resp.read(&mut buf);
                                        match r {
                                            Ok(n) => {
                                                if n == 0 {
                                                    break;
                                                }
                                                let r = writer.write(&buf[..n]);
                                                if let Err(e) = r {
                                                    return Err(
                                                        SyncError::ReadDownloadContentFailed,
                                                    );
                                                }
                                            }
                                            Err(_) => {
                                                return Err(SyncError::ReadDownloadContentFailed);
                                            }
                                        }
                                    }
                                    let r = writer.flush();
                                    if let Err(e) = r {
                                        return Err(SyncError::CreateFileFailed);
                                    }
                                }
                                Err(_) => {
                                    return Err(SyncError::CreateFileFailed);
                                }
                            }
                        }
                        Err(_) => {
                            return Err(SyncError::DownloadFailed);
                        }
                    }
                }
                FileType::Dir => {
                    let create_dir_r = fs::create_dir_all(&full_file_path);
                    if let Err(e) = create_dir_r {
                        warn!(
                            "create dir failed, full_file_path: {:?}, err: {}",
                            full_file_path, e
                        );
                        return Err(SyncError::CreateDirFailed);
                    }
                }
                FileType::Symlink => {
                    let mut content = "".to_string();
                    let url = format!(
                        "{}/api/v1/download/{}",
                        task.data_nodes.get(0).unwrap().address.to_address_string(),
                        task.file_info.relative_path
                    );
                    let resp_r = reqwest::blocking::get(url);
                    match resp_r {
                        Ok(mut resp) => {
                            let read_r = resp.read_to_string(&mut content);
                            if let Err(e) = read_r {
                                return Err(SyncError::ReadDownloadContentFailed);
                            }
                        }
                        Err(_) => {
                            return Err(SyncError::DownloadFailed);
                        }
                    }
                    let create_symlink_r = unix::fs::symlink(&content, &full_file_path);
                    if let Err(e) = create_symlink_r {
                        warn!(
                            "create symlink failed, full_file_path: {:?}, err: {}",
                            full_file_path, e
                        );
                        return Err(SyncError::CreateSymlinkFailed);
                    }
                }
            }
        }
        SyncTaskType::Delete => {
            debug!("will delete, file_info: {:?}", task.file_info);
            let full_file_path = Path::new(&task.base_path).join(&task.file_info.relative_path);
            debug!(
                "will delete, file_info: {:?}, full_file_path: {:?}",
                task.file_info, full_file_path
            );
            let r = delete_file(&full_file_path);
            if let Err(e) = r {
                return Err(e);
            }
        }
    }
    Ok(())
}

fn launch_app(
    app_data_ptr: AppDataPtr,
    app: &App,
    app_server: &AppServer,
    launch_tx: Sender<LaunchMessage>,
) {
    debug!("Launch app");
    set_launch_status(
        Arc::clone(&app_data_ptr),
        app,
        app_server,
        StartStatus::Starting,
    );

    thread::sleep(Duration::from_millis(1000));

    set_launch_status(
        Arc::clone(&app_data_ptr),
        app,
        app_server,
        StartStatus::Started,
    );
    launch_tx.send(LaunchMessage::StartCompleted);
}

fn set_launch_status(
    app_data_ptr: AppDataPtr,
    app: &App,
    app_server: &AppServer,
    status: StartStatus,
) {
    let mut app_data_g = app_data_ptr.lock().unwrap();
    app_data_g
        .app_manager
        .apps
        .get_mut(Box::leak(app.uid.clone().into_boxed_str()))
        .unwrap()
        .app_server_info
        .servers
        .get_mut(&app_server.uid)
        .unwrap()
        .start_status = status;
    drop(app_data_g);
}

fn diff_server_client(
    sfi: &ServerFileInfo,
    cfi: &ClientFileInfo,
) -> (Vec<FileInfo>, Vec<FileInfo>, Vec<FileInfo>) {
    let mut del_files: Vec<FileInfo> = vec![];
    let mut add_files: Vec<FileInfo> = vec![];
    let mut changed_files: Vec<FileInfo> = vec![];

    for cf in &cfi.files {
        if !is_in(cf, &sfi.files) {
            del_files.push(cf.clone());
        }
    }
    for sf in &sfi.files {
        if !is_in(sf, &cfi.files) {
            add_files.push(sf.clone());
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
    (add_files, changed_files, del_files)
}

fn is_in(f: &FileInfo, fs: &Vec<FileInfo>) -> bool {
    let f_path = Path::new(&f.relative_path);
    let mut flag = false;
    for x in fs {
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
    add_files: &Vec<FileInfo>,
    changed_files: &Vec<FileInfo>,
    del_files: &Vec<FileInfo>,
) {
    print_file_info(&sfi.files, "server");
    print_file_info(&cfi.files, "client");
    print_file_info(&add_files, "add_files");
    print_file_info(&changed_files, "changed_files");
    print_file_info(&del_files, "del_files");
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

fn delete_file(full_file_path: &PathBuf) -> Result<(), SyncError> {
    let full_file_path_exists_r = full_file_path.try_exists();
    match full_file_path_exists_r {
        Ok(exists) => {
            let is_symlink = full_file_path.is_symlink();
            // symlink is special!
            if is_symlink {
                warn!("[DEL][Symlink]{:?}", full_file_path);
                let r = fs::remove_file(&full_file_path);
                if let Err(e) = r {
                    warn!("[DEL][Symlink][Failed]{:?}, err: {}", full_file_path, e);
                    return Err(SyncError::DeleteFailed);
                }
            } else {
                if exists {
                    if is_symlink {
                        // symlink is special!
                    } else if full_file_path.is_dir() {
                        warn!("[DEL][Dir]{:?}", full_file_path);
                        let r = fs::remove_dir(&full_file_path);
                        if let Err(e) = r {
                            warn!("[DEL][Dir][Failed]{:?}, err: {}", full_file_path, e);
                            return Err(SyncError::DeleteFailed);
                        }
                    } else if full_file_path.is_file() {
                        warn!("[DEL][File]{:?}", full_file_path);
                        let r = fs::remove_file(&full_file_path);
                        if let Err(e) = r {
                            warn!("[DEL][File][Failed]{:?}, err: {}", full_file_path, e);
                            return Err(SyncError::DeleteFailed);
                        }
                    } else {
                        return Err(SyncError::UnknownFileType);
                    }
                }
            }
        }
        Err(_) => {
            return Err(SyncError::CheckExistsFailed);
        }
    }
    Ok(())
}
