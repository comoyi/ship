use crate::request;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize_repr, Clone, Debug)]
#[repr(i8)]
pub enum ScanStatus {
    Wait = 10,
    Scanning = 20,
    Failed = 30,
    Completed = 40,
}

#[derive(PartialEq, Deserialize_repr, Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct FileInfo {
    pub relative_path: String,
    pub file_type: FileType,
    pub size: u64,
    pub hash: String,
}

impl FileInfo {
    pub fn new(relative_path: &str, file_type: FileType, size: u64, hash: &str) -> Self {
        Self {
            relative_path: relative_path.to_string(),
            file_type,
            size,
            hash: hash.to_string(),
        }
    }
}

impl From<&request::app_server::get_file_info::FileInfo> for FileInfo {
    fn from(value: &request::app_server::get_file_info::FileInfo) -> Self {
        Self {
            relative_path: value.relative_path.clone(),
            file_type: value.file_type.clone(),
            size: value.size,
            hash: value.hash.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ServerFileInfo {
    pub scan_status: ScanStatus,
    pub last_scan_finish_time: i64,
    pub files: Vec<FileInfo>,
}

impl From<&request::app_server::get_file_info::ServerFileInfoVo> for ServerFileInfo {
    fn from(value: &request::app_server::get_file_info::ServerFileInfoVo) -> Self {
        Self {
            scan_status: value.scan_status.clone(),
            last_scan_finish_time: value.last_scan_finish_time,
            files: value.files.iter().map(|x| FileInfo::from(x)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct ClientFileInfo {
    pub scan_status: ScanStatus,
    pub last_scan_finish_time: i64,
    pub files: Vec<FileInfo>,
}

impl ClientFileInfo {
    pub fn new(scan_status: ScanStatus, last_scan_finish_time: i64, files: Vec<FileInfo>) -> Self {
        Self {
            scan_status,
            last_scan_finish_time,
            files,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DataNode {
    pub name: String,
    pub address: Address,
}

impl From<&request::app_server::get_app_server::DataNode> for DataNode {
    fn from(value: &request::app_server::get_app_server::DataNode) -> Self {
        Self {
            name: value.name.clone(),
            address: value.address.clone(),
        }
    }
}
