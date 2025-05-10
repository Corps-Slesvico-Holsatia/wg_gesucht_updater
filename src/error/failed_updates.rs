use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

/// Details about failed updates.
#[derive(Debug, Default)]
pub struct FailedUpdates {
    pub(crate) activate: BTreeMap<u32, anyhow::Error>,
    pub(crate) deactivate: BTreeMap<u32, anyhow::Error>,
    pub(crate) bump: BTreeMap<u32, anyhow::Error>,
}

impl FailedUpdates {
    /// Return `true` iff there are no errors.
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
