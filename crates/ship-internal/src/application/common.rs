use crate::application::app::AppManager;
use crate::application::settings::SettingsManager;
use std::sync::{Arc, Mutex};

pub fn get_data_path_by_app_server_id<'a>(
    app_server_id: u64,
    app_manager: Arc<Mutex<AppManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
) -> Result<String, &'a str> {
    let settings_manager_g = settings_manager.lock().unwrap();
    let base_path = settings_manager_g
        .settings
        .general_settings
        .data_dir_path
        .clone();
    drop(settings_manager_g);
    let (app_id, app_dir_name) = get_app_data(app_server_id, app_manager)?;
    Ok(format!(
        "{}/{}/{}/{}/{}",
        base_path, app_dir_name, app_id, app_server_id, app_dir_name
    ))
}

fn get_app_data<'a>(
    app_server_id: u64,
    app_manager: Arc<Mutex<AppManager>>,
) -> Result<(u64, String), &'a str> {
    let app_manager_g = app_manager.lock().unwrap();
    for (_, app_tmp) in &app_manager_g.apps {
        for (_, app_server_tmp) in &app_tmp.app_server_info.servers {
            if app_server_tmp.id == app_server_id {
                return Ok((app_tmp.id, app_tmp.code.clone()));
            }
        }
    }
    drop(app_manager_g);
    Err("get app_id failed")
}
