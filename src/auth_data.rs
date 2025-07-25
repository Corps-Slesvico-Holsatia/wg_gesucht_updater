use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AuthData {
    user_id: String,
    client_id: String,
    access_token: String,
    dev_ref: String,
    csrf_token: String,
}

impl AuthData {
    pub const fn new(
        user_id: String,
        client_id: String,
        access_token: String,
        dev_ref: String,
        csrf_token: String,
    ) -> Self {
        Self {
            user_id,
            client_id,
            access_token,
            dev_ref,
            csrf_token,
        }
    }

    pub const fn user_id(&self) -> &str {
        self.user_id.as_str()
    }

    pub const fn csrf_token(&self) -> &str {
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
            "X-Authorization",
            HeaderValue::try_from(format!("Bearer {}", &auth_data.access_token))?,
        );
        map.append("X-Dev-Ref-No", HeaderValue::try_from(&auth_data.dev_ref)?);
        Ok(map)
    }
}
