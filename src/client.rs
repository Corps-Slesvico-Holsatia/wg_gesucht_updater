use crate::args::{Action, Mode, Settings};
use crate::config_file::ConfigFile;
use crate::session::{TIMEOUT, USER_AGENT};
use crate::{Args, Session};
use serde_rw::FromFile;
use std::process::exit;
use std::time::Duration;

/// Source-agnostic client
///
/// The client's settings can be either parsed from the
/// command line arguments or from a configuration file.
pub struct Client {
    user_name: String,
    password: String,
    user_agent: String,
    timeout: Duration,
    activate: Vec<u32>,
    bump: Vec<u32>,
    deactivate: Vec<u32>,
}

impl Client {
    /// Run the client as per its settings
    ///
    /// This function will exit the program with an appropriate
    /// exit code when all operations are done or errors occurred.
    pub async fn run(&self) {
        match Session::new(self.timeout, &self.user_agent) {
            Ok(mut session) => self.run_with_session(&mut session).await,
            Err(error) => {
                eprintln!("{error}");
                exit(1);
            }
        }
    }

    async fn run_with_session(&self, session: &mut Session) {
        session
            .login(&self.user_name, &self.password)
            .await
            .unwrap_or_else(|error| {
                eprintln!("{error}");
                exit(2)
            });
        let mut exit_code = 0;
        self.deactivate_offers(session, &mut exit_code).await;
        self.activate_offers(session, &mut exit_code).await;
        self.bump_offers(session, &mut exit_code).await;
        exit(exit_code);
    }

    async fn activate_offers(&self, session: &mut Session, exit_code: &mut i32) {
        for &id in &self.activate {
            println!("Activating offer: {id}");
            session.activate(id).await.unwrap_or_else(|error| {
                eprintln!("Could not activate offer {id}: {error}");
                *exit_code += 1;
            });
        }
    }

    async fn bump_offers(&self, session: &mut Session, exit_code: &mut i32) {
        for &id in &self.bump {
            println!("Bumping offer: {id}");
            session.bump(id).await.unwrap_or_else(|error| {
                eprintln!("Could not bump offer {id}: {error}");
                *exit_code += 1;
            });
        }
    }

    async fn deactivate_offers(&self, session: &mut Session, exit_code: &mut i32) {
        for &id in &self.deactivate {
            println!("Deactivating offer: {id}");
            session.deactivate(id).await.unwrap_or_else(|error| {
                eprintln!("Could not deactivate offer {id}: {error}");
                *exit_code += 1;
            });
        }
    }
}

impl From<ConfigFile> for Client {
    fn from(config: ConfigFile) -> Self {
        Self {
            user_name: config.user_name,
            password: config.password,
            user_agent: config.user_agent.unwrap_or_else(|| USER_AGENT.to_string()),
            timeout: config.timeout_sec.map_or(TIMEOUT, Duration::from_secs),
            activate: config.activate.unwrap_or_default(),
            bump: config.bump.unwrap_or_default(),
            deactivate: config.deactivate.unwrap_or_default(),
        }
    }
}

impl From<Settings> for Client {
    fn from(settings: Settings) -> Self {
        Self {
            user_name: settings.user_name,
            password: settings.password,
            user_agent: settings.user_agent,
            timeout: Duration::from_secs(settings.timeout),
            activate: if let Action::Activate { offers } = &settings.action {
                offers.clone()
            } else {
                Vec::new()
            },
            bump: if let Action::Bump { offers } = &settings.action {
                offers.clone()
            } else {
                Vec::new()
            },
            deactivate: if let Action::Deactivate { offers } = &settings.action {
                offers.clone()
            } else {
                Vec::new()
            },
        }
    }
}

impl TryFrom<Args> for Client {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        match args.mode {
            Mode::Cli(settings) => Ok(settings.into()),
            Mode::ConfigFile { config_file } => ConfigFile::from_file(config_file).map(Into::into),
        }
    }
}
