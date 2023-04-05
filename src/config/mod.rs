use crate::utils::filepath;
use config::builder::DefaultState;
use config::ConfigBuilder;
use lazy_static::lazy_static;
use log::debug;
use serde::Deserialize;
use std::path::Path;

lazy_static! {
    pub static ref CONFIG: Config = init_config();
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

pub fn init_config() -> Config {
    let mut b = config::Config::builder();

    b = set_default(b);

    let exe_dir_r = filepath::get_exe_dir();
    let exe_dir = match exe_dir_r {
        Ok(exe_dir) => exe_dir,
        Err(e) => {
            panic!("get exe_dir failed, err: {}", e);
        }
    };
    let config_path = Path::new(&exe_dir).join("config.toml");
    let cps = vec![config_path];
    for cp_str in cps {
        let cp = Path::new(&cp_str);
        if cp.exists() {
            // println!("Add config file: {:?}", cp);
            b = b.add_source(config::File::from(cp))
        }
    }

    let c = b.build().unwrap();
    let conf_r = c.try_deserialize::<Config>();
    let conf = match conf_r {
        Ok(c) => c,
        Err(e) => {
            println!("load config failed: {}", e);
            panic!("load config failed: {}", e);
        }
    };
    // println!("{:?}", conf);
    conf
}

fn set_default(b: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
    b.set_default("log_level", "TRACE").unwrap()
}
