use crate::utils::hash::md5::md5_string;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GServerInfo {
    pub servers: HashMap<String, GServer>,
}

impl GServerInfo {
    pub fn test_data() -> Self {
        let mut servers = HashMap::new();
        let s1 = GServer::new(
            1,
            "Server-1",
            Address::new("http", "127.0.0.1", 57111),
            "Server-1 description",
        );
        servers.insert(s1.uid.to_string(), s1);
        let s2 = GServer::new(
            2,
            "Server-2",
            Address::new("http", "127.0.0.1", 57211),
            "Server-2 description",
        );
        servers.insert(s2.uid.to_string(), s2);
        Self::new(servers)
    }
}

impl GServerInfo {
    fn new(servers: HashMap<String, GServer>) -> Self {
        Self { servers: servers }
    }
}

#[derive(Debug, Clone)]
pub struct GServer {
    pub id: u64,
    pub uid: String,
    pub name: String,
    pub address: Address,
    pub description: String,
}

impl GServer {
    pub fn new(id: u64, name: &str, address: Address, description: &str) -> Self {
        Self {
            id,
            uid: format!("{}-{}", id, md5_string(&address.to_address_string())),
            name: name.to_string(),
            address,
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Address {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

impl Address {
    pub fn new(protocol: &str, host: &str, port: u16) -> Self {
        Self {
            protocol: protocol.to_string(),
            host: host.to_string(),
            port,
        }
    }

    pub fn to_address_string(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

pub enum StartStatus {
    Wait,
    CheckUpdate,
    Starting,
    Started,
}
