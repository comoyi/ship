use crate::application::app::app_server::AppServerInfo;
use crate::types::common::Launch;
use std::collections::HashMap;

pub mod app_manage;
pub mod app_server;

#[derive(Default)]
pub struct AppManager {
    pub selected_app_id: Option<u64>,
    pub apps: Apps,
}

impl AppManager {
    pub fn new(apps: Apps) -> Self {
        Self {
            selected_app_id: None,
            apps,
        }
    }

    pub fn select_app(&mut self, appid: u64) {
        self.selected_app_id = Some(appid);
    }

    pub fn select_app_server(&mut self, app_server_id: u64, app_id: u64) {
        if let Some(a) = self.apps.get_mut(&app_id) {
            a.selected_app_server_id = Some(app_server_id);
        }
    }
}

pub type Apps = HashMap<u64, App>;

#[derive(Debug)]
pub struct App {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub dir_name: String,
    pub priority: i64,
    pub launch: Launch,
    pub app_server_info: AppServerInfo,
    pub selected_app_server_id: Option<u64>,
}

impl App {
    pub fn uid(&self) -> String {
        format!("{}", self.id)
    }
}
