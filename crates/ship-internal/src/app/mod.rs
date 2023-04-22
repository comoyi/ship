use crate::config::CONFIG;
use crate::log::init_log;
use log::debug;

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
    }
}
