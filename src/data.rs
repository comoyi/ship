use crate::info::InfoManager;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct GuiFlags {
    pub data: Arc<Mutex<AppData>>,
    pub info_manager: Arc<Mutex<InfoManager>>,
}

impl GuiFlags {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Default)]
pub struct AppData {
    pub update_progress: UpdateProgress,
    pub dir: String,
    pub servers: Vec<Server>,
    pub infos: Vec<String>,
}

impl AppData {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize)]
pub struct UpdateProgress {
    pub value: f32,
    pub total: f32,
}

impl Default for UpdateProgress {
    fn default() -> Self {
        Self {
            value: 0.0,
            total: 100.0,
        }
    }
}

#[derive(Serialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub dir: String,
    pub file_info: Option<ServerFileInfo>,
    pub selected: bool,
    pub update_progress: UpdateProgress,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(i8)]
pub enum ScanStatus {
    Wait = 10,
    Scanning = 20,
    Failed = 30,
    Completed = 40,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerFileInfo {
    #[serde(rename = "status")]
    pub scan_status: ScanStatus,
    pub last_scan_finish_time: i64,
    pub files: Vec<FileInfo>,
}

impl Default for ServerFileInfo {
    fn default() -> Self {
        Self {
            scan_status: ScanStatus::Wait,
            last_scan_finish_time: 0,
            files: vec![],
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(i8)]
pub enum FileType {
    Unknown = 0,
    File = 1,
    Dir = 2,
    Symlink = 4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub relative_path: String,
    #[serde(rename = "type")]
    pub file_type: FileType,
    pub size: u64,
    pub hash: String,
}

// impl FileInfo {
//     pub fn new() -> Self {
//         FileInfo {
//             relative_path: "".to_string(),
//             file_type: FileType::Unknown,
//             size: 0,
//             hash: "".to_string(),
//         }
//     }
// }

struct Cache {
    chunks: HashMap<String, CacheFile>,
}

struct CacheFile {
    pub hash: String,
}
