use clap::{Args, Parser};

mod command;
pub use command::run;

mod commands;
use commands::*;

mod config;
pub use config::*;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[doc = "Path to configuration file (TOML format)"]
    #[arg(long = "config", global = true)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn init() -> Self {
        Cli::parse()
    }
}
