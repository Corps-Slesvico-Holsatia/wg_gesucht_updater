use std::process::exit;

use crate::settings::Settings;
use crate::{Args, Session};

/// Source-agnostic client
///
/// The client's settings can be either parsed from the
/// command line arguments or from a configuration file.
pub struct Client {
    settings: Settings,
}

impl Client {
    /// Create a new client from the given settings
    #[must_use]
    pub const fn new(settings: Settings) -> Self {
        Self { settings }
    }

    /// Run the client as per its settings
    ///
    /// This function will exit the program with an appropriate
    /// exit code when all operations are done or errors occurred.
    pub async fn run(&self) {
        match Session::new(self.settings.timeout, &self.settings.user_agent) {
            Ok(mut session) => self.run_with_session(&mut session).await,
            Err(error) => {
                eprintln!("{error}");
                exit(1);
            }
        }
    }

    async fn run_with_session(&self, session: &mut Session) {
        session
            .login(&self.settings.user_name, &self.settings.password)
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
        for &id in &self.settings.activate {
            println!("Activating offer: {id}");
            session.activate(id).await.unwrap_or_else(|error| {
                eprintln!("Could not activate offer {id}: {error}");
                *exit_code += 1;
            });
        }
    }

    async fn bump_offers(&self, session: &mut Session, exit_code: &mut i32) {
        for &id in &self.settings.bump {
            println!("Bumping offer: {id}");
            session.bump(id).await.unwrap_or_else(|error| {
                eprintln!("Could not bump offer {id}: {error}");
                *exit_code += 1;
            });
        }
    }

    async fn deactivate_offers(&self, session: &mut Session, exit_code: &mut i32) {
        for &id in &self.settings.deactivate {
            println!("Deactivating offer: {id}");
            session.deactivate(id).await.unwrap_or_else(|error| {
                eprintln!("Could not deactivate offer {id}: {error}");
                *exit_code += 1;
            });
        }
    }
}

impl From<Settings> for Client {
    fn from(settings: Settings) -> Self {
        Self::new(settings)
    }
}

impl TryFrom<Args> for Client {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        args.try_into().map(Self::new)
    }
}
