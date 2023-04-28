use crate::application::update::{Error, UpdateTask};
use log::debug;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct UpdateManager {
    next_update_task_id: AtomicU64,
    update_tasks: HashMap<u64, UpdateTask>,
    operate_task_mutex: Mutex<()>,
}

impl UpdateManager {
    pub fn create_task(&self, app_server_id: u64) -> UpdateTask {
        let next_update_task_id = self.next_update_task_id.fetch_add(1, Ordering::Relaxed);
        let update_task = UpdateTask::new(next_update_task_id, app_server_id);
        update_task
    }

    pub fn add_task(&mut self, update_task: UpdateTask) -> Result<(), Error> {
        let mu = self.operate_task_mutex.lock().unwrap();

        // TODO check duplicate

        self.update_tasks.insert(update_task.id, update_task);
        drop(mu);
        Ok(())
    }

    // TODO
    pub fn cancel_task(&self, id: u64) {}
}

impl Default for UpdateManager {
    fn default() -> Self {
        Self {
            next_update_task_id: AtomicU64::new(1),
            update_tasks: Default::default(),
            operate_task_mutex: Mutex::new(()),
        }
    }
}

pub fn start(update_manager: Arc<Mutex<UpdateManager>>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1000));
        let mut update_manager_g = update_manager.lock().unwrap();
        // TODO
        // debug!("{:?}", update_manager_g.update_tasks.len());
        drop(update_manager_g);
    });
}
