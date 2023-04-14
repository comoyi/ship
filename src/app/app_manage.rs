use crate::data::apps::{App, AppManager, Apps};
use crate::data::common::{Address, AppServer, AppServerInfo};
use crate::data::core::AppDataPtr;
use crate::error::Error;
use crate::requests;
use log::debug;
use std::collections::HashMap;

pub fn start(app_data_ptr: AppDataPtr) {
    let apps_r = requests::get_apps::get_apps();
    let apps = match apps_r {
        Ok(v) => v,
        Err(_) => {
            debug!("get apps info failed");
            return;
        }
    };

    let mut a = Apps::new();
    for app_tmp in apps {
        let app_servers_r = requests::get_app_servers::get_app_servers(app_tmp.id);
        let app_servers = match app_servers_r {
            Ok(v) => v,
            Err(_) => {
                debug!("get app_servers info failed");
                return;
            }
        };
        let mut server_map: HashMap<String, AppServer> = Default::default();
        for app_server_tmp in app_servers {
            let addr = Address::new(
                &app_server_tmp.address.protocol,
                &app_server_tmp.address.host,
                app_server_tmp.address.port,
            );
            let app_server = AppServer::new(
                app_server_tmp.id,
                &app_server_tmp.name,
                addr,
                &app_server_tmp.description,
                app_server_tmp.priority,
            );
            server_map.insert(app_server.uid(), app_server);
        }
        let app_server_info = AppServerInfo::new(server_map);
        let app = App {
            id: app_tmp.id,
            name: app_tmp.name,
            priority: app_tmp.priority,
            app_server_info,
            selected_app_server_uid: None,
        };
        a.insert(Box::leak(app.uid().into_boxed_str()), app);
    }

    let app_manager = AppManager::new(a);

    let mut app_data_g = app_data_ptr.lock().unwrap();
    app_data_g.app_manager = app_manager;
    drop(app_data_g);
}
