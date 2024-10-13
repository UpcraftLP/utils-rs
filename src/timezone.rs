use crate::maps::Location;
use anyhow::bail;
use chrono::{DateTime, NaiveDateTime};
use chrono_tz::Tz;
use reqwest::header::ACCEPT;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub async fn get_time_at(location: &Location) -> anyhow::Result<GetTimeResponse> {
    let client = reqwest::Client::new();

    let mut url = Url::parse("https://timeapi.io/api/time/current/coordinate")?;
    url.query_pairs_mut()
        .append_pair("latitude", &location.lat.to_string())
        .append_pair("longitude", &location.lon.to_string());
    let result = client
        .get(url)
        //TODO UA header
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .json::<GetTimeResponse>()
        .await?;
    Ok(result)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTimeResponse {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    seconds: i32,
    #[serde(rename = "milliSeconds")]
    milli_seconds: i32,
    #[serde(rename = "dateTime")]
    raw_date_time: String,
    #[serde(rename = "date")]
    raw_date: String,
    #[serde(rename = "time")]
    raw_time: String,
    #[serde(rename = "timeZone")]
    time_zone: String,
    #[serde(rename = "dayOfWeek")]
    day_of_week: DayOfWeek,
    #[serde(rename = "dstActive")]
    dst_active: bool,
}

impl GetTimeResponse {
    pub fn date_time(&self) -> anyhow::Result<DateTime<Tz>> {
        let time_zone = self.time_zone.parse::<Tz>()?;
        let date_time = self.raw_date_time.parse::<NaiveDateTime>()?;
        let mapped = date_time.and_local_timezone(time_zone).unwrap();
        Ok(mapped)
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Default for DayOfWeek {
    fn default() -> Self {
        DayOfWeek::Sunday
    }
}

impl Display for DayOfWeek {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DayOfWeek::Sunday => "Sunday",
                DayOfWeek::Monday => "Monday",
                DayOfWeek::Tuesday => "Tuesday",
                DayOfWeek::Wednesday => "Wednesday",
                DayOfWeek::Thursday => "Thursday",
                DayOfWeek::Friday => "Friday",
                DayOfWeek::Saturday => "Saturday",
            }
        )
    }
}

impl FromStr for DayOfWeek {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "sunday" => Ok(DayOfWeek::Sunday),
            "monday" => Ok(DayOfWeek::Monday),
            "tuesday" => Ok(DayOfWeek::Tuesday),
            "wednesday" => Ok(DayOfWeek::Wednesday),
            "thursday" => Ok(DayOfWeek::Thursday),
            "friday" => Ok(DayOfWeek::Friday),
            "saturday" => Ok(DayOfWeek::Saturday),
            _ => bail!("Invalid day of week: {s}"),
        }
    }
}
