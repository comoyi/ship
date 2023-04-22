use crate::log::init_log;
use log::debug;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&self) {
        init_log();

        debug!("log inited");
    }
}
