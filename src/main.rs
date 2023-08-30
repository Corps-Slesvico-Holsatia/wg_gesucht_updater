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

    match Session::new(args.user_name.as_str(), args.password.as_str()) {
        Ok(mut session) => {
            session.login().await.unwrap_or_else(|error| {
                eprintln!("{error}");
                exit(2)
            });
            for ad_id in args.ad_ids {
                session.update(ad_id).await.unwrap_or_else(|error| {
                    eprintln!("Could not update ad {ad_id}: {error}");
                    exit(3);
                });
            }
        }
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    }
}
