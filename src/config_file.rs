use serde::Deserialize;

/// Configuration file content.
#[derive(Deserialize)]
pub struct ConfigFile {
    pub(crate) user_name: String,
    pub(crate) password: String,
    pub(crate) user_agent: Option<String>,
    pub(crate) timeout_sec: Option<u64>,
    pub(crate) activate: Option<Vec<u32>>,
    pub(crate) bump: Option<Vec<u32>>,
    pub(crate) deactivate: Option<Vec<u32>>,
}
