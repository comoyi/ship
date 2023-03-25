use crate::data::AppData;
use crate::gui;
use log::info;
use std::sync::{Arc, Mutex};

pub const APP_NAME: &str = "Valheim Launcher";

pub fn start() {
    info!("start app");

    let app_data = AppData::new();
    let d = Arc::new(Mutex::new(app_data));
    let d1 = Arc::clone(&d);

    gui::start(d1);
}
