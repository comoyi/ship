mod settings;

use crate::app::settings::Settings;
use crate::config::CONFIG;
use crate::log::init_log;
use crate::request;
use internationalization::DICTIONARY;
use log::{debug, warn};

pub const APP_NAME: &str = "Launcher";

#[derive(Default)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self::default()
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
    }
}

struct AppData {
    settings: Settings,
}
