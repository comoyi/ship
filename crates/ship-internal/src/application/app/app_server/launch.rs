use crate::application::app::AppManager;
use crate::application::common::get_data_path_by_app_server_id;
use crate::application::settings::SettingsManager;
use log::{debug, info, warn};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn launch(
    app_server_id: u64,
    app_id: u64,
    app_manager: Arc<Mutex<AppManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
) {
    thread::spawn(move || {
        do_launch(app_server_id, app_id, app_manager, settings_manager);
    });
}

pub fn do_launch(
    app_server_id: u64,
    app_id: u64,
    app_manager: Arc<Mutex<AppManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
) {
    debug!(
        "launch, app_id: {}, app_server_id: {}",
        app_id, app_server_id
    );
    let app_manager_g = app_manager.lock().unwrap();
    let app_o = app_manager_g.apps.get(&app_id);
    let app = match app_o {
        None => {
            warn!("app not found, app_id: {}", app_id);
            return;
        }
        Some(x) => x,
    };

    debug!("app: {:?}", app);
    let launch = app.launch.clone();
    drop(app_manager_g);
    debug!("launch: {:?}", launch);

    let data_path_r = get_data_path_by_app_server_id(
        app_server_id,
        Arc::clone(&app_manager),
        Arc::clone(&settings_manager),
    );
    let data_path = match data_path_r {
        Ok(p) => p,
        Err(_) => {
            warn!("get_data_path_by_app_server_id failed");
            return;
        }
    };
    debug!("app_server_id: {}, data_path: {}", app_server_id, data_path);

    for x in launch.steps {
        if x.command.is_empty() {
            continue;
        }
        if x.pre_time > 0 {
            debug!("pre wait {} ms", x.pre_time);
            thread::sleep(Duration::from_millis(x.pre_time));
        }
        let p = Path::new(&data_path).join(x.command);
        debug!("program path: {:?}", p);
        let s = Command::new(p).spawn();
        if let Err(e) = s {
            debug!("exec failed, err: {}", e);
            return;
        }
    }
    info!("all exec steps finished");
}
