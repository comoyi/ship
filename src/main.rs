use crate::config::CONFIG;

mod app;
mod cache;
mod config;
mod data;
mod download;
mod error;
mod gui;
mod info;
mod log;
mod update;
mod util;
mod version;

fn main() {
    // for init config
    let _ = &CONFIG.log_level;
    // config::init_config();

    log::init_log();

    app::start();
}
