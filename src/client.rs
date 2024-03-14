use crate::settings::Settings;
use crate::{Args, Session};
use log::{error, info};

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
    /// Returns an [`anyhow::Error`] if a session could not be created.
    #[must_use]
    pub fn new(settings: Settings) -> Self {
        Self {
            session: Session::new(settings.timeout, &settings.user_agent),
            settings,
        }
    }

    /// Run the client as per its settings
    ///
    /// # Errors
    /// Returns an [`Vec<anyhow::Error>`] containing any errors that occurred.
    pub async fn run(&mut self) -> Result<(), Vec<anyhow::Error>> {
        if let Err(error) = self
            .session
            .login(&self.settings.user_name, &self.settings.password)
            .await
        {
            error!("{error}");
            return Err(vec![error]);
        }

        let mut errors = Vec::new();
        self.deactivate_offers(&mut errors).await;
        self.activate_offers(&mut errors).await;
        self.bump_offers(&mut errors).await;

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    async fn activate_offers(&mut self, errors: &mut Vec<anyhow::Error>) {
        for &id in &self.settings.activate {
            info!("Activating offer: {id}");

            if let Err(error) = self.session.activate(id).await {
                error!("Could not activate offer {id}: {error}");
                errors.push(error);
            };
        }
    }

    async fn bump_offers(&mut self, errors: &mut Vec<anyhow::Error>) {
        for &id in &self.settings.bump {
            info!("Bumping offer: {id}");

            if let Err(error) = self.session.bump(id).await {
                error!("Could not bump offer {id}: {error}");
                errors.push(error);
            };
        }
    }

    async fn deactivate_offers(&mut self, errors: &mut Vec<anyhow::Error>) {
        for &id in &self.settings.deactivate {
            info!("Deactivating offer: {id}");

            if let Err(error) = self.session.deactivate(id).await {
                error!("Could not deactivate offer {id}: {error}");
                errors.push(error);
            };
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
