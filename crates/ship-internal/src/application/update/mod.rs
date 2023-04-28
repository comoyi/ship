mod sync;
pub mod update_manage;

use crate::application::app::AppManager;
use crate::application::scan;
use crate::application::update::update_manage::UpdateManager;
use std::sync::{Arc, Mutex};

pub enum Error {
    AddUpdateTaskFailed,
}

#[derive(Default)]
pub enum UpdateTaskStatus {
    #[default]
    Wait,
    Processing,
    Canceled,
    Failed,
    Finished,
}

#[derive(Default)]
pub struct UpdateTask {
    pub id: u64,
    pub app_server_id: u64,
    pub status: UpdateTaskStatus,
}

impl UpdateTask {
    pub fn new(id: u64, app_server_id: u64) -> Self {
        Self {
            id,
            app_server_id,
            status: Default::default(),
        }
    }
}

pub fn update(
    app_server_id: u64,
    app_id: u64,
    app_manager: Arc<Mutex<AppManager>>,
    update_manager: Arc<Mutex<UpdateManager>>,
) {
    // let app_manager_g = app_manager.lock().unwrap();
    // // let app_server = app_manager_g.apps.get(&app_id);
    // drop(app_manager_g);

    let mut update_manager_g = update_manager.lock().unwrap();
    let update_task = update_manager_g.create_task(app_server_id);
    update_manager_g.add_task(update_task);
    drop(update_manager_g);

    scan::scan("/tmp/test");
}
