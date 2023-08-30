use clap::Parser;
use std::process::exit;
use wg_gesucht_updater::Session;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short, long)]
    user_name: String,
    #[clap(short, long)]
    password: String,
    #[clap(index = 1)]
    ad_ids: Vec<u32>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    Session::new(args.user_name.as_str(), args.password.as_str())
        .unwrap_or_else(|error| {
            eprintln!("{error}");
            exit(1);
        })
        .update_all(args.ad_ids.as_slice())
        .await
        .unwrap_or_else(|error| {
            eprintln!("{error}");
            exit(2);
        });
}
