use std::{fs, path::PathBuf};

use crate::error::Error::{self, MissingConfig};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
}

impl Config {
    pub fn load(dir: PathBuf) -> Result<Config, Error> {
        if let Ok(raw_config) = fs::read_to_string(dir.join("aiken.toml")) {
            let config = toml::from_str(&raw_config).unwrap();
            Ok(config)
        } else {
            Err(MissingConfig { path: dir })
        }
    }
}
