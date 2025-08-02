use std::borrow::Cow;

use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AuthData {
    user_id: Cow<'static, str>,
    client_id: Cow<'static, str>,
    access_token: Cow<'static, str>,
    dev_ref: Cow<'static, str>,
    csrf_token: Cow<'static, str>,
}

impl AuthData {
    pub const fn new(
        user_id: Cow<'static, str>,
        client_id: Cow<'static, str>,
        access_token: Cow<'static, str>,
        dev_ref: Cow<'static, str>,
        csrf_token: Cow<'static, str>,
    ) -> Self {
        Self {
            user_id,
            client_id,
            access_token,
            dev_ref,
            csrf_token,
        }
    }

    pub fn user_id(&self) -> &str {
        self.user_id.as_ref()
    }

    pub fn csrf_token(&self) -> &str {
        self.csrf_token.as_ref()
    }
}

impl TryFrom<&AuthData> for HeaderMap {
    type Error = InvalidHeaderValue;

    fn try_from(auth_data: &AuthData) -> Result<Self, Self::Error> {
        let mut map = Self::new();
        map.append(
            "X-User-ID",
            HeaderValue::try_from(auth_data.user_id.as_ref())?,
        );
        map.append(
            "X-Client-ID",
            HeaderValue::try_from(auth_data.client_id.as_ref())?,
        );
        map.append(
            "X-Authorization",
            HeaderValue::try_from(format!("Bearer {}", auth_data.access_token))?,
        );
        map.append(
            "X-Dev-Ref-No",
            HeaderValue::try_from(auth_data.dev_ref.as_ref())?,
        );
        Ok(map)
    }
}
