use std::fs::File;
use std::io::prelude::*;

use serde_derive::Deserialize;
use toml;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub addr: String,
    pub db_url: String,
    pub admin_user: String,
    pub admin_password: String,
    pub admin_email: String,
    pub default_avatar: String,
    pub secret_key: String,
    pub expires_at: usize,
    pub salt: String,
}

pub fn read_config() -> Config {
    let mut contents = String::new();    
    let mut file = File::open("config.toml").unwrap();
    file.read_to_string(&mut contents).unwrap();

    let config: Config = toml::from_str(&contents).unwrap();
    config
}
