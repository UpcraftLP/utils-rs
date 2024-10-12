use chrono::{Local, Offset};
use clap::crate_authors;
use clap::Parser;

#[derive(Parser)]
#[clap(arg_required_else_help = true)]
#[command(author = crate_authors!(), version = utils::VERSION, about, name = "localtime")]
struct Args {
    #[arg(trailing_var_arg = true)]
    location: Vec<String>,
}

impl Args {
    fn args_string(&self) -> String {
        self.location.join(" ")
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let cfg = utils::config::load_config()?;
    let location = cfg.maps.create_provider().get_location(&args.args_string()).await?;

    if location.is_none() {
        println!("Unable to find location: '{}' with provider '{}'", args.args_string(), cfg.maps);
        return Ok(());
    }

    match location {
        None => println!("No location found for '{}'", args.args_string()),
        Some(location) => {
            let time_info = utils::maps::Location::get_time_at(&location).await?;

            println!("Location: {}", location.name.unwrap_or_else(|| args.args_string()));
            println!("Latitude: {}", location.lat);
            println!("Longitude: {}", location.lon);
            println!("Timezone: {}", time_info.timezone().name());
            println!("Time: {}", time_info.format("%Y/%m/%d %H:%M:%S"));

            let target_offset_utc = time_info.offset().fix().local_minus_utc();
            let local_offset_utc = Local::now().offset().fix().local_minus_utc();

            let difference = target_offset_utc - local_offset_utc;
            if difference != 0 {
                let hours = difference / 3600;
                let minutes_part = ((difference % 3600) / 60).abs();
                println!("Offset to current time: {hours:+03}:{minutes_part:02}");
            }
        }
    }
    Ok(())
}
