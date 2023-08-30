use clap::Parser;
use std::process::exit;
use wg_gesucht_updater::Session;

const DESCRIPTION: &str = "Bump advertisements on wg-gesucht.de";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = DESCRIPTION)]
pub struct Args {
    #[clap(short, long)]
    user_name: String,
    #[clap(short, long)]
    password: String,
    #[clap(short = 'a', long, default_value = USER_AGENT)]
    user_agent: String,
    #[clap(index = 1)]
    ad_ids: Vec<u32>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match Session::new(USER_AGENT) {
        Ok(mut session) => {
            session
                .login(&args.user_name, &args.password)
                .await
                .unwrap_or_else(|error| {
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
