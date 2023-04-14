use crate::error::Error;
use crate::requests::get_full_url;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AppServerInfo {
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

pub fn get_app_servers(app_id: u64) -> Result<Vec<AppServer>, Error> {
    debug!("get_app_servers");
    let url = get_full_url(&format!("{}?app_id={}", "/api/v1/app_servers", app_id));
    let resp = reqwest::blocking::get(&url).map_err(|_| Error::QueryError)?;
    let body = resp.text().map_err(|_| Error::ReadBodyError)?;
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<AppServerInfo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        return Error::DecodeError;
    })?;
    debug!("data: {:?}", data);
    Ok(data.servers)
}
