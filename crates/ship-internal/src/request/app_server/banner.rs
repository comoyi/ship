use crate::request::error::Error;
use crate::request::{get, get_full_url_by_server_address};
use crate::types::banner::Banner;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BannerVo {
    pub banners: Vec<Banner>,
}

pub fn get_banner(server_address: &str) -> Result<BannerVo, Error> {
    debug!("get_banner");
    let url = get_full_url_by_server_address("/api/v1/banner", server_address);
    let resp = get(&url)?;
    let body = resp.text().map_err(|_| Error::ReadBodyError)?;
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<BannerVo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        Error::DecodeError
    })?;
    debug!("data: {:?}", data);
    Ok(data)
}
