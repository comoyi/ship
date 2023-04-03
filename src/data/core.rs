use crate::data::common::GServerInfo;
use std::sync::{Arc, Mutex};

pub type AppDataPtr = Arc<Mutex<AppData>>;

pub struct AppData {
    pub base_dir: String,
    pub g_server_info: GServerInfo,
    pub selected_g_server_uid: Option<String>,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            base_dir: "".to_string(),
            g_server_info: GServerInfo::test_data(),
            selected_g_server_uid: None,
        }
    }
}
