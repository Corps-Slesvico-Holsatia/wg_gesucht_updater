use std::borrow::Cow;
use std::time::Duration;

use anyhow::anyhow;
use log::{error, info};
use serde_rw::FromFile;

use crate::args::{Action, Args, Mode, Parameters};
use crate::client::{Client, TIMEOUT, USER_AGENT};
use crate::config_file::{Account, ConfigFile};
use crate::error::{Error, FailedUpdates};

/// Source-agnostic settings
///
/// The settings can be either parsed from the
/// command line arguments or from a configuration file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Settings {
    user_name: String,
    password: String,
    user_agent: String,
    timeout: Duration,
    activate: Vec<u32>,
    bump: Vec<u32>,
    deactivate: Vec<u32>,
}

impl Settings {
    /// Apply the settings.
    ///
    /// # Errors
    /// Return an [`Vec<anyhow::Error>`] containing any errors that occurred.
    pub async fn apply(self) -> Result<(), Error> {
        let session = match Client::new(self.timeout, Cow::Owned(self.user_agent))
            .login(&self.user_name, &self.password)
            .await
        {
            Ok(session) => session,
            Err(error) => {
                error!("Login failed: {error}");
                return Err(Error::Login(error));
            }
        };

        let mut failed_updates = FailedUpdates::default();

        for &id in &self.deactivate {
            info!("Deactivating offer: {id}");

            if let Err(error) = session.deactivate(id).await {
                error!("Could not deactivate offer {id}: {error}");
                failed_updates.deactivate.insert(id, error);
            }
        }

        for &id in &self.activate {
            info!("Activating offer: {id}");

            if let Err(error) = session.activate(id).await {
                error!("Could not activate offer {id}: {error}");
                failed_updates.activate.insert(id, error);
            }
        }

        for &id in &self.bump {
            info!("Bumping offer: {id}");

            if let Err(error) = session.bump(id).await {
                error!("Could not bump offer {id}: {error}");
                failed_updates.bump.insert(id, error);
            }
        }

        if failed_updates.is_empty() {
            Ok(())
        } else {
            Err(failed_updates.into())
        }
    }
}

impl From<Account> for Settings {
    fn from(account: Account) -> Self {
        Self {
            user_name: account.user_name,
            password: account.password,
            user_agent: account.user_agent.unwrap_or_else(|| USER_AGENT.to_string()),
            timeout: account.timeout_sec.map_or(TIMEOUT, Duration::from_secs),
            activate: account.activate,
            bump: account.bump,
            deactivate: account.deactivate,
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
                activate: offers,
                bump: Vec::with_capacity(0),
                deactivate: Vec::with_capacity(0),
            },
            Action::Bump { offers } => Self {
                user_name: settings.user_name,
                password: settings.password,
                user_agent: settings.user_agent,
                timeout: Duration::from_secs(settings.timeout),
                activate: Vec::with_capacity(0),
                bump: offers,
                deactivate: Vec::with_capacity(0),
            },
            Action::Deactivate { offers } => Self {
                user_name: settings.user_name,
                password: settings.password,
                user_agent: settings.user_agent,
                timeout: Duration::from_secs(settings.timeout),
                activate: Vec::with_capacity(0),
                bump: Vec::with_capacity(0),
                deactivate: offers,
            },
        }
    }
}

impl TryFrom<Args> for Vec<Settings> {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        match args.mode {
            Mode::Cli(settings) => Ok(vec![settings.into()]),
            Mode::ConfigFile { config_file } => ConfigFile::from_file(config_file)
                .map_err(|error| anyhow!("{error}"))
                .map(|config_file| config_file.accounts.into_iter().map(Into::into).collect()),
        }
    }
}
