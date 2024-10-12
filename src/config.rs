use std::{fs, path};
use std::path::PathBuf;
use serde::Deserialize;
use crate::maps::MapConfig;

#[derive(Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub maps: MapConfig,
}

pub fn load_config() -> anyhow::Result<AppConfig> {
    let config_path = PathBuf::from("config.toml");
    if !fs::exists(&config_path)? {
        println!("Config file not found at {:?}", path::absolute(config_path)?);
        todo!("create config file")
    }
    let config_file = fs::read_to_string(&config_path)?;
    let config = toml::from_str::<AppConfig>(&config_file)?;
    Ok(config)
}