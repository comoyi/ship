use crate::application::app::AppManager;
use crate::application::settings::SettingsManager;
use crate::application::update::update::handle_update_control;
use crate::application::update::{Error, UpdateTask, UpdateTaskControlMessage};
use log::{debug, warn};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, RecvError, Sender, TryRecvError};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct UpdateManager {
    next_update_task_id: AtomicU64,
    pub update_tasks: HashMap<u64, UpdateTask>,
    operate_task_mutex: Mutex<()>,
    tx: Sender<UpdateTaskControlMessage>,
    rx: Receiver<UpdateTaskControlMessage>,
}

impl UpdateManager {
    pub fn create_task(&self, app_server_id: u64) -> UpdateTask {
        let next_update_task_id = self.next_update_task_id.fetch_add(1, Ordering::Relaxed);
        let update_task = UpdateTask::new(next_update_task_id, app_server_id);
        update_task
    }

    pub fn get_update_task_by_id(&self, id: u64) -> Option<&UpdateTask> {
        self.update_tasks.get(&id)
    }

    pub fn add_task(&mut self, update_task: UpdateTask) -> Result<(), Error> {
        let mu = self.operate_task_mutex.lock().unwrap();
        // TODO check duplicate by app_server_id
        self.update_tasks.insert(update_task.id, update_task);
        drop(mu);
        Ok(())
    }

    pub fn remove_task(&mut self, id: u64) -> Result<(), Error> {
        let mu = self.operate_task_mutex.lock().unwrap();
        self.update_tasks.remove(&id);
        drop(mu);
        Ok(())
    }

    pub fn start_task(&mut self, app_server_id: u64) -> Result<(), Error> {
        let update_task_control_message = UpdateTaskControlMessage::Start { app_server_id };
        let r = self.tx.send(update_task_control_message);
        if let Err(e) = r {
            warn!("add UpdateTaskControlMessage failed, err: {}", e);
        }
        Ok(())
    }

    pub fn stop_task(&self, app_server_id: u64) -> Result<(), Error> {
        let update_task_control_message = UpdateTaskControlMessage::Stop { app_server_id };
        let r = self.tx.send(update_task_control_message);
        if let Err(e) = r {
            warn!("add UpdateTaskControlMessage failed, err: {}", e);
        }

        Ok(())
    }
}

impl Default for UpdateManager {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel::<UpdateTaskControlMessage>();
        Self {
            next_update_task_id: AtomicU64::new(1),
            update_tasks: Default::default(),
            operate_task_mutex: Mutex::new(()),
            tx,
            rx,
        }
    }
}

pub fn start(
    update_manager: Arc<Mutex<UpdateManager>>,
    app_manager: Arc<Mutex<AppManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        let update_manager_2 = Arc::clone(&update_manager);
        let mut update_manager_g = update_manager.lock().unwrap();
        let task_r = update_manager_g.rx.try_recv();
        drop(update_manager_g);

        match task_r {
            Ok(message) => {
                let app_manager = Arc::clone(&app_manager);
                let settings_manager = Arc::clone(&settings_manager);
                thread::spawn(move || {
                    handle_update_control(message, update_manager_2, app_manager, settings_manager);
                });
            }
            Err(e) => match e {
                TryRecvError::Empty => {}
                e => {
                    debug!("get UpdateTaskControlMessage failed, err: {}", e)
                }
            },
        }
    });
}
