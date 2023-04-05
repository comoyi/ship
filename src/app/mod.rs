use crate::config::CONFIG;
use crate::data::apps::AppManager;
use crate::data::common::AppServerInfo;
use crate::data::core::AppData;
use crate::gui;
use crate::gui::GuiFlags;
use crate::i18n::DICTIONARY;
use crate::log::init_log;
use crate::utils::filepath;
use log::warn;
use std::sync::{Arc, Mutex};

pub const APP_NAME: &str = "Launcher";

pub fn start() {
    let _ = &CONFIG.log_level;

    init_log();

    CONFIG.print_config();

    let switch_language_r = DICTIONARY.switch_language_by_code(&CONFIG.language);
    if let Err(e) = switch_language_r {
        warn!("switch language failed, err: {}", e);
    }

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
    let app_manager = AppManager::test_data();
    app_data.app_manager = app_manager;
    let gsi = AppServerInfo::test_data();
    app_data.app_server_info = gsi;
    let app_data_ptr = Arc::new(Mutex::new(app_data));
    let gui_flags = GuiFlags::new(&app_data_ptr);
    gui::start(gui_flags);
}
