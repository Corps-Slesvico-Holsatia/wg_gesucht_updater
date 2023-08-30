use crate::functions::bool_to_int_str;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginData {
    #[serde(rename = "login_email_username")]
    user_name: String,
    #[serde(rename = "login_password")]
    password: String,
    #[serde(rename = "login_form_autologin", serialize_with = "bool_to_int_str")]
    autologin: bool,
    #[serde(rename = "display_language")]
    language: String,
}

impl LoginData {
    pub fn new(user_name: &str, password: &str, autologin: bool, language: &str) -> Self {
        Self {
            user_name: user_name.to_string(),
            password: password.to_string(),
            autologin,
            language: language.to_string(),
        }
    }
}

impl From<(&str, &str)> for LoginData {
    fn from((user_name, password): (&str, &str)) -> Self {
        Self::new(user_name, password, true, "de")
    }
}
