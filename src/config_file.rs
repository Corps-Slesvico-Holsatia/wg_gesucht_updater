use serde::Deserialize;
use std::time::Duration;

/// Configuration file content
#[derive(Deserialize)]
pub struct ConfigFile {
    pub(crate) user_name: String,
    pub(crate) password: String,
    pub(crate) user_agent: Option<String>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) activate: Option<Vec<u32>>,
    pub(crate) bump: Option<Vec<u32>>,
    pub(crate) deactivate: Option<Vec<u32>>,
}
