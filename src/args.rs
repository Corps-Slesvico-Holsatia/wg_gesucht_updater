use crate::session::{TIMEOUT, USER_AGENT};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

impl From<Action> for (Vec<u32>, Vec<u32>, Vec<u32>) {
    fn from(action: Action) -> Self {
        match action {
            Action::Activate { offers } => (offers, Vec::with_capacity(0), Vec::with_capacity(0)),
            Action::Bump { offers } => (Vec::with_capacity(0), offers, Vec::with_capacity(0)),
            Action::Deactivate { offers } => (Vec::with_capacity(0), Vec::with_capacity(0), offers),
        }
    }
}
