use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) user_name: String,
    pub(crate) password: String,
    pub(crate) user_agent: Option<String>,
    pub(crate) ad_ids: Vec<u32>,
}
