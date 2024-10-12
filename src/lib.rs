pub mod config;
pub mod maps;
pub mod timezone;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub const VERSION: Option<&str> = built_info::GIT_VERSION;
pub const APP_NAME: &'static str = "up_utils";