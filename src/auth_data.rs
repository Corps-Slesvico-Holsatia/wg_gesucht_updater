use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};

#[derive(Debug)]
pub struct AuthData {
    user_id: String,
    client_id: String,
    php_session_id: String,
    csrf_token: String,
}

impl AuthData {
    pub const fn new(
        user_id: String,
        client_id: String,
        php_session_id: String,
        csrf_token: String,
    ) -> Self {
        Self {
            user_id,
            client_id,
            php_session_id,
            csrf_token,
        }
    }

    pub fn user_id(&self) -> &str {
        self.user_id.as_str()
    }

    pub fn csrf_token(&self) -> &str {
        self.csrf_token.as_str()
    }
}

impl TryFrom<&AuthData> for HeaderMap {
    type Error = InvalidHeaderValue;

    fn try_from(auth_data: &AuthData) -> Result<Self, Self::Error> {
        let mut map = Self::new();
        map.append("X-User-ID", HeaderValue::try_from(&auth_data.user_id)?);
        map.append("X-Client-ID", HeaderValue::try_from(&auth_data.client_id)?);
        map.append(
            "Cookies",
            HeaderValue::try_from(format!("PHPSESSID={}", &auth_data.php_session_id))?,
        );
        Ok(map)
    }
}
