use crate::request::error::Error;
use crate::request::{get, get_full_url_by_server_address};
use crate::types::common::{FileType, ScanStatus};
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerFileInfoVo {
    #[serde(rename = "status")]
    pub scan_status: ScanStatus,
    pub last_scan_finish_time: i64,
    pub files: Vec<FileInfo>,
}

#[derive(Deserialize, Debug)]
pub struct FileInfo {
    pub relative_path: String,
    #[serde(rename = "type")]
    pub file_type: FileType,
    pub size: u64,
    pub hash: String,
}

pub fn get_file_info(server_address: &str) -> Result<ServerFileInfoVo, Error> {
    debug!("get_file_info");
    let url = get_full_url_by_server_address("/api/v1/files", server_address);
    let resp = get(&url)?;
    let body = resp.text().map_err(|_| Error::ReadBodyError)?;
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<ServerFileInfoVo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        Error::DecodeError
    })?;
    debug!("data: {:?}", data);
    Ok(data)
}
