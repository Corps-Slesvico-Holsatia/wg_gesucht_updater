use crate::config::Config;
use crate::functions::parse_duration;
use crate::session::{TIMEOUT, USER_AGENT};
use clap::{Parser, Subcommand};
use serde_rw::FromFile;
use std::path::PathBuf;
use std::time::Duration;

const DESCRIPTION: &str = "Bump advertisements on wg-gesucht.de";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = DESCRIPTION)]
pub struct Args {
    #[clap(subcommand)]
    mode: Mode,
}

impl Args {
    /// Returns the configuration settings
    ///
    /// # Errors
    /// Returns an `[anyhow::Error]` in case the config file parsing fails
    pub fn settings(self) -> anyhow::Result<Settings> {
        match self.mode {
            Mode::Cli(settings) => Ok(settings),
            Mode::ConfigFile { config_file } => Config::from_file(config_file).map(Into::into),
        }
    }
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
    pub user_name: String,
    #[clap(short, long)]
    pub password: String,
    #[clap(short = 'a', long, default_value = USER_AGENT)]
    pub user_agent: String,
    #[clap(short, long, value_parser = parse_duration, name = "SECS", default_value = "10")]
    pub timeout: Duration,
    #[clap(index = 1)]
    pub ad_ids: Vec<u32>,
}

impl From<Config> for Settings {
    fn from(config: Config) -> Self {
        Self {
            user_name: config.user_name,
            password: config.password,
            user_agent: config.user_agent.unwrap_or_else(|| USER_AGENT.to_string()),
            timeout: config.timeout.unwrap_or(TIMEOUT),
            ad_ids: config.ad_ids,
        }
    }
}
