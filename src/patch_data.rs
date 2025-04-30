use serde::Serialize;

use crate::functions::bool_to_int_str;

#[derive(Debug, Serialize)]
pub struct PatchData<'token> {
    #[serde(serialize_with = "bool_to_int_str")]
    deactivated: bool,
    csrf_token: &'token str,
}

impl<'token> PatchData<'token> {
    pub const fn new(deactivated: bool, csrf_token: &'token str) -> Self {
        Self {
            deactivated,
            csrf_token,
        }
    }
}
