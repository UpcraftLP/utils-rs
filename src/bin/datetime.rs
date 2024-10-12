use chrono::{DateTime, Local, Utc};
use clap::builder::PossibleValue;
use clap::crate_authors;
use clap::{Parser, ValueEnum};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Parser)]
#[command(author = crate_authors!(), version = utils::VERSION, about, name = "datetime")]
struct Args {
    #[arg(short, long, default_value = "unix", value_enum)]
    epoch: Epoch,

    #[arg(long = "ms", default_value = "false", help = "output milliseconds instead of seconds")]
    milliseconds: bool,
}

#[derive(Clone, Copy)]
enum Epoch {
    Unix,
    TwitterSnowflake,
    DiscordSnowflake,
    Custom(DateTime<Utc>),
}

impl Epoch {
    pub fn to_utc(self) -> DateTime<Utc> {
        match self {
            Epoch::Unix => DateTime::from_timestamp_nanos(0),
            Epoch::TwitterSnowflake => DateTime::from_timestamp_nanos(1288834974657000000),
            Epoch::DiscordSnowflake => DateTime::from_timestamp_nanos(1420070400000000000),
            Epoch::Custom(dt) => dt,
        }
    }
}

impl FromStr for Epoch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "epoch" => Ok(Epoch::Unix),
            "unix" => Ok(Epoch::Unix),
            "twitter" => Ok(Epoch::TwitterSnowflake),
            "discord" => Ok(Epoch::DiscordSnowflake),
            _ => s.parse::<DateTime<Local>>().map_err(|e| format!("failed to parse DateTime: {e}")).map(|value| Epoch::Custom(value.to_utc())),
        }
    }
}

impl Display for Epoch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Epoch::Unix => "unix",
            Epoch::TwitterSnowflake => "twitter",
            Epoch::DiscordSnowflake => "discord",
            Epoch::Custom(dt) => &*dt.to_rfc3339(),
        };

        f.write_str(value)
    }
}

impl ValueEnum for Epoch {
    fn value_variants<'a>() -> &'a [Self] {
        // "2021-01-01T00:00:00Z"
        const EXAMPLE_TIMESTAMP: DateTime<Utc> = DateTime::from_timestamp_nanos(1609459200000000000);
        
        &[Epoch::Unix, Epoch::TwitterSnowflake, Epoch::DiscordSnowflake, Epoch::Custom(EXAMPLE_TIMESTAMP)]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Epoch::Unix => PossibleValue::new("unix").help("the Unix epoch (Jan 1, 1970)"),
            Epoch::TwitterSnowflake => PossibleValue::new("twitter").help("the Twitter Snowflake epoch (Jan 1, 2010)"),
            Epoch::DiscordSnowflake => PossibleValue::new("discord").help("the Discord Snowflake epoch (Jan 1, 2015)"),
            Epoch::Custom(dt) => PossibleValue::new("custom").help(format!("a custom epoch in RFC3339 format. Example: {timestamp}", timestamp = dt.to_rfc3339())),
        })
    }
}

fn main() {
    let args = Args::parse();
    let now = Utc::now();

    if now < args.epoch.to_utc() {
        println!("specified epoch is later than the current time");
    } else {
        let elapsed = now.signed_duration_since(args.epoch.to_utc());
        if args.milliseconds {
            println!("{:?}", elapsed.num_milliseconds());
        } else {
            println!("{:?}", elapsed.num_seconds());
        }
    }
}