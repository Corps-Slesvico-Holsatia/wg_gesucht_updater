use serde::Serialize;

use crate::functions::bool_to_int_str;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct LoginData<'creds> {
    #[serde(rename = "login_email_username")]
    user_name: &'creds str,
    #[serde(rename = "login_password")]
    password: &'creds str,
    #[serde(rename = "login_form_autologin", serialize_with = "bool_to_int_str")]
    autologin: bool,
    #[serde(rename = "display_language")]
    language: &'creds str,
}

impl<'creds> LoginData<'creds> {
    pub const fn new(
        user_name: &'creds str,
        password: &'creds str,
        autologin: bool,
        language: &'creds str,
    ) -> Self {
        Self {
            user_name,
            password,
            autologin,
            language,
        }
    }
}
