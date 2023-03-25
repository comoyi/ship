use crate::data::{Server, ServerFileInfo};
use crate::error::Error;
use log::debug;

pub fn start(id: String, servers: &Vec<Server>) {
    debug!("update, id: {}", id);
    let server_o = get_server_by_id(id.to_string(), servers);
    let server;
    match server_o {
        None => {
            debug!("server not found, id: {}", id);
            return;
        }
        Some(s) => {
            server = s;
        }
    }
    let j = serde_json::to_string(&server);
    debug!(
        "found server, id: {}, server: {:?}",
        id,
        j.unwrap_or("".to_string())
    );
    let sfi = get_server_file_info(&server);
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

fn get_server_by_id(id: String, servers: &Vec<Server>) -> Option<&Server> {
    let mut server = None;
    for s in servers {
        if s.id == id {
            server = Some(s);
            break;
        }
    }
    server
}
