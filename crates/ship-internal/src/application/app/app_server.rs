pub mod launch;

use crate::types::banner::Banner;
use crate::types::common::Address;
use crate::types::launch::LaunchStatus;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AppServerInfo {
    pub servers: AppServers,
}

impl AppServerInfo {
    pub fn new(servers: AppServers) -> Self {
        Self { servers }
    }
}

pub type AppServers = HashMap<u64, AppServer>;

#[derive(Debug)]
pub struct AppServer {
    pub id: u64,
    pub app_id: u64,
    pub name: String,
    pub address: Address,
    pub description: String,
    pub announcement: Announcement,
    pub banners: Vec<Banner>,
    pub priority: i64,
    pub launch_status: LaunchStatus,
}

impl AppServer {
    pub fn new(
        id: u64,
        app_id: u64,
        name: &str,
        address: Address,
        description: &str,
        priority: i64,
    ) -> Self {
        Self {
            id,
            app_id,
            name: name.to_string(),
            address,
            description: description.to_string(),
            announcement: Default::default(),
            banners: Default::default(),
            priority,
            launch_status: Default::default(),
        }
    }

    pub fn uid(&self) -> String {
        format!("{}", self.id)
    }
}

#[derive(Default, Debug)]
pub struct Announcement {
    pub content: String,
}
