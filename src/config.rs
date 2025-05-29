use serde::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub local_bind: String,
    pub forward_to: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Self> {
        let contents: String = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}