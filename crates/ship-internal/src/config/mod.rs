mod error;

use crate::config::error::Error;
use config::builder::DefaultState;
use config::ConfigBuilder;
use lazy_static::lazy_static;
use log::debug;
use serde::Deserialize;
use std::path::Path;
use util::filepath;

lazy_static! {
    pub static ref CONFIG: Config = init_config().unwrap();
}

#[derive(Deserialize)]
pub struct Config {
    pub log_level: String,
    pub language: String,
    pub server: Server,
}

impl Config {
    pub fn print_config(&self) {
        debug!("log_level: {}", self.log_level);
        debug!(
            "server.address: {}",
            self.server.address.to_address_string()
        );
    }
}

#[derive(Deserialize)]
pub struct Server {
    pub address: Address,
}

#[derive(Deserialize)]
pub struct Address {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

impl Address {
    pub fn to_address_string(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

fn init_config() -> Result<Config, Error> {
    let mut b = config::Config::builder();

    b = set_default(b);

    let exe_dir = filepath::get_exe_dir().map_err(|e| {
        println!("get exe_dir failed, err: {}", e);
        Error::GetExeDirFailed
    })?;
    let config_path = Path::new(&exe_dir).join("config.toml");
    let cps = vec![config_path];
    for cp_str in cps {
        let cp = Path::new(&cp_str);
        if cp.exists() {
            b = b.add_source(config::File::from(cp))
        }
    }

    let c = b.build().map_err(|e| {
        println!("build config failed: {}", e);
        Error::BuildConfigFailed
    })?;
    let conf = c.try_deserialize::<Config>().map_err(|e| {
        println!("deserialize config failed: {}", e);
        Error::DeserializeConfigFailed
    })?;
    Ok(conf)
}

fn set_default(b: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
    b.set_default("log_level", "OFF")
        .unwrap()
        .set_default("language", "en_US")
        .unwrap()
}
