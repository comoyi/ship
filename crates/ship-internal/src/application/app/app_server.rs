use crate::types::common::Address;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AppServerInfo {
    pub servers: HashMap<String, AppServer>,
}

impl AppServerInfo {
    pub fn new(servers: HashMap<String, AppServer>) -> Self {
        Self { servers }
    }
}

#[derive(Debug)]
pub struct AppServer {
    pub id: u64,
    pub name: String,
    pub address: Address,
    pub description: String,
    pub priority: i64,
}

impl AppServer {
    pub fn new(id: u64, name: &str, address: Address, description: &str, priority: i64) -> Self {
        Self {
            id,
            name: name.to_string(),
            address,
            description: description.to_string(),
            priority,
        }
    }
}
