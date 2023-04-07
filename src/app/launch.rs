use crate::data::apps::App;
use crate::data::common;
use crate::data::common::{AppServer, StartStatus, SyncTask};
use crate::data::core::AppDataPtr;
use crate::utils::filepath;
use image::Progress;
use log::debug;
use std::error::Error;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

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
        .start_status = StartStatus::StartHandle;
    drop(app_data_g);
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
    let path_buf = Path::new(&data_dir).join(&app.uid).join(&app_server.uid);
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

    // TODO scan client file info

    // TODO get server file info

    // TODO diff file info

    // TODO use task channel
    let mut tasks = vec![];
    for i in 0..1000 {
        tasks.push(SyncTask {
            relative_file_path: format!("foo/bar/file-{}", i),
        });
    }

    let (launch_control_tx, launch_control_rx) = mpsc::channel::<LaunchControlMessage>();
    let (task_event_tx, task_event_rx) = mpsc::channel::<TaskEventMessage>();

    thread::spawn(move || {
        start_tasks(tasks, launch_control_rx, task_event_tx);
    });

    let app_copy = app.clone();
    let app_server_copy = app_server.clone();
    thread::spawn(move || {
        watch_task(app_data_ptr, &app_copy, &app_server_copy, task_event_rx);
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
                debug!("recv ControlMessage Err");
            }
        }

        // handle task
        task_event_tx
            .send(TaskEventMessage::Progress(common::Progress {
                v: index,
                total: tasks.len(),
                task: task.clone(),
            }))
            .unwrap();
        thread::sleep(Duration::from_millis(10));
    }

    task_event_tx.send(TaskEventMessage::Done).unwrap();
}

fn watch_task(
    app_data_ptr: AppDataPtr,
    app: &App,
    app_server: &AppServer,
    task_event_rx: Receiver<TaskEventMessage>,
) {
    loop {
        let task_message_r = task_event_rx.recv();
        match task_message_r {
            Ok(m) => {
                let mut app_data_g = app_data_ptr.lock().unwrap();
                match m {
                    TaskEventMessage::Start => {
                        debug!("TaskMessage::Start");
                    }
                    TaskEventMessage::Progress(p) => {
                        debug!("TaskMessage::Progress");
                        app_data_g
                            .app_manager
                            .apps
                            .get_mut(Box::leak(app.uid.clone().into_boxed_str()))
                            .unwrap()
                            .app_server_info
                            .servers
                            .get_mut(&app_server.uid)
                            .unwrap()
                            .start_status = StartStatus::Updating(p);
                    }
                    TaskEventMessage::Stopped => {
                        debug!("TaskMessage::Stopped ");
                        app_data_g
                            .app_manager
                            .apps
                            .get_mut(Box::leak(app.uid.clone().into_boxed_str()))
                            .unwrap()
                            .app_server_info
                            .servers
                            .get_mut(&app_server.uid)
                            .unwrap()
                            .start_status = StartStatus::Cancelled;
                        break;
                    }
                    TaskEventMessage::Failed => {
                        debug!("TaskMessage::Failed ");
                        app_data_g
                            .app_manager
                            .apps
                            .get_mut(Box::leak(app.uid.clone().into_boxed_str()))
                            .unwrap()
                            .app_server_info
                            .servers
                            .get_mut(&app_server.uid)
                            .unwrap()
                            .start_status = StartStatus::Failed;

                        break;
                    }
                    TaskEventMessage::Done => {
                        debug!("TaskMessage::Done ");
                        app_data_g
                            .app_manager
                            .apps
                            .get_mut(Box::leak(app.uid.clone().into_boxed_str()))
                            .unwrap()
                            .app_server_info
                            .servers
                            .get_mut(&app_server.uid)
                            .unwrap()
                            .start_status = StartStatus::UpdateCompleted;
                        break;
                    }
                }

                drop(app_data_g);
            }
            Err(_) => {
                debug!("recv TaskMessage Err");
            }
        }
        // thread::sleep(Duration::from_secs(1));
    }
}

fn check_update() {}

fn launch_app() {}
