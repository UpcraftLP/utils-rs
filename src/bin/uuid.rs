use clap::crate_authors;
use clap::{arg, Parser};
use uuid::Uuid;

#[derive(Parser)]
#[command(author = crate_authors!(), version, about, long_about = None, name = "uuid")]
struct Args {
    #[arg(short = 's', long, help = "output a 'slim' UUID without hyphens")]
    slim: bool,
    #[arg(short = 'g', long, help = "output a Microsoft style GUID")]
    guid: bool,
    #[arg(short = 'C', long, help = "capitalize the output", default_value = "false")]
    capitalize: bool,
}

fn main() {
    let args = Args::parse();
    let id = Uuid::new_v4();
    
    let mut id_str = match args.slim {
        true => id.as_simple().to_string(),
        false => id.as_hyphenated().to_string(),
    };
    
    if args.capitalize {
        id_str = id_str.to_uppercase();
    }
    
    if args.guid {
        id_str = format!("{{{:}}}", id_str);
    }
    
    println!("{id_str}");
}