use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::client::{TIMEOUT, USER_AGENT};

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
    Cli(Parameters),
    #[clap(about = "Load settings from a config file")]
    ConfigFile {
        #[clap(index = 1)]
        config_file: PathBuf,
    },
}

#[derive(Debug, Parser)]
pub struct Parameters {
    #[clap(short, long)]
    pub(crate) user_name: String,
    #[clap(short, long)]
    pub(crate) password: String,
    #[clap(short = 'a', long, default_value = USER_AGENT)]
    pub(crate) user_agent: String,
    #[clap(short, long, name = "SECS", default_value_t = TIMEOUT.as_secs())]
    pub(crate) timeout: u64,
    #[clap(subcommand)]
    pub(crate) action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    #[clap(about = "Activate offers")]
    Activate {
        #[clap(index = 1)]
        offers: Vec<u32>,
    },
    #[clap(about = "Bump offers to newest")]
    Bump {
        #[clap(index = 1)]
        offers: Vec<u32>,
    },
    #[clap(about = "Deactivate offers")]
    Deactivate {
        #[clap(index = 1)]
        offers: Vec<u32>,
    },
}
