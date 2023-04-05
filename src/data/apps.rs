use crate::data::common::{AppServer, AppServerInfo};
use std::collections::HashMap;

pub enum AppUid {
    ProjectA,
    ProjectB,
}

pub struct AppManager {
    pub selected_app_uid: Option<AppUid>,
    pub apps: Apps,
}

impl AppManager {
    pub fn test_data() -> AppManager {
        let mut apps = Apps::new();
        let app_1 = App {
            uid: AppUid::ProjectA,
            name: "App-A".to_string(),
            app_server_info: AppServerInfo::test_data(),
            selected_app_server_uid: None,
        };
        apps.insert("AAA", app_1);
        let app_2 = App {
            uid: AppUid::ProjectB,
            name: "App-B".to_string(),
            app_server_info: AppServerInfo {
                servers: Default::default(),
            },
            selected_app_server_uid: None,
        };
        apps.insert("BBB", app_2);
        AppManager {
            selected_app_uid: None,
            apps: apps,
        }
    }
}

pub type Apps = HashMap<&'static str, App>;

pub struct App {
    pub uid: AppUid,
    pub name: String,
    pub app_server_info: AppServerInfo,
    pub selected_app_server_uid: Option<String>,
}
