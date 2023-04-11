use crate::data::common::AppServer;
use crate::error::Error;
use crate::requests::get_full_url_by_server;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
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

pub fn get_info(server: &AppServer) -> Result<Info, Error> {
    debug!("get_info");
    let url = get_full_url_by_server("/api/v1/info", server);
    let resp = reqwest::blocking::get(&url).map_err(|_| Error::QueryError)?;
    let body_r = resp.text();
    let body = match body_r {
        Ok(v) => v,
        Err(_) => {
            return Err(Error::ReadBodyError);
        }
    };
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<Info>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        return Error::DecodeError;
    })?;
    debug!("data: {:?}", data);
    Ok(data)
}
