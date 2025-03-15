use std::{fs, net::IpAddr, str::FromStr};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: Server
}

#[derive(Deserialize)]
pub struct Server {
    pub bind_interface: IpAddr,
    pub bind_port: u16,
    pub max_clients: u16
}

pub fn load_config() -> Config {
    let default_config = Config {
                                    server: Server { 
                                        bind_interface: IpAddr::from_str("127.0.0.1").unwrap(),
                                        bind_port: 21000, 
                                        max_clients: 10 }
                                };
    match fs::read_to_string("config.toml") {
        Ok(config_string) => {
            match toml::from_str(&config_string) {
                Ok(config) => config,
                Err(e) => {
                    println!("[ERROR] Failed to parse configuration: {e}. Falling back to default settings.");
                    default_config
                }
            }
        }
        Err(_) => {
            println!("[ERROR] Failed to read configuration file, using defaults");
            default_config
        }
    }
}