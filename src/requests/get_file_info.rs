use crate::error::Error;
use crate::requests::get_full_url;
use log::debug;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize, Debug)]
pub struct ServerFileInfo {
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
pub fn get_file_info() -> Result<ServerFileInfo, Error> {
    debug!("get_info");
    let url = get_full_url("/files");
    let resp = reqwest::blocking::get(url).map_err(|_| Error::QueryError)?;
    let body_r = resp.text();
    let body = match body_r {
        Ok(v) => v,
        Err(_) => {
            return Err(Error::ReadBodyError);
        }
    };
    debug!("body: {}", body);
    let data = serde_json::from_str::<ServerFileInfo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        return Error::DecodeError;
    })?;
    debug!("data: {:?}", data);
    Ok(data)
}
