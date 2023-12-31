use clap::Parser;
use log::error;
use std::process::exit;
use wg_gesucht_updater::{Args, Client};

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut client: Client = Args::parse().try_into().unwrap_or_else(|error| {
        error!("Could not parse config file: {error}");
        exit(1);
    });

    if let Err(errors) = client.run().await {
        exit(i32::try_from(errors.len()).unwrap_or(i32::MAX));
    }

    exit(0);
}
