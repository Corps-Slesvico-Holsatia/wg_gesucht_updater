use clap::Parser;
use std::process::exit;
use wg_gesucht_updater::{Args, Session};

#[tokio::main]
async fn main() {
    let settings = Args::parse().settings().unwrap_or_else(|error| {
        eprintln!("Could not parse config file: {error}");
        exit(1);
    });

    match Session::new(&settings.user_agent) {
        Ok(mut session) => {
            session
                .login(&settings.user_name, &settings.password)
                .await
                .unwrap_or_else(|error| {
                    eprintln!("{error}");
                    exit(2)
                });
            for ad_id in settings.ad_ids {
                println!("Bumping ad: {ad_id}");
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
