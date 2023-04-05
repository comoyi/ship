use crate::data::apps::{AppManager, AppUid};
use crate::data::common::{AppServerInfo, StartStatus};
use crate::data::page::{Pag, PageManager};
use crate::data::settings::Settings;
use std::sync::{Arc, Mutex};

pub type AppDataPtr = Arc<Mutex<AppData>>;

pub struct AppData {
    pub start_status: StartStatus,
    pub settings: Settings,
    pub page_manager: PageManager,
    pub app_manager: AppManager,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            start_status: StartStatus::Wait,
            settings: Settings {
                data_dir: "".to_string(),
                language: "".to_string(),
            },
            page_manager: PageManager {
                current_page: Pag::Apps,
                pages: Default::default(),
            },
            app_manager: AppManager {
                selected_app_uid: None,
                apps: Default::default(),
            },
        }
    }
}
