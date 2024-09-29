use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// Errors that can occur during API calls.
#[derive(Debug)]
pub enum Error {
    /// An error occurred during login.
    Login(anyhow::Error),
    /// Some offers failed to update.
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

/// Details about failed updates.
#[derive(Debug, Default)]
pub struct FailedUpdates {
    pub(crate) activate: HashMap<u32, anyhow::Error>,
    pub(crate) deactivate: HashMap<u32, anyhow::Error>,
    pub(crate) bump: HashMap<u32, anyhow::Error>,
}

impl FailedUpdates {
    /// Returns a map of offer IDs that failed to activate
    /// alongside the respective error that occurred.
    #[must_use]
    pub const fn activate(&self) -> &HashMap<u32, anyhow::Error> {
        &self.activate
    }

    /// Returns a map of offer IDs that failed to deactivate
    /// alongside the respective error that occurred.
    #[must_use]
    pub const fn deactivate(&self) -> &HashMap<u32, anyhow::Error> {
        &self.deactivate
    }

    /// Returns a map of offer IDs that failed to bump
    /// alongside the respective error that occurred.
    #[must_use]
    pub const fn bump(&self) -> &HashMap<u32, anyhow::Error> {
        &self.bump
    }

    /// Returns `true` iff there are no errors.
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
