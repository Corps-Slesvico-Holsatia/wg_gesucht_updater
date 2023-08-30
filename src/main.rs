use clap::Parser;
use serde_rw::FromFile;
use std::process::exit;
use wg_gesucht_updater::{Args, Config, Mode, Session};

#[tokio::main]
async fn main() {
    let settings = match Args::parse().mode {
        Mode::Cli(settings) => settings,
        Mode::ConfigFile { config_file } => Config::from_file(config_file)
            .unwrap_or_else(|error| {
                eprintln!("Could not parse config file: {error}");
                exit(1);
            })
            .into(),
    };

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
