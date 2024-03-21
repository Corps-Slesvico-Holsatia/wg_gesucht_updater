use clap::Parser;
use log::error;
use std::process::exit;
use wg_gesucht_updater::{Args, Error, Settings};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::builder().format_timestamp(None).init();

    Settings::try_from(Args::parse())
        .unwrap_or_else(|error| {
            error!("Could not parse config file: {error}");
            exit(1);
        })
        .apply()
        .await
}
