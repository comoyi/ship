use crate::error::Error;
use crate::requests::get_full_url;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Vo {
    apps: Vec<App>,
}
#[derive(Deserialize, Debug)]
pub struct App {
    pub id: u64,
    pub name: String,
    pub priority: i64,
    pub code: String,
}

pub fn get_apps() -> Result<Vec<App>, Error> {
    debug!("get_apps");
    let url = get_full_url("/api/v1/apps");
    let resp = reqwest::blocking::get(&url).map_err(|_| Error::QueryError)?;
    let body = resp.text().map_err(|_| Error::ReadBodyError)?;
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<Vo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        return Error::DecodeError;
    })?;
    debug!("data: {:?}", data);
    Ok(data.apps)
}
