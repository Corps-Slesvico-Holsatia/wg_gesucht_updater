use crate::args::{Action, Mode, Settings};
use crate::config_file::ConfigFile;
use crate::session::{TIMEOUT, USER_AGENT};
use crate::Args;
use serde_rw::FromFile;
use std::time::Duration;

pub struct Configuration {
    pub user_name: String,
    pub password: String,
    pub user_agent: String,
    pub timeout: Duration,
    pub bump: Vec<u32>,
    pub activate: Vec<u32>,
    pub deactivate: Vec<u32>,
}

impl From<ConfigFile> for Configuration {
    fn from(config: ConfigFile) -> Self {
        Self {
            user_name: config.user_name,
            password: config.password,
            user_agent: config.user_agent.unwrap_or_else(|| USER_AGENT.to_string()),
            timeout: config.timeout.unwrap_or(TIMEOUT),
            bump: config.bump.unwrap_or_default(),
            activate: config.activate.unwrap_or_default(),
            deactivate: config.deactivate.unwrap_or_default(),
        }
    }
}

impl From<Settings> for Configuration {
    fn from(settings: Settings) -> Self {
        match settings.action {
            Action::Bump { ad_ids } => Self {
                user_name: settings.user_name,
                password: settings.password,
                user_agent: settings.user_agent,
                timeout: settings.timeout,
                bump: ad_ids,
                activate: Vec::new(),
                deactivate: Vec::new(),
            },
            Action::Activate { ad_ids } => Self {
                user_name: settings.user_name,
                password: settings.password,
                user_agent: settings.user_agent,
                timeout: settings.timeout,
                bump: Vec::new(),
                activate: ad_ids,
                deactivate: Vec::new(),
            },
            Action::Deactivate { ad_ids } => Self {
                user_name: settings.user_name,
                password: settings.password,
                user_agent: settings.user_agent,
                timeout: settings.timeout,
                bump: Vec::new(),
                activate: Vec::new(),
                deactivate: ad_ids,
            },
        }
    }
}

impl TryFrom<Args> for Configuration {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        match args.mode {
            Mode::Cli(settings) => Ok(settings.into()),
            Mode::ConfigFile { config_file } => ConfigFile::from_file(config_file).map(Into::into),
        }
    }
}
