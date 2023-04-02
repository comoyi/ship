use crate::config::CONFIG;
use crate::gui;
use crate::gui::GuiFlags;
use crate::log::init_log;
use log::{debug, warn};
use std::env::current_exe;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub const APP_NAME: &str = "Valheim Launcher";

pub fn start() {
    let _ = &CONFIG.log_level;

    init_log();

    CONFIG.print_config();

    let mut app_data = AppData::new();
    let base_dir = get_base_dir();
    app_data.base_dir = base_dir;
    let app_data_ptr = Arc::new(Mutex::new(app_data));
    let gui_flags = GuiFlags::new(&app_data_ptr);
    gui::start(gui_flags);
}

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

fn get_base_dir() -> String {
    let exe_path_r = current_exe();
    let exe_path;
    match exe_path_r {
        Ok(p) => {
            exe_path = p;
        }
        Err(e) => {
            warn!("{}", e);
            panic!("get current_exe_path failed!");
        }
    }
    debug!("exe_path: {:?}", exe_path);
    let exe_dir_o = exe_path.parent();
    let exe_dir;
    match exe_dir_o {
        None => {
            panic!("get exe_dir failed!");
        }
        Some(p) => {
            exe_dir = p;
        }
    }
    debug!("exe_dir: {:?}", exe_dir);

    let exe_dir_o2 = exe_dir.to_str();
    let base_dir;
    match exe_dir_o2 {
        None => {
            panic!("convert exe_dir failed!");
        }
        Some(p) => {
            base_dir = p.to_string();
        }
    }
    base_dir
}
