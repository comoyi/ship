use crate::request::error::Error;
use crate::request::{get, get_full_url_by_server_address};
use log::debug;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct AppServerVo {
    pub data_nodes: Vec<DataNode>,
}

#[derive(Deserialize, Debug)]
pub struct DataNode {
    pub name: String,
    pub address: Address,
}

#[derive(Deserialize, Debug)]
pub struct Address {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

pub fn get_app_server(server_address: &str) -> Result<AppServerVo, Error> {
    debug!("get_app_server");
    let url = get_full_url_by_server_address("/api/v1/info", server_address);
    let resp = get(&url)?;
    let body = resp.text().map_err(|_| Error::ReadBodyError)?;
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<AppServerVo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        Error::DecodeError
    })?;
    debug!("data: {:?}", data);
    Ok(data)
}
