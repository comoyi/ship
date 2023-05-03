mod sync;
mod update;
pub mod update_manage;

use crate::application::app::AppManager;
use crate::application::scan;
use crate::application::update::sync::SyncTask;
use crate::application::update::update_manage::UpdateManager;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};

#[derive(Debug)]
pub enum Error {
    SendControlMessageFailed,
    SendTraceMessageFailed,

    TaskNotExist,
    GetDataPathFailed,
    CreateDirFailed,
    GetAppServerFailed,
    GetServerFileInfoFailed,
    GetClientFileInfoFailed,
    GetDataNodesFailed,
    AddSyncTaskFailed,
    HandleSyncTaskFailed,
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
    Processing {
        progress: Progress,
        sync_task: SyncTask,
    },
    Canceled,
    Failed,
    Finished,
}

#[derive(Default, Debug)]
pub struct Progress {
    value: u64,
    total: u64,
}

impl Progress {
    pub fn new(value: u64, total: u64) -> Self {
        Self { value, total }
    }
}

#[derive(Default, Debug)]
pub enum UpdateTaskTraceMessage {
    #[default]
    Wait,
    Processing {
        progress: Progress,
        sync_task: SyncTask,
    },
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
    trace_tx: Sender<UpdateTaskTraceMessage>,
    trace_rx: Receiver<UpdateTaskTraceMessage>,
}

impl UpdateTask {
    pub fn new(id: u64, app_server_id: u64) -> Self {
        let (tx, rx) = mpsc::channel::<TaskControlMessage>();
        let (trace_tx, trace_rx) = mpsc::channel::<UpdateTaskTraceMessage>();
        Self {
            id,
            app_server_id,
            status: Default::default(),
            tx,
            rx,
            trace_tx,
            trace_rx,
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
