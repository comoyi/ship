use crate::config::CONFIG;
use crate::request::error::Error;
use crate::version;
use std::time::Duration;

pub mod app_server;
pub mod check_update;
mod error;
pub mod get_app_servers;
pub mod get_apps;

fn get_full_url(path: &str) -> String {
    let address = CONFIG.server.address.to_address_string();
    format!("{}{}", address, path)
}

fn get_full_url_by_server_address(path: &str, server_address: &str) -> String {
    format!("{}{}", server_address, path)
}

fn get(url: &str) -> Result<reqwest::blocking::Response, Error> {
    let version_no = version::VERSION_NO;
    let version_text = version::VERSION_TEXT;
    let channel_code = &CONFIG.distribution_channel_code;
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .map_err(|_| Error::BuildRequestError)?
        .get(url)
        .header("version_no", version_no)
        .header("version_text", version_text)
        .header("channel_code", channel_code)
        .send()
        .map_err(|_| Error::RequestError)
}
