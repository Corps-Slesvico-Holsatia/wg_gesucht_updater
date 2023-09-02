use clap::Parser;
use std::process::exit;
use wg_gesucht_updater::{Args, Client};

#[tokio::main]
async fn main() {
    let mut client: Client = Args::parse().try_into().unwrap_or_else(|error| {
        eprintln!("Could not parse config file: {error}");
        exit(1);
    });
    exit(client.run().await);
}
