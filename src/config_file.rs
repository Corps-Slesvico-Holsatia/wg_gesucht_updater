use serde::Deserialize;

pub use account::Account;

mod account;

/// Configuration file content.
#[derive(Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ConfigFile {
    pub(crate) accounts: Vec<Account>,
}
