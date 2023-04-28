pub mod app;
mod scan;
pub mod settings;
pub mod update;

use crate::application::app::{app_manage, AppManager};
use crate::application::settings::SettingsManager;
use crate::application::update::update_manage;
use crate::application::update::update_manage::UpdateManager;
use crate::config::CONFIG;
use crate::log::init_log;
use crate::request;
use internationalization::DICTIONARY;
use log::{debug, warn};
use std::path::Path;
use std::sync::{Arc, Mutex};
use util::filepath;

pub const APP_NAME: &str = "Launcher";

#[derive(Default)]
pub struct App {
    settings_manager: Arc<Mutex<SettingsManager>>,
    app_manager: Arc<Mutex<AppManager>>,
    update_manager: Arc<Mutex<UpdateManager>>,
}

impl App {
    pub fn new(
        settings_manager: Arc<Mutex<SettingsManager>>,
        app_manager: Arc<Mutex<AppManager>>,
        update_manager: Arc<Mutex<UpdateManager>>,
    ) -> Self {
        Self {
            settings_manager,
            app_manager,
            update_manager,
        }
    }

    pub fn run(&self) {
        let _ = &CONFIG.log_level;

        init_log(&CONFIG.log_level);
        debug!("log inited");

        CONFIG.print_config();

        DICTIONARY
            .switch_language_by_code(&CONFIG.language)
            .unwrap_or_else(|e| {
                warn!("switch language failed, err: {}", e);
            });

        let mut settings_manager = self.settings_manager.lock().unwrap();
        let program_dir_path_r = filepath::get_exe_dir();
        match program_dir_path_r {
            Ok(program_dir_path) => {
                settings_manager.settings.general_settings.program_dir_path =
                    program_dir_path.clone();
                let p = Path::new(&program_dir_path).join("data");
                settings_manager.settings.general_settings.data_dir_path =
                    p.to_str().unwrap().to_string();
            }
            Err(e) => {
                // TODO
                panic!("err: {}", e);
            }
        }

        drop(settings_manager);

        update_manage::start(Arc::clone(&self.update_manager));

        app_manage::start(Arc::clone(&self.app_manager));
    }
}
