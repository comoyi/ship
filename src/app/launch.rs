use crate::data::apps::App;
use crate::data::common;
use crate::data::common::{AppServer, StartStatus};
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

enum ControlMessage {
    Start,
    Stop,
}

enum TaskMessage {
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

    let (tx, rx) = mpsc::channel::<ControlMessage>();
    let (tx_task, rx_task) = mpsc::channel::<TaskMessage>();

    thread::spawn(move || {
        start_tasks(rx, tx_task);
    });

    let app_copy = app.clone();
    let app_server_copy = app_server.clone();
    thread::spawn(move || {
        watch_task(app_data_ptr, &app_copy, &app_server_copy, rx_task);
    });

    tx.send(ControlMessage::Start).unwrap();

    // thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(1));
    //     let _ = tx.send(ControlMessage::Stop);
    //     debug!("send ControlMessage::Stop");
    // });
}

fn start_tasks(rx: Receiver<ControlMessage>, tx_task: Sender<TaskMessage>) {
    let mut tasks = vec![];
    for i in 0..1000 {
        tasks.push(i);
    }
    loop {
        let m_r = rx.recv();
        match m_r {
            Ok(m) => match m {
                ControlMessage::Start => {
                    debug!("recv ControlMessage::Start");
                    break;
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
    for (index, task) in tasks.iter().enumerate() {
        let m_r = rx.try_recv();
        match m_r {
            Ok(m) => match m {
                ControlMessage::Stop => {
                    debug!("recv ControlMessage::Stop");

                    // stop task
                    thread::sleep(Duration::from_secs(1));

                    tx_task.send(TaskMessage::Stopped).unwrap();
                    return;
                }
                _ => {}
            },
            Err(_) => {
                debug!("recv ControlMessage Err");
            }
        }

        // handle task
        thread::sleep(Duration::from_millis(100));
        tx_task
            .send(TaskMessage::Progress(common::Progress {
                v: index,
                total: tasks.len(),
            }))
            .unwrap();
    }

    tx_task.send(TaskMessage::Done).unwrap();
}

fn watch_task(
    app_data_ptr: AppDataPtr,
    app: &App,
    app_server: &AppServer,
    rx_task: Receiver<TaskMessage>,
) {
    loop {
        let task_message_r = rx_task.recv();
        match task_message_r {
            Ok(m) => {
                let mut app_data_g = app_data_ptr.lock().unwrap();
                match m {
                    TaskMessage::Start => {
                        debug!("TaskMessage::Start");
                    }
                    TaskMessage::Progress(p) => {
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
                    TaskMessage::Stopped => {
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
                    TaskMessage::Failed => {
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
                    TaskMessage::Done => {
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
