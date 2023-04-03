#[derive(Debug)]
pub struct GServerInfo {
    pub servers: Vec<GServer>,
}

impl GServerInfo {
    pub fn test_data() -> Self {
        let mut servers = vec![];
        servers.push(GServer::new("Server-1", Address::new("", "", 8081)));
        servers.push(GServer::new("Server-2", Address::new("", "", 8082)));
        Self::new(servers)
    }
}

impl GServerInfo {
    fn new(servers: Vec<GServer>) -> Self {
        Self { servers: servers }
    }
}

#[derive(Debug)]
pub struct GServer {
    pub name: String,
    pub address: Address,
}

impl GServer {
    pub fn new(name: &str, address: Address) -> Self {
        Self {
            name: name.to_string(),
            address,
        }
    }
}

#[derive(Debug)]
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
}
