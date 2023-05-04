use crate::request::error::Error;
use crate::request::{get, get_full_url};
use crate::types::common::Launch;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppsVo {
    pub apps: Vec<App>,
}

#[derive(Deserialize, Debug)]
pub struct App {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub dir_name: String,
    pub priority: i64,
    pub launch: Launch,
}

pub fn get_apps() -> Result<AppsVo, Error> {
    debug!("get_apps");
    let url = get_full_url("/api/v1/apps");
    let resp = get(&url)?;
    let body = resp.text().map_err(|_| Error::ReadBodyError)?;
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<AppsVo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        Error::DecodeError
    })?;
    debug!("data: {:?}", data);
    Ok(data)
}
