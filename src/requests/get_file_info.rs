use crate::data::common::{AppServer, ServerFileInfo};
use crate::error::Error;
use crate::requests::get_full_url_by_server;
use log::debug;

pub fn get_file_info(server: &AppServer) -> Result<ServerFileInfo, Error> {
    debug!("get_file_info");
    let url = get_full_url_by_server("/files", server);
    let resp = reqwest::blocking::get(&url).map_err(|_| Error::QueryError)?;
    let body_r = resp.text();
    let body = match body_r {
        Ok(v) => v,
        Err(_) => {
            return Err(Error::ReadBodyError);
        }
    };
    debug!("url: {}, body: {}", url, body);
    let data = serde_json::from_str::<ServerFileInfo>(&body).map_err(|e| {
        debug!("decode failed, err: {}", e);
        return Error::DecodeError;
    })?;
    debug!("data: {:?}", data);
    Ok(data)
}
