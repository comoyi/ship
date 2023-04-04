use crate::config::CONFIG;
use crate::data::common::GServerInfo;
use crate::data::core::AppData;
use crate::gui;
use crate::gui::GuiFlags;
use crate::log::init_log;
use crate::utils::filepath;
use log::{debug, warn};
use std::env::current_exe;
use std::sync::{Arc, Mutex};

pub const APP_NAME: &str = "Valheim Launcher";

pub fn start() {
    let _ = &CONFIG.log_level;

    init_log();

    CONFIG.print_config();

    let mut app_data = AppData::default();
    let data_dir_r = filepath::get_exe_dir();
    match data_dir_r {
        Ok(data_dir) => {
            app_data.settings.data_dir = data_dir;
        }
        Err(e) => {
            panic!("err: {}", e);
        }
    }
    let gsi = GServerInfo::test_data();
    app_data.g_server_info = gsi;
    let app_data_ptr = Arc::new(Mutex::new(app_data));
    let gui_flags = GuiFlags::new(&app_data_ptr);
    gui::start(gui_flags);
}
