mod sync;
mod update;
pub mod update_manage;

use crate::application::app::AppManager;
use crate::application::scan;
use crate::application::update::update_manage::UpdateManager;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};

pub enum Error {
    AddUpdateTaskFailed,
}

#[derive(Debug)]
pub enum UpdateTaskControlMessage {
    Start { app_server_id: u64 },
    Stop { app_server_id: u64 },
}

#[derive(Debug)]
pub enum TaskControlMessage {
    Start,
    Stop,
}

#[derive(Default, Debug)]
pub enum UpdateTaskStatus {
    #[default]
    Wait,
    Processing,
    Canceled,
    Failed,
    Finished,
}

#[derive(Debug)]
pub struct UpdateTask {
    pub id: u64,
    pub app_server_id: u64,
    pub status: UpdateTaskStatus,
    pub tx: Sender<TaskControlMessage>,
    rx: Receiver<TaskControlMessage>,
}

impl UpdateTask {
    pub fn new(id: u64, app_server_id: u64) -> Self {
        let (tx, rx) = mpsc::channel::<TaskControlMessage>();
        Self {
            id,
            app_server_id,
            status: Default::default(),
            tx,
            rx,
        }
    }
}

pub fn start_update(
    app_server_id: u64,
    app_id: u64,
    app_manager: Arc<Mutex<AppManager>>,
    update_manager: Arc<Mutex<UpdateManager>>,
) {
    // let app_manager_g = app_manager.lock().unwrap();
    // // let app_server = app_manager_g.apps.get(&app_id);
    // drop(app_manager_g);

    let mut update_manager_g = update_manager.lock().unwrap();
    update_manager_g.start_task(app_server_id);
    drop(update_manager_g);
}

pub fn stop_update(
    app_server_id: u64,
    app_id: u64,
    app_manager: Arc<Mutex<AppManager>>,
    update_manager: Arc<Mutex<UpdateManager>>,
) {
    let mut update_manager_g = update_manager.lock().unwrap();
    update_manager_g.stop_task(app_server_id);
    drop(update_manager_g);
}
