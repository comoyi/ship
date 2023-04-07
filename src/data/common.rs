use crate::t;
use crate::utils::hash::md5::md5_string;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AppServerInfo {
    pub servers: HashMap<String, AppServer>,
}

impl AppServerInfo {
    pub fn test_data() -> Self {
        let mut servers = HashMap::new();
        let s1 = AppServer::new(
            1,
            "Server-1",
            Address::new("http", "127.0.0.1", 57111),
            "Server-1 description",
            100,
        );
        servers.insert(s1.uid.to_string(), s1);
        let s2 = AppServer::new(
            2,
            "Server-2",
            Address::new("http", "127.0.0.1", 57211),
            "Server-2 description",
            50,
        );
        servers.insert(s2.uid.to_string(), s2);
        Self::new(servers)
    }
}

impl AppServerInfo {
    fn new(servers: HashMap<String, AppServer>) -> Self {
        Self { servers: servers }
    }
}

#[derive(Debug, Clone)]
pub struct AppServer {
    pub id: u64,
    pub uid: String,
    pub name: String,
    pub address: Address,
    pub description: String,
    pub priority: i64,
    pub start_status: StartStatus,
}

impl AppServer {
    pub fn new(id: u64, name: &str, address: Address, description: &str, priority: i64) -> Self {
        Self {
            id,
            uid: format!("{}-{}", id, md5_string(&address.to_address_string())),
            name: name.to_string(),
            address,
            description: description.to_string(),
            priority,
            start_status: StartStatus::Wait,
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

#[derive(Debug, Clone)]
pub enum StartStatus {
    Wait,
    StartHandle,
    CheckUpdate,
    Updating(Progress),
    UpdateCompleted,
    Starting,
    Started,
    Cancelled,
    Failed,
}

impl StartStatus {
    pub fn description(&self) -> &'static str {
        match self {
            StartStatus::Wait => "",
            StartStatus::StartHandle => t!("start_status_start_processing"),
            StartStatus::CheckUpdate => t!("start_status_check_update"),
            StartStatus::Updating(p) => Box::leak(
                format!("{} {}/{}", t!("start_status_updating"), p.v, p.total).into_boxed_str(),
            ),
            StartStatus::UpdateCompleted => {
                t!("start_status_update_completed")
            }
            StartStatus::Starting => t!("start_status_starting"),
            StartStatus::Started => t!("start_status_started"),
            StartStatus::Cancelled => {
                t!("start_status_cancelled")
            }
            StartStatus::Failed => {
                t!("start_status_failed")
            }
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(i8)]
pub enum ScanStatus {
    Wait = 10,
    Scanning = 20,
    Failed = 30,
    Completed = 40,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(i8)]
pub enum FileType {
    Unknown = 0,
    File = 1,
    Dir = 2,
    Symlink = 4,
}

impl FileType {
    pub fn to_formatted_string(&self) -> String {
        match self {
            FileType::Unknown => String::from("Unknown"),
            FileType::File => String::from("File   "),
            FileType::Dir => String::from("Dir    "),
            FileType::Symlink => String::from("Symlink"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub relative_path: String,
    #[serde(rename = "type")]
    pub file_type: FileType,
    pub size: u64,
    pub hash: String,
}

impl FileInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            relative_path: "".to_string(),
            file_type: FileType::Unknown,
            size: 0,
            hash: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientFileInfo {
    #[serde(rename = "status")]
    pub scan_status: ScanStatus,
    pub last_scan_finish_time: i64,
    pub files: Vec<FileInfo>,
}

impl Default for ClientFileInfo {
    fn default() -> Self {
        Self {
            scan_status: ScanStatus::Wait,
            last_scan_finish_time: 0,
            files: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Progress {
    pub v: usize,
    pub total: usize,
}
