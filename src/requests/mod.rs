use crate::config::CONFIG;

mod get_file_info;
mod get_info;

use crate::data::common::AppServer;
pub use get_file_info::get_file_info;
pub use get_info::get_info;

fn get_full_url(path: &str) -> String {
    let address = CONFIG.server.address.to_address_string();
    format!("{}{}", address, path)
}

fn get_full_url_by_server(path: &str, server: &AppServer) -> String {
    let address = server.address.to_address_string();
    format!("{}{}", address, path)
}
