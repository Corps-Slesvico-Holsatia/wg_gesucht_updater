use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Login(anyhow::Error),
    Updates(FailedUpdates),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Login(error) => write!(f, "Login failed: {error}"),
            Self::Updates(updates) => Display::fmt(updates, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Login(_) => None,
            Self::Updates(error) => Some(error),
        }
    }
}

#[derive(Debug, Default)]
pub struct FailedUpdates {
    pub(crate) activate: HashMap<u32, anyhow::Error>,
    pub(crate) deactivate: HashMap<u32, anyhow::Error>,
    pub(crate) bump: HashMap<u32, anyhow::Error>,
}

impl FailedUpdates {
    #[must_use]
    pub const fn activate(&self) -> &HashMap<u32, anyhow::Error> {
        &self.activate
    }

    #[must_use]
    pub const fn deactivate(&self) -> &HashMap<u32, anyhow::Error> {
        &self.deactivate
    }

    #[must_use]
    pub const fn bump(&self) -> &HashMap<u32, anyhow::Error> {
        &self.bump
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.activate.is_empty() && self.deactivate.is_empty() && self.bump.is_empty()
    }
}

impl Display for FailedUpdates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (id, error) in &self.activate {
            write!(f, "Failed to activate #{id}: {error}")?;
        }

        for (id, error) in &self.deactivate {
            write!(f, "Failed to deactivate #{id}: {error}")?;
        }

        for (id, error) in &self.bump {
            write!(f, "Failed to bump #{id}: {error}")?;
        }

        Ok(())
    }
}

impl std::error::Error for FailedUpdates {}
