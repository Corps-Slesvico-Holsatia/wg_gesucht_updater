use crate::functions::bool_to_int_str;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PatchData {
    #[serde(serialize_with = "bool_to_int_str")]
    deactivated: bool,
    csrf_token: String,
}

impl PatchData {
    pub fn new(deactivated: bool, csrf_token: &str) -> Self {
        Self {
            deactivated,
            csrf_token: csrf_token.to_string(),
        }
    }
}
