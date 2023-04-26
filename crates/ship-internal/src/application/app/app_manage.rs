use crate::application::app::app_server::{AppServer, AppServerInfo, AppServers};
use crate::application::app::{App, AppManager, Apps};
use crate::request;
use std::sync::{Arc, Mutex};

pub enum Error {
    GetAppsFailed,
    GetAppServersFailed,
}

pub fn start(app_manager: Arc<Mutex<AppManager>>) -> Result<(), Error> {
    let apps_vo = request::get_apps::get_apps().map_err(|_| Error::GetAppsFailed)?;
    let mut apps = Apps::new();

    for app_tmp in &apps_vo.apps {
        let app_servers_vo = request::get_app_servers::get_app_servers(app_tmp.id)
            .map_err(|_| Error::GetAppServersFailed)?;
        let mut app_servers = AppServers::new();

        let mut selected_app_server_id = None;
        let mut sort_vec: Vec<_> = app_servers_vo.servers.iter().collect();
        sort_vec.sort_by(|a, b| b.priority.cmp(&a.priority));
        if let Some(app) = sort_vec.first() {
            selected_app_server_id = Some(app.id);
        }
        for app_server_tmp in app_servers_vo.servers {
            let app_server = AppServer::new(
                app_server_tmp.id,
                app_server_tmp.app_id,
                &app_server_tmp.name,
                app_server_tmp.address,
                &app_server_tmp.description,
                app_server_tmp.priority,
            );
            app_servers.insert(app_server.id, app_server);
        }
        let app = App {
            id: app_tmp.id,
            name: app_tmp.name.clone(),
            code: app_tmp.code.clone(),
            priority: app_tmp.priority,
            app_server_info: AppServerInfo::new(app_servers),
            selected_app_server_id,
        };
        apps.insert(app.id, app);
    }

    let mut sort_vec: Vec<_> = apps_vo.apps.iter().collect();
    sort_vec.sort_by(|a, b| b.priority.cmp(&a.priority));

    let mut app_manager_g = app_manager.lock().unwrap();
    app_manager_g.apps = apps;
    if let Some(app) = sort_vec.first() {
        app_manager_g.selected_app_id = Some(app.id);
    }
    drop(app_manager_g);

    Ok(())
}
