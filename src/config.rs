use crate::maps::MapConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, path};
use anyhow::Context;

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub maps: MapConfig,
}

pub fn create_default_config(path: &PathBuf) -> anyhow::Result<AppConfig> {
    let config = AppConfig::default();

    let cfg_string = toml::to_string_pretty(&config)?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, cfg_string)?;

    Ok(config)
}

pub fn load_config() -> anyhow::Result<AppConfig> {
    let config_path = path::absolute(
        dirs::config_dir().expect("Unable to find config directory")
            .join(format!(".{}", crate::APP_NAME))
            .join("config.toml")
    )?;

    if !config_path.try_exists()? {
        eprintln!("Config file not found at {}, creating default config...", &config_path.to_string_lossy().replace("\\", "/"));
        return Ok(create_default_config(&config_path)?);
    }
    let cfg_string = fs::read_to_string(&config_path)
        .context("Unable to read config file!")?;
    let config = toml::from_str::<AppConfig>(&cfg_string).unwrap_or_else(|e| {
        eprintln!("Error parsing config file: {e}");
        eprintln!("Using default config...");
        AppConfig::default()
    });
    Ok(config)
}