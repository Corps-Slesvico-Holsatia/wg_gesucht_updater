use serde::Deserialize;

/// Per-account settings.
#[derive(Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct Account {
    pub(crate) user_name: String,
    pub(crate) password: String,
    pub(crate) user_agent: Option<String>,
    pub(crate) timeout_sec: Option<u64>,
    #[serde(default)]
    pub(crate) activate: Vec<u32>,
    #[serde(default)]
    pub(crate) bump: Vec<u32>,
    #[serde(default)]
    pub(crate) deactivate: Vec<u32>,
}
