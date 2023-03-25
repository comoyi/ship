use crate::config::CONFIG;
use crate::data::{AppData, GuiFlags, Server};
use crate::downloader::Downloader;
use crate::gui;
use crate::info::InfoManager;
use log::info;
use std::sync::{Arc, Mutex};
use std::thread;

pub const APP_NAME: &str = "Valheim Launcher";

pub fn start() {
    info!("start app");

    // init app_data
    let mut app_data = AppData::new();
    app_data.dir = CONFIG.dir.clone();

    let mut ss: Vec<Server> = vec![];
    for s in &CONFIG.servers {
        let server = Server {
            name: s.name.to_string(),
            protocol: s.protocol.to_string(),
            host: s.host.to_string(),
            port: s.port,
            dir: s.dir.to_string(),
            file_info: None,
        };
        ss.push(server);
    }
    app_data.servers = ss;

    // clone app_data
    let d = Arc::new(Mutex::new(app_data));
    let d1 = Arc::clone(&d);
    let d2 = Arc::clone(&d);

    // init info_manager
    let mut info_manager = InfoManager::new();
    info_manager.data = d1;
    let im = Arc::new(Mutex::new(info_manager));
    let im1 = Arc::clone(&im);
    let im2 = Arc::clone(&im);

    thread::spawn(|| {
        let downloader = Downloader::new();
        downloader.start(im1);
    });

    let mut gui_flags = GuiFlags::new();
    gui_flags.data = d2;
    gui_flags.info_manager = im2;
    gui::start(gui_flags);
}
