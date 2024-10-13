use crate::maps::{Location, MapsProvider};
use async_trait::async_trait;
use reqwest::header::ACCEPT;
use reqwest::Url;
use serde::{de, Deserialize, Deserializer, Serialize};

use serde_aux::prelude::*;

pub struct MapsCoProvider {
    api_key: String,
}

impl MapsCoProvider {
    pub fn new(api_key: &str) -> Self {
        MapsCoProvider {
            api_key: api_key.to_string(),
        }
    }
}

#[async_trait]
impl MapsProvider for MapsCoProvider {
    async fn get_location(&self, query: &str) -> anyhow::Result<Option<Location>> {
        let client = reqwest::Client::new();
        let mut url = Url::parse("https://geocode.maps.co/search")?;
        url.query_pairs_mut()
            .append_pair("q", query)
            .append_pair("api_key", &self.api_key);
        let result = client
            .get(url)
            .header(ACCEPT, "application/json")
            // TODO UA header
            .send()
            .await?
            .json::<Vec<MapsCoResponse>>()
            .await?;

        if result.is_empty() {
            return Ok(None);
        }

        let first = &result[0];

        Ok(Some(Location {
            lat: first.lat,
            lon: first.lon,
            name: first.display_name.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MapsCoResponse {
    place_id: u64,
    license: Option<String>,
    osm_type: Option<String>,
    osm_id: Option<u64>,
    #[serde(rename = "boundingbox", deserialize_with = "deserialize_f64_vec")]
    bounding_box: Vec<f64>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    lat: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    lon: f64,
    display_name: Option<String>,
    class: Option<String>,
    #[serde(rename = "type")]
    _type: Option<String>,
    importance: Option<f64>,
}

fn deserialize_f64_vec<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize as Vec<String> first
    let strings: Vec<String> = Vec::deserialize(deserializer)?;

    // Try to parse each string into f64
    let result: Result<Vec<f64>, _> = strings
        .into_iter()
        .map(|s| s.parse::<f64>().map_err(de::Error::custom))
        .collect();

    result
}
