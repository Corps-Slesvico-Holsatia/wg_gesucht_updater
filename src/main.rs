use clap::Parser;
use wg_gesucht_updater::{Args, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp(None).init();
    Ok(Settings::try_from(Args::parse())?.apply().await?)
}
