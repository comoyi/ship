use crate::config::CONFIG;
use crate::gui;
use crate::gui::GuiFlags;
use crate::log::init_log;
use std::sync::{Arc, Mutex};

pub const APP_NAME: &str = "Valheim Launcher";

pub fn start() {
    let _ = &CONFIG.log_level;

    init_log();

    let app_data = AppData::new();
    let app_data_ptr = Arc::new(Mutex::new(app_data));
    let gui_flags = GuiFlags::new(&app_data_ptr);
    gui::start(gui_flags);
}

pub type AppDataPtr = Arc<Mutex<AppData>>;
pub struct AppData {}

impl AppData {
    pub fn new() -> Self {
        AppData {}
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self {}
    }
}
