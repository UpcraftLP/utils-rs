use std::fmt::Display;
use async_trait::async_trait;
use chrono::DateTime;
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use crate::timezone;

mod maps_co;
mod no_op;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "provider")]
pub enum MapConfig {
    #[serde(rename = "maps.co")]
    MapsCo { api_key: String },
    #[serde(rename = "none")]
    None,
}

impl MapConfig {
    pub fn create_provider(&self) -> Box<dyn MapsProvider> {
        let provider: Box<dyn MapsProvider> = match self {
            MapConfig::MapsCo { api_key } => Box::new(maps_co::MapsCoProvider::new(api_key)),
            MapConfig::None => Box::new(no_op::NoOpMapsProvider{}),
        };
        provider
    }
}

impl Default for MapConfig {
    fn default() -> Self {
        MapConfig::None
    }
}

#[async_trait]
pub trait MapsProvider {
    async fn get_location(&self, query: &str) -> anyhow::Result<Option<Location>>;
}

impl Display for MapConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MapConfig::MapsCo { .. } => "maps.co",
            MapConfig::None => "none",
        })
    }
}

pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub name: Option<String>,
}

impl Location {
    pub async fn get_time_at(&self) -> anyhow::Result<DateTime<Tz>> {
        timezone::get_time_at(self).await?.date_time()
    }
}