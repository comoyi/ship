use crate::request::error::Error;
use crate::request::{get, get_full_url};
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppServersVo {
    servers: Vec<AppServer>,
}

#[derive(Deserialize, Debug)]
pub struct AppServer {
    pub id: u64,
    pub name: String,
    pub address: Address,
    pub description: String,
    pub priority: i64,
}

#[derive(Deserialize, Debug)]
pub struct Address {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

pub fn get_app_servers(app_id: u64) -> Result<AppServersVo, Error> {
    debug!("get_app_servers");
    let url = get_full_url(&format!("{}?app_id={}", "/api/v1/app_servers", app_id));
    let resp = get(&url)?;
    let body = resp.text().map_err(|_| Error::ReadBodyError)?;
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<AppServersVo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        Error::DecodeError
    })?;
    debug!("data: {:?}", data);
    Ok(data)
}
