//! CLI tool to programmatically update offers on wg-gesucht.de.

pub use args::Args;
pub use error::{Error, FailedUpdates};
pub use settings::Settings;

mod args;
mod auth_data;
mod client;
mod config_file;
mod error;
mod functions;
mod login_data;
mod patch_data;
mod settings;

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp(None).init();
    Ok(Settings::try_from(Args::parse())?.apply().await?)
}
