use crate::args::{Action, Mode, Parameters};
use crate::config_file::ConfigFile;
use crate::session::{TIMEOUT, USER_AGENT};
use crate::Args;
use serde_rw::FromFile;
use std::time::Duration;

/// Source-agnostic settings
///
/// The settings can be either parsed from the
/// command line arguments or from a configuration file.
pub struct Settings {
    pub(crate) user_name: String,
    pub(crate) password: String,
    pub(crate) user_agent: String,
    pub(crate) timeout: Duration,
    pub(crate) activate: Vec<u32>,
    pub(crate) bump: Vec<u32>,
    pub(crate) deactivate: Vec<u32>,
}

impl From<ConfigFile> for Settings {
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

impl From<Parameters> for Settings {
    fn from(settings: Parameters) -> Self {
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

impl TryFrom<Args> for Settings {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        match args.mode {
            Mode::Cli(settings) => Ok(settings.into()),
            Mode::ConfigFile { config_file } => ConfigFile::from_file(config_file).map(Into::into),
        }
    }
}
