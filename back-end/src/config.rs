/// config mod implements the loading of conf/config.yaml.
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub server: Server,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: usize,
}

lazy_static! {
    pub static ref CONFIG: Configuration = match get_config("./conf/config.yaml") {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
}

/// get_config loads conf/config.yaml
pub fn get_config(filename: &str) -> Result<Configuration, &'static str> {
    let content: String = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_e) => {
            return Result::Err("read config file failed");
        }
    };

    Result::Ok(serde_yaml::from_str(&content).unwrap())
}
