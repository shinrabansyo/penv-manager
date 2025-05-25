use std::env;
use std::fs;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub channel: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let path = format!("{}/.shinrabansyo/config.toml", env::var("HOME")?);
        let config = fs::read_to_string(path)?;
        let config = toml::from_str::<Config>(&config)?;
        Ok(config)
    }
}
