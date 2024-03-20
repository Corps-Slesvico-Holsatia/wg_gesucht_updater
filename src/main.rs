use clap::Parser;
use log::error;
use std::process::exit;
use wg_gesucht_updater::{Args, Settings};

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();

    if let Err(errors) = Settings::try_from(Args::parse())
        .unwrap_or_else(|error| {
            error!("Could not parse config file: {error}");
            exit(1);
        })
        .apply()
        .await
    {
        exit(i32::try_from(errors.len()).unwrap_or(i32::MAX));
    }

    exit(0);
}
