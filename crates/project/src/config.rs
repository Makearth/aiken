use std::{fs, io, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
}

impl Config {
    pub fn load(dir: PathBuf) -> io::Result<Config> {
        if let Ok(raw_config) = fs::read_to_string(dir.join("aiken.toml")){
            let config = toml::from_str(&raw_config).unwrap();
            Ok(config)
        }else{
            Err(io::Error::new(io::ErrorKind::NotFound, "Not in build directory: aiken.toml"))
        }
    }
}