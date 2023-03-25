use crate::config::CONFIG;
use crate::data::ServerFileInfo;
use crate::error::Error;
use log::debug;

pub fn start() {
    let sfi = get_server_file_info();
    if let Err(_) = sfi {
        return;
    }
}

fn get_server_file_info() -> Result<ServerFileInfo, Error> {
    let url = get_full_url("/files");
    let resp_r = reqwest::blocking::get(url);
    match resp_r {
        Ok(resp) => {
            let sfi_r = resp.json::<ServerFileInfo>();
            match sfi_r {
                Ok(sfi) => {
                    debug!("{}", serde_json::to_string(&sfi).unwrap());
                    Ok(sfi)
                }
                Err(_) => Err(Error::DeserializeServerFileInfoError),
            }
        }
        Err(_) => Err(Error::GetServerFileInfoError),
    }
}

pub fn get_full_url(u: &str) -> String {
    format!("{}://{}:{}{}", CONFIG.protocol, CONFIG.host, CONFIG.port, u)
}
