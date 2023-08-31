use clap::Parser;
use std::process::exit;
use wg_gesucht_updater::{Args, Configuration, Session};

#[tokio::main]
async fn main() {
    let parameters: Configuration = Args::parse().try_into().unwrap_or_else(|error| {
        eprintln!("Could not parse config file: {error}");
        exit(1);
    });

    match Session::new(parameters.timeout, &parameters.user_agent) {
        Ok(mut session) => {
            session
                .login(&parameters.user_name, &parameters.password)
                .await
                .unwrap_or_else(|error| {
                    eprintln!("{error}");
                    exit(2)
                });
            let mut exit_code = 0;

            for ad_id in parameters.deactivate {
                println!("Deactivating ad: {ad_id}");
                session.deactivate(ad_id).await.unwrap_or_else(|error| {
                    eprintln!("Could not deactivate ad {ad_id}: {error}");
                    exit_code = 3;
                });
            }

            for ad_id in parameters.activate {
                println!("Activating ad: {ad_id}");
                session.activate(ad_id).await.unwrap_or_else(|error| {
                    eprintln!("Could not activate ad {ad_id}: {error}");
                    exit_code = 3;
                });
            }

            for ad_id in parameters.bump {
                println!("Bumping ad: {ad_id}");
                session.bump(ad_id).await.unwrap_or_else(|error| {
                    eprintln!("Could not bump ad {ad_id}: {error}");
                    exit_code = 3;
                });
            }

            exit(exit_code);
        }
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    }
}
