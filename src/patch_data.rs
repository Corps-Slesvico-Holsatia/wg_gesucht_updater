use crate::functions::bool_to_int_str;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PatchData<'a> {
    #[serde(serialize_with = "bool_to_int_str")]
    deactivated: bool,
    csrf_token: &'a str,
}

impl<'a> PatchData<'a> {
    pub const fn new(deactivated: bool, csrf_token: &'a str) -> Self {
        Self {
            deactivated,
            csrf_token,
        }
    }
}
