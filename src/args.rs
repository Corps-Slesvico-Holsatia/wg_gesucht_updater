use crate::functions::parse_duration;
use crate::session::USER_AGENT;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::time::Duration;

const DESCRIPTION: &str = "Bump advertisements on wg-gesucht.de";

/// Command line arguments
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = DESCRIPTION)]
pub struct Args {
    #[clap(subcommand)]
    pub(crate) mode: Mode,
}

#[derive(Debug, Subcommand)]
pub enum Mode {
    #[clap(about = "Pass settings via command line arguments")]
    Cli(Settings),
    #[clap(about = "Load settings from a config file")]
    ConfigFile {
        #[clap(index = 1)]
        config_file: PathBuf,
    },
}

#[derive(Debug, Parser)]
pub struct Settings {
    #[clap(short, long)]
    pub(crate) user_name: String,
    #[clap(short, long)]
    pub(crate) password: String,
    #[clap(short = 'a', long, default_value = USER_AGENT)]
    pub(crate) user_agent: String,
    #[clap(short, long, value_parser = parse_duration, name = "SECS", default_value = "10")]
    pub(crate) timeout: Duration,
    #[clap(subcommand)]
    pub(crate) action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    Activate {
        #[clap(index = 1)]
        ad_ids: Vec<u32>,
    },
    Bump {
        #[clap(index = 1)]
        ad_ids: Vec<u32>,
    },
    Deactivate {
        #[clap(index = 1)]
        ad_ids: Vec<u32>,
    },
}
