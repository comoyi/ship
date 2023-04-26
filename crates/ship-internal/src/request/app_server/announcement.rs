use crate::request::error::Error;
use crate::request::{get, get_full_url_by_server_address};
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AnnouncementVo {
    pub content: String,
}

pub fn get_announcement(server_address: &str) -> Result<AnnouncementVo, Error> {
    debug!("get_announcement");
    let url = get_full_url_by_server_address("/api/v1/announcement", server_address);
    let resp = get(&url)?;
    let body = resp.text().map_err(|_| Error::ReadBodyError)?;
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<AnnouncementVo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        Error::DecodeError
    })?;
    debug!("data: {:?}", data);
    Ok(data)
}
