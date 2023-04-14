mod app_manage;
pub mod launch;

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
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

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
    let program_dir_path_r = filepath::get_exe_dir();
    match program_dir_path_r {
        Ok(program_dir_path) => {
            app_data.settings.program_dir_path = program_dir_path.clone();
            let p = Path::new(&program_dir_path).join("data");
            app_data.settings.data_dir_path = p.to_str().unwrap().to_string();
        }
        Err(e) => {
            panic!("err: {}", e);
        }
    }
    let app_data_ptr = Arc::new(Mutex::new(app_data));

    let app_data_ptr_1 = Arc::clone(&app_data_ptr);
    thread::spawn(move || {
        app_manage::start(app_data_ptr_1);
    });

    let app_data_ptr_2 = Arc::clone(&app_data_ptr);
    let gui_flags = GuiFlags::new(app_data_ptr_2);
    gui::start(gui_flags);
}
