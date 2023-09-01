use crate::settings::Settings;
use crate::{Args, Session};

/// Client to operate on offers at wg-gesucht.de
///
/// The client's settings can be either parsed from the
/// command line arguments or from a configuration file.
pub struct Client {
    settings: Settings,
    session: Session,
}

impl Client {
    /// Create a new client from the given settings
    ///
    /// Upon creation, create a new session to use for API requests.
    ///
    /// # Errors
    /// Returns an `[anyhow::Error]` if a session could not be created.
    pub fn new(settings: Settings) -> anyhow::Result<Self> {
        Ok(Self {
            session: Session::new(settings.timeout, &settings.user_agent)?,
            settings,
        })
    }

    /// Run the client as per its settings
    ///
    /// This function will exit the program with an appropriate
    /// exit code when all operations are done or errors occurred.
    pub async fn run(&mut self) -> i32 {
        if let Err(error) = self
            .session
            .login(&self.settings.user_name, &self.settings.password)
            .await
        {
            eprintln!("{error}");
            return 2;
        }

        let mut exit_code = 0;
        self.deactivate_offers(&mut exit_code).await;
        self.activate_offers(&mut exit_code).await;
        self.bump_offers(&mut exit_code).await;
        exit_code
    }

    async fn activate_offers(&mut self, exit_code: &mut i32) {
        for &id in &self.settings.activate {
            println!("Activating offer: {id}");
            self.session.activate(id).await.unwrap_or_else(|error| {
                eprintln!("Could not activate offer {id}: {error}");
                *exit_code += 1;
            });
        }
    }

    async fn bump_offers(&mut self, exit_code: &mut i32) {
        for &id in &self.settings.bump {
            println!("Bumping offer: {id}");
            self.session.bump(id).await.unwrap_or_else(|error| {
                eprintln!("Could not bump offer {id}: {error}");
                *exit_code += 1;
            });
        }
    }

    async fn deactivate_offers(&mut self, exit_code: &mut i32) {
        for &id in &self.settings.deactivate {
            println!("Deactivating offer: {id}");
            self.session.deactivate(id).await.unwrap_or_else(|error| {
                eprintln!("Could not deactivate offer {id}: {error}");
                *exit_code += 1;
            });
        }
    }
}

impl TryFrom<Settings> for Client {
    type Error = anyhow::Error;

    fn try_from(settings: Settings) -> Result<Self, Self::Error> {
        Self::new(settings)
    }
}

impl TryFrom<Args> for Client {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        args.try_into().and_then(Self::new)
    }
}
