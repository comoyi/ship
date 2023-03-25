use crate::config::CONFIG;

mod app;
mod config;
mod data;
mod downloader;
mod error;
mod gui;
mod info;
mod log;
mod update;
mod version;

fn main() {
    // for init config
    let _ = &CONFIG.log_level;
    // config::init_config();

    log::init_log();

    app::start();
}
