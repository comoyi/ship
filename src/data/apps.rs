use crate::data::common::{AppServer, AppServerInfo};
use std::collections::HashMap;

pub type AppUid = String;

pub struct AppManager {
    pub selected_app_uid: Option<AppUid>,
    pub apps: Apps,
}

impl AppManager {
    pub fn test_data() -> AppManager {
        let mut apps = Apps::new();
        let app_1 = App {
            uid: "Project-A".to_string(),
            name: "App-A".to_string(),
            app_server_info: AppServerInfo::test_data(),
            selected_app_server_uid: None,
        };
        apps.insert(Box::leak(app_1.uid.clone().into_boxed_str()), app_1);
        let app_2 = App {
            uid: "Project-B".to_string(),
            name: "App-B".to_string(),
            app_server_info: AppServerInfo {
                servers: Default::default(),
            },
            selected_app_server_uid: None,
        };
        apps.insert(Box::leak(app_2.uid.clone().into_boxed_str()), app_2);
        AppManager {
            selected_app_uid: None,
            apps: apps,
        }
    }
}

pub type Apps = HashMap<&'static str, App>;

#[derive(Debug, Clone)]
pub struct App {
    pub uid: AppUid,
    pub name: String,
    pub app_server_info: AppServerInfo,
    pub selected_app_server_uid: Option<String>,
}
