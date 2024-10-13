use clap::crate_authors;
use clap::Parser;
use cpu_time::{ProcessTime, ThreadTime};
use std::io::ErrorKind::NotFound;
use std::process::{Command, Stdio};
use std::time::Duration;
use which::which;

#[derive(Debug, Parser)]
#[clap(arg_required_else_help = true)]
#[command(author = crate_authors!(), version = utils::VERSION, about, name = "time")]
struct Args {
    #[arg(short = 'p', long_help = "Use the POSIX standard output format")]
    posix: bool,
    program: String,
    args: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let program_path = which(&args.program).unwrap_or_else(|_| {
        eprintln!("{}: command not found", args.program);
        std::process::exit(127);
    });

    match program_path.metadata() {
        Ok(metadata) => {
            if !metadata.is_file() {
                eprintln!("{}: Is not a regular file", program_path.display());
                std::process::exit(126);
            }

            let mut cmd = Command::new(&args.program);
            cmd.stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
            if !args.args.is_empty() {
                cmd.args(&args.args);
            }

            let start_real = std::time::Instant::now();
            let start_sys = ProcessTime::now();
            let start_thread = ThreadTime::now();
            let exit_code = cmd.status();
            let end_thread = start_thread.elapsed();
            let end_sys = start_sys.elapsed();
            let end_real = start_real.elapsed();

            let print_duration = |name: &str, duration: Duration| match args.posix {
                true => eprintln!(
                    "{name} {seconds:?}s",
                    seconds = duration.as_secs() as f64
                        + (duration.subsec_nanos() as f64 / 1_000_000_000f64)
                ),
                false => {
                    let minutes = duration.as_secs() / 60;
                    let seconds = (duration.as_secs() % 60) as f64
                        + (duration.subsec_nanos() as f64 / 1_000_000_000f64);
                    let name_str = name.to_owned() + ":";
                    eprintln!("{name_str:8}{minutes}m{seconds:.03}s")
                }
            };

            print_duration("real", end_real);
            print_duration("user", end_thread);
            print_duration("sys", end_sys);

            match exit_code {
                Ok(status) => {
                    std::process::exit(status.code().unwrap_or(1));
                }
                Err(e) => {
                    eprintln!("{}: {}", program_path.display(), e);
                    std::process::exit(2);
                }
            }
        }
        Err(e) => {
            if e.kind() == NotFound {
                eprintln!("No such file: {}", program_path.display());
                std::process::exit(127);
            }

            eprintln!(
                "{}: {}",
                program_path
                    .file_name()
                    .unwrap_or_else(|| program_path.as_os_str())
                    .to_string_lossy(),
                e
            );
            std::process::exit(126);
        }
    }
}
