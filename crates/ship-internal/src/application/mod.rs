pub mod app;
pub mod common;
pub mod scan;
pub mod settings;
pub mod update;

use crate::application::app::{app_manage, AppManager};
use crate::application::settings::SettingsManager;
use crate::application::update::update_manage;
use crate::application::update::update_manage::UpdateManager;
use crate::config::CONFIG;
use crate::log::init_log;
use crate::version::version_manage;
use crate::version::version_manage::VersionManager;
use internationalization::DICTIONARY;
use log::{debug, warn};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{env, fs};
use util::filepath;

pub const APP_NAME: &str = "Ship";

#[derive(Default)]
pub struct App {
    version_manager: Arc<Mutex<VersionManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
    app_manager: Arc<Mutex<AppManager>>,
    update_manager: Arc<Mutex<UpdateManager>>,
}

impl App {
    pub fn new(
        version_manager: Arc<Mutex<VersionManager>>,
        settings_manager: Arc<Mutex<SettingsManager>>,
        app_manager: Arc<Mutex<AppManager>>,
        update_manager: Arc<Mutex<UpdateManager>>,
    ) -> Self {
        Self {
            version_manager,
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
                // TODO remove unwrap
                fs::create_dir_all(&p).unwrap();
                settings_manager.settings.general_settings.data_dir_path =
                    p.to_str().unwrap().to_string();
            }
            Err(e) => {
                // TODO
                panic!("err: {}", e);
            }
        }

        drop(settings_manager);

        let mut version_manager_g = self.version_manager.lock().unwrap();
        // TODO remove unwrap
        let exe_path = env::current_exe().unwrap().to_str().unwrap().to_string();
        version_manager_g.exe_path = exe_path;
        drop(version_manager_g);
        version_manage::start(Arc::clone(&self.version_manager));

        update_manage::start(
            Arc::clone(&self.update_manager),
            Arc::clone(&self.app_manager),
            Arc::clone(&self.settings_manager),
        );

        app_manage::start(Arc::clone(&self.app_manager));
    }
}
