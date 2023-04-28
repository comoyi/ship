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

#[derive(Deserialize_repr, Debug)]
#[repr(i8)]
pub enum ScanStatus {
    Wait = 10,
    Scanning = 20,
    Failed = 30,
    Completed = 40,
}

#[derive(Deserialize_repr, Debug)]
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

#[derive(Debug)]
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

pub struct ServerFileInfo {
    pub scan_status: ScanStatus,
    pub last_scan_finish_time: i64,
    pub files: Vec<FileInfo>,
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
