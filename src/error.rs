use std::fmt::{Display, Formatter};

pub use failed_updates::FailedUpdates;

mod failed_updates;

/// Errors that can occur during API calls.
#[derive(Debug)]
pub enum Error {
    /// An error occurred during login.
    Login(anyhow::Error),
    /// Some offers failed to update.
    Updates(Box<FailedUpdates>),
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

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self::Login(error)
    }
}

impl From<FailedUpdates> for Error {
    fn from(failed_updates: FailedUpdates) -> Self {
        Self::Updates(Box::new(failed_updates))
    }
}
