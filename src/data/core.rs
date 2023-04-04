use crate::data::common::{GServerInfo, StartStatus};
use crate::data::settings::Settings;
use std::sync::{Arc, Mutex};

pub type AppDataPtr = Arc<Mutex<AppData>>;

pub struct AppData {
    pub g_server_info: GServerInfo,
    pub selected_g_server_uid: Option<String>,
    pub start_status: StartStatus,
    pub settings: Settings,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            g_server_info: GServerInfo::test_data(),
            selected_g_server_uid: None,
            start_status: StartStatus::Wait,
            settings: Settings {
                data_dir: "".to_string(),
            },
        }
    }
}
