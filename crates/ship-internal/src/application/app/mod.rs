use crate::application::app::app_server::AppServerInfo;
use std::collections::HashMap;

mod app_server;

#[derive(Default)]
pub struct AppManager {
    pub selected_app_id: Option<String>,
    pub apps: Apps,
}

impl AppManager {
    pub fn new(apps: Apps) -> Self {
        Self {
            selected_app_id: None,
            apps,
        }
    }
}

pub type Apps = HashMap<String, App>;

#[derive(Debug)]
pub struct App {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub priority: i64,
    pub app_server_info: AppServerInfo,
    pub selected_app_server_id: Option<String>,
}
