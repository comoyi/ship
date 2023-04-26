pub mod app;
mod settings;

use crate::application::app::{app_manage, AppManager};
use crate::application::settings::Settings;
use crate::config::CONFIG;
use crate::log::init_log;
use internationalization::DICTIONARY;
use log::{debug, warn};
use std::sync::{Arc, Mutex};

pub const APP_NAME: &str = "Launcher";

#[derive(Default)]
pub struct App {
    app_manager: Arc<Mutex<AppManager>>,
    settings: Settings,
}

impl App {
    pub fn new(app_manager: Arc<Mutex<AppManager>>) -> Self {
        Self {
            app_manager,
            settings: Default::default(),
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

        app_manage::start(Arc::clone(&self.app_manager));
    }
}
