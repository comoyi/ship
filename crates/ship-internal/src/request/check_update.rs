use crate::request::error::Error;
use crate::request::{get, get_full_url};
use crate::version::version_manage::NewVersionInfo;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct CheckUpdateVo {
    pub new_version: NewVersionInfo,
}

pub fn check_update() -> Result<CheckUpdateVo, Error> {
    debug!("check_update");
    let url = get_full_url("/api/v1/version/check");
    let resp = get(&url)?;
    let body = resp.text().map_err(|_| Error::ReadBodyError)?;
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<CheckUpdateVo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        Error::DecodeError
    })?;
    debug!("data: {:?}", data);
    Ok(data)
}
