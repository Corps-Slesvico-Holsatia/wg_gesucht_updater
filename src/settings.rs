use crate::args::{Action, Mode, Parameters};
use crate::config_file::ConfigFile;
use crate::session::{TIMEOUT, USER_AGENT};
use crate::{Args, Session};
use log::{error, info};
use serde_rw::FromFile;
use std::time::Duration;

/// Source-agnostic settings
///
/// The settings can be either parsed from the
/// command line arguments or from a configuration file.
#[derive(Debug)]
pub struct Settings {
    pub(crate) user_name: String,
    pub(crate) password: String,
    pub(crate) user_agent: String,
    pub(crate) timeout: Duration,
    pub(crate) activate: Option<Vec<u32>>,
    pub(crate) bump: Option<Vec<u32>>,
    pub(crate) deactivate: Option<Vec<u32>>,
}

impl Settings {
    /// Apply the settings.
    ///
    /// # Errors
    /// Returns an [`Vec<anyhow::Error>`] containing any errors that occurred.
    pub async fn apply(&self) -> Result<(), Vec<anyhow::Error>> {
        let mut session = Session::new(self.timeout, &self.user_agent);

        if let Err(error) = session.login(&self.user_name, &self.password).await {
            error!("Login failed: {error}");
            return Err(vec![error]);
        }

        let mut errors = Vec::new();

        if let Some(ref offers) = self.deactivate {
            for &id in offers {
                info!("Deactivating offer: {id}");

                if let Err(error) = session.deactivate(id).await {
                    error!("Could not deactivate offer {id}: {error}");
                    errors.push(error);
                };
            }
        }

        if let Some(ref offers) = self.activate {
            for &id in offers {
                info!("Activating offer: {id}");

                if let Err(error) = session.activate(id).await {
                    error!("Could not activate offer {id}: {error}");
                    errors.push(error);
                };
            }
        }

        if let Some(ref offers) = self.bump {
            for &id in offers {
                info!("Bumping offer: {id}");

                if let Err(error) = session.bump(id).await {
                    error!("Could not bump offer {id}: {error}");
                    errors.push(error);
                };
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl From<ConfigFile> for Settings {
    fn from(config: ConfigFile) -> Self {
        Self {
            user_name: config.user_name,
            password: config.password,
            user_agent: config.user_agent.unwrap_or_else(|| USER_AGENT.to_string()),
            timeout: config.timeout_sec.map_or(TIMEOUT, Duration::from_secs),
            activate: config.activate,
            bump: config.bump,
            deactivate: config.deactivate,
        }
    }
}

impl From<Parameters> for Settings {
    fn from(settings: Parameters) -> Self {
        match settings.action {
            Action::Activate { offers } => Self {
                user_name: settings.user_name,
                password: settings.password,
                user_agent: settings.user_agent,
                timeout: Duration::from_secs(settings.timeout),
                activate: Some(offers),
                bump: None,
                deactivate: None,
            },
            Action::Bump { offers } => Self {
                user_name: settings.user_name,
                password: settings.password,
                user_agent: settings.user_agent,
                timeout: Duration::from_secs(settings.timeout),
                activate: None,
                bump: Some(offers),
                deactivate: None,
            },
            Action::Deactivate { offers } => Self {
                user_name: settings.user_name,
                password: settings.password,
                user_agent: settings.user_agent,
                timeout: Duration::from_secs(settings.timeout),
                activate: None,
                bump: None,
                deactivate: Some(offers),
            },
        }
    }
}

impl TryFrom<Args> for Settings {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        match args.mode {
            Mode::Cli(settings) => Ok(settings.into()),
            Mode::ConfigFile { config_file } => ConfigFile::from_file(config_file).map(Into::into),
        }
    }
}
