use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::Duration;

use anyhow::anyhow;
use reqwest::{Request, Response};
use scraper::{Html, Selector};

use crate::auth_data::AuthData;
use crate::login_data::LoginData;
use crate::Session;

pub mod session;

const LOGIN_URL: &str = "https://www.wg-gesucht.de/ajax/sessions.php?action=login";
const OFFERS_LIST_URL: &str = "https://www.wg-gesucht.de/meine-anzeigen.html";
const CLIENT_ID: &str = "wg_desktop_website";
pub const TIMEOUT: Duration = Duration::from_secs(10);
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";
static CSRF_TOKEN_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("a[data-csrf_token]").expect("Could not create CSRF token selector")
});
static USER_ID_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("a[data-user_id]").expect("Could not create user ID selector")
});

/// Client to the wg-gesucht web API.
#[derive(Debug)]
pub struct Client {
    timeout: Duration,
    user_agent: String,
    #[allow(clippy::struct_field_names)]
    client: reqwest::Client,
}

impl Client {
    /// Create a new client to the "WG gesucht" API.
    ///
    /// # Attributes
    /// * `timeout` - The HTTP request timeout.
    /// * `user_agent` - The HTTP user agent to send with the requests.
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] if the session client could not be constructed.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new(timeout: Duration, user_agent: &str) -> Self {
        Self {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .expect("Client builder should never fail."),
            timeout,
            user_agent: user_agent.to_string(),
        }
    }

    /// Initiate API session.
    ///
    /// A login must be performed as the first call to the API
    /// in order to use subsequent requests to modify offers.
    ///
    /// # Attributes
    /// * `user_name` - The username of the wg-gesucht.de account.
    /// * `password` - The password associated with above username.
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on request errors
    pub async fn login(self, user_name: &str, password: &str) -> anyhow::Result<Session> {
        self.get_auth_data(user_name, password)
            .await
            .map(|auth_data| {
                Session::new(
                    self.client,
                    auth_data,
                    self.timeout,
                    Cow::Owned(self.user_agent),
                )
            })
    }

    async fn get_dev_ref_and_access_token(
        &self,
        user_name: &str,
        password: &str,
    ) -> anyhow::Result<(String, String)> {
        scrape_dev_ref_and_access_token(
            &self
                .execute_login_request(user_name, password)
                .await?
                .cookies()
                .map(|cookie| (cookie.name().to_string(), cookie.value().to_string()))
                .collect::<HashMap<_, _>>(),
        )
        .map(|(dev_ref, access_token)| (dev_ref.to_string(), access_token.to_string()))
    }

    fn build_offer_list_request(&self) -> reqwest::Result<Request> {
        self.client
            .get(OFFERS_LIST_URL)
            .header("User-Agent", &self.user_agent)
            .timeout(self.timeout)
            .build()
    }

    async fn get_csrf_token_and_user_id(&self) -> anyhow::Result<(String, String)> {
        scrape_csrf_token_and_user_id(&Html::parse_document(&String::from_utf8(
            self.client
                .execute(self.build_offer_list_request()?)
                .await?
                .error_for_status()?
                .bytes()
                .await?
                .to_vec(),
        )?))
        .map(|(csrf_token, user_id)| (csrf_token.to_string(), user_id.to_string()))
    }

    async fn get_auth_data(&self, user_name: &str, password: &str) -> anyhow::Result<AuthData> {
        let (dev_ref, access_token) = self
            .get_dev_ref_and_access_token(user_name, password)
            .await?;
        let (csrf_token, user_id) = self.get_csrf_token_and_user_id().await?;
        Ok(AuthData::new(
            user_id,
            CLIENT_ID.to_string(),
            access_token,
            dev_ref,
            csrf_token,
        ))
    }

    fn build_login_request(&self, user_name: &str, password: &str) -> reqwest::Result<Request> {
        self.client
            .post(LOGIN_URL)
            .json(&LoginData::new(user_name, password, true, "de"))
            .header("User-Agent", &self.user_agent)
            .timeout(self.timeout)
            .build()
    }

    async fn execute_login_request(
        &self,
        user_name: &str,
        password: &str,
    ) -> reqwest::Result<Response> {
        self.client
            .execute(self.build_login_request(user_name, password)?)
            .await?
            .error_for_status()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new(TIMEOUT, USER_AGENT)
    }
}

fn scrape_dev_ref_and_access_token(
    cookies: &HashMap<String, String>,
) -> anyhow::Result<(&str, &str)> {
    Ok((
        cookies
            .get("X-Dev-Ref-No")
            .ok_or_else(|| anyhow!("X-Dev-Ref-No not found in cookies"))?,
        cookies
            .get("X-Access-Token")
            .ok_or_else(|| anyhow!("X-Access-Token not found in cookies"))?,
    ))
}

fn scrape_csrf_token_and_user_id(html: &Html) -> anyhow::Result<(&str, &str)> {
    Ok((
        html.select(&CSRF_TOKEN_SELECTOR)
            .next()
            .ok_or_else(|| anyhow!("Could not find element with CSRF token"))?
            .value()
            .attr("data-csrf_token")
            .ok_or_else(|| anyhow!("Could extract not CSRF token from element"))?,
        html.select(&USER_ID_SELECTOR)
            .next()
            .ok_or_else(|| anyhow!("Could not find element with user ID"))?
            .value()
            .attr("data-user_id")
            .ok_or_else(|| anyhow!("Could not extract user ID from element"))?,
    ))
}
