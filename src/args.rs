use crate::config::Config;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

const DESCRIPTION: &str = "Bump advertisements on wg-gesucht.de";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = DESCRIPTION)]
pub struct Args {
    #[clap(subcommand)]
    pub mode: Mode,
}

#[derive(Debug, Subcommand)]
pub enum Mode {
    Cli(Settings),
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
    #[clap(index = 1)]
    pub ad_ids: Vec<u32>,
}

impl From<Config> for Settings {
    fn from(config: Config) -> Self {
        Self {
            user_name: config.user_name,
            password: config.password,
            user_agent: config.user_agent.unwrap_or_else(|| USER_AGENT.to_string()),
            ad_ids: config.ad_ids,
        }
    }
}
