use crate::config::CONFIG;

mod get_file_info;
mod get_info;

pub use get_file_info::get_file_info;
pub use get_info::get_info;

fn get_full_url(path: &str) -> String {
    let address = CONFIG.server.address.to_address_string();
    format!("{}{}", address, path)
}
