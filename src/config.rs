use serde::Deserialize;
use std::fs;
use anyhow::Result;

use toml::map::Map;
use toml::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub local_bind: String,
    pub default_server: String,
    pub servers: Map<String, Value>,
}

impl Config {
    pub fn new(path: &str) -> Result<Self> {
        let contents: String = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;

        println!("-------- Servers --------");
        for (server, data) in &config.servers {
            println!("{}: {}", server, data["ip"]);
        }
        println!("-------------------------\n");

        Ok(config)
    }
}