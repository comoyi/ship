use crate::application::app::app_server::{AppServer, AppServerInfo, AppServers};
use crate::application::app::{App, AppManager, Apps};
use crate::request;
use crate::request::app_server::announcement::AnnouncementVo;
use log::{debug, warn};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

    thread::spawn(move || {
        refresh_announcement(Arc::clone(&app_manager));
    });

    Ok(())
}

fn refresh_announcement(app_manager: Arc<Mutex<AppManager>>) {
    loop {
        let mut address = None;
        let mut app_id = 0;
        let mut app_server_id = 0;
        let mut app_manager_g = app_manager.lock().unwrap();
        if let Some(selected_app_id) = app_manager_g.selected_app_id {
            app_id = selected_app_id;
            if let Some(app) = app_manager_g.apps.get_mut(&selected_app_id) {
                if let Some(selected_app_server_id) = app.selected_app_server_id {
                    app_server_id = selected_app_server_id;
                    if let Some(app_server) =
                        app.app_server_info.servers.get_mut(&selected_app_server_id)
                    {
                        address = Some(app_server.address.to_address_string());
                    }
                }
            }
        }
        drop(app_manager_g);

        if let Some(address) = address {
            let announcement_r = request::app_server::announcement::get_announcement(&address);
            match announcement_r {
                Ok(announcement_vo) => {
                    set_announcement(
                        app_server_id,
                        app_id,
                        announcement_vo,
                        Arc::clone(&app_manager),
                    );
                }
                Err(_) => {
                    warn!("get announcement failed");
                }
            }
        }

        thread::sleep(Duration::from_secs(60));
    }
}

fn set_announcement(
    app_server_id: u64,
    app_id: u64,
    announcement_vo: AnnouncementVo,
    app_manager: Arc<Mutex<AppManager>>,
) {
    let mut app_manager_g = app_manager.lock().unwrap();

    if let Some(app) = app_manager_g.apps.get_mut(&app_id) {
        if let Some(app_server) = app.app_server_info.servers.get_mut(&app_server_id) {
            app_server.announcement.content = announcement_vo.content;
        }
    }

    drop(app_manager_g);
}
