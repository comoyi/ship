use crate::data::{Server, ServerFileInfo};
use crate::error::Error;
use log::debug;

pub fn start() {
    let s = Server {
        name: "".to_string(),
        protocol: "".to_string(),
        host: "".to_string(),
        port: 0,
        dir: "".to_string(),
        file_info: None,
        selected: false,
    };
    let sfi = get_server_file_info(&s);
    if let Err(_) = sfi {
        return;
    }
}

fn get_server_file_info(s: &Server) -> Result<ServerFileInfo, Error> {
    let url = get_full_url("/files", s);
    debug!("url: {}", url);
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

pub fn get_full_url(u: &str, s: &Server) -> String {
    format!("{}://{}:{}{}", s.protocol, s.host, s.port, u)
}
