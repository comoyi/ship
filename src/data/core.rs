use std::sync::{Arc, Mutex};

pub type AppDataPtr = Arc<Mutex<AppData>>;

pub struct AppData {
    pub base_dir: String,
}

impl AppData {
    pub fn new() -> Self {
        AppData {
            base_dir: "".to_string(),
        }
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            base_dir: "".to_string(),
        }
    }
}
