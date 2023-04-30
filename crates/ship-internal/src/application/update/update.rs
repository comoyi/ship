use crate::application::app::app_server::AppServer;
use crate::application::app::AppManager;
use crate::application::scan;
use crate::application::scan::Error;
use crate::application::settings::SettingsManager;
use crate::application::update::update_manage::UpdateManager;
use crate::application::update::{TaskControlMessage, UpdateTask, UpdateTaskControlMessage};
use crate::request;
use crate::request::app_server::get_file_info::ServerFileInfoVo;
use crate::types::common::ClientFileInfo;
use log::{debug, warn};
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex};
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
            handle_task(app_server_id, update_manager, app_manager, settings_manager);
        }
        UpdateTaskControlMessage::Stop { app_server_id } => {}
    }
}

fn handle_task(
    app_server_id: u64,
    update_manager: Arc<Mutex<UpdateManager>>,
    app_manager: Arc<Mutex<AppManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
) {
    let mut update_manager_g = update_manager.lock().unwrap();
    let task = update_manager_g.create_task(app_server_id);
    let task_id = task.id;
    task.tx.send(TaskControlMessage::Start);
    update_manager_g.add_task(task);
    drop(update_manager_g);

    // check start
    loop {
        thread::sleep(Duration::from_millis(100));
        let update_manager_g = update_manager.lock().unwrap();
        let task_o = update_manager_g.get_update_task_by_id(task_id);
        match task_o {
            None => {
                drop(update_manager_g);
                warn!("task not exist, id: {}", task_id);
                return;
            }
            Some(task) => {
                let m_r = task.rx.try_recv();
                drop(update_manager_g);
                match m_r {
                    Ok(message) => match message {
                        TaskControlMessage::Start => {
                            debug!("get message: {:?}", message);
                            break;
                        }

                        _ => {}
                    },
                    Err(_) => {}
                }
            }
        }
    }

    let data_path_r =
        get_data_path_by_app_server_id(app_server_id, Arc::clone(&app_manager), settings_manager);
    let data_path = match data_path_r {
        Ok(p) => p,
        Err(_) => {
            warn!("get_data_path_by_app_server_id failed");
            return;
        }
    };
    debug!("app_server_id: {}, data_path: {}", app_server_id, data_path);
    if let Err(e) = fs::create_dir_all(&data_path) {
        warn!("create dir failed, path: {}", data_path);
        return;
    }

    let mut address = "".to_string();
    let app_manager_g = app_manager.lock().unwrap();
    let mut is_found = false;
    'outer: for (_, app) in &app_manager_g.apps {
        for (_, app_server) in &app.app_server_info.servers {
            if app_server.id == app_server_id {
                is_found = true;
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
        return;
    }

    // get server files
    let sfi_r = request::app_server::get_file_info::get_file_info(&address);
    let sfi = match sfi_r {
        Ok(x) => x,
        Err(e) => {
            warn!(
                "get ServerFileInfo failed, app_server_id: {}, err: {:?}",
                app_server_id, e
            );
            return;
        }
    };

    // scan local files
    let cfi_r = scan::scan(&data_path);
    let cfi = match cfi_r {
        Ok(x) => x,
        Err(e) => {
            warn!(
                "get ClientFileInfo failed, app_server_id: {}, err: {:?}",
                app_server_id, e
            );
            return;
        }
    };

    loop {
        thread::sleep(Duration::from_millis(10));

        let update_manager_g = update_manager.lock().unwrap();
        let task_o = update_manager_g.get_update_task_by_id(task_id);
        match task_o {
            None => {
                drop(update_manager_g);
                warn!("task not exist, id: {}", task_id);
                return;
            }
            Some(task) => {
                let m_r = task.rx.try_recv();
                drop(update_manager_g);
                match m_r {
                    Ok(message) => match message {
                        TaskControlMessage::Stop => {
                            debug!("get message: {:?}", message);
                            break;
                        }

                        _ => {}
                    },
                    Err(_) => {}
                }
            }
        }

        // handle
    }
}

fn get_data_path_by_app_server_id<'a>(
    app_server_id: u64,
    app_manager: Arc<Mutex<AppManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
) -> Result<String, &'a str> {
    let settings_manager_g = settings_manager.lock().unwrap();
    let base_path = settings_manager_g
        .settings
        .general_settings
        .data_dir_path
        .clone();
    drop(settings_manager_g);
    let app_id = get_app_id(app_server_id, app_manager)?;
    Ok(format!("{}/{}/{}", base_path, app_id, app_server_id))
}

fn get_app_id<'a>(app_server_id: u64, app_manager: Arc<Mutex<AppManager>>) -> Result<u64, &'a str> {
    let app_manager_g = app_manager.lock().unwrap();
    let mut app_id = None;
    'outer: for (_, app) in &app_manager_g.apps {
        for (x_, app_server) in &app.app_server_info.servers {
            if app_server.id == app_server_id {
                app_id = Some(app.id);
                break 'outer;
            }
        }
    }
    drop(app_manager_g);
    app_id.ok_or("get app_id failed")
}
