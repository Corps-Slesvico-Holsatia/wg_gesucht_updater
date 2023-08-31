use crate::auth_data::AuthData;
use crate::login_data::LoginData;
use crate::patch_data::PatchData;
use anyhow::anyhow;
use reqwest::{Client, Error, Request, Response, StatusCode};
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::time::Duration;

const LOGIN_URL: &str = "https://www.wg-gesucht.de/ajax/sessions.php?action=login";
const OFFERS_LIST_URL: &str = "https://www.wg-gesucht.de/meine-anzeigen.html";
const OFFER_MODIFY_URL: &str = "https://www.wg-gesucht.de/api/offers";
pub const TIMEOUT: Duration = Duration::from_secs(10);
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Session {
    user_agent: String,
    timeout: Duration,
    client: Client,
    auth_data: Option<AuthData>,
}

impl Session {
    /// Create a new session to the "WG gesucht" API
    ///
    /// # Errors
    /// Returns an `[reqwest::Error]` if the session client could not be constructed
    pub fn new(user_agent: &str, timeout: Duration) -> Result<Self, Error> {
        Client::builder()
            .cookie_store(true)
            .build()
            .map(|client| Self {
                client,
                auth_data: None,
                user_agent: user_agent.to_string(),
                timeout,
            })
    }

    /// Initiate API session
    ///
    /// # Errors
    /// Returns an `[anyhow::Error]` on request errors
    pub async fn login(&mut self, user_name: &str, password: &str) -> anyhow::Result<()> {
        self.get_auth_data(user_name, password)
            .await
            .map(|auth_data| self.auth_data = Some(auth_data))
    }

    /// Bump an advertisement
    ///
    /// # Errors
    /// Returns an `[anyhow::Error]` on request errors
    pub async fn bump(&mut self, ad_id: u32) -> anyhow::Result<()> {
        if self
            .client
            .execute(self.make_patch_request(ad_id, true)?)
            .await?
            .status()
            != StatusCode::from_u16(200)?
        {
            return Err(anyhow!("Could not deactivate ad"));
        }

        if self
            .client
            .execute(self.make_patch_request(ad_id, false)?)
            .await?
            .status()
            != StatusCode::from_u16(200)?
        {
            return Err(anyhow!("Could not reactivate ad"));
        }

        Ok(())
    }

    async fn get_auth_data(&mut self, user_name: &str, password: &str) -> anyhow::Result<AuthData> {
        let (dev_ref, access_token) = self
            .get_dev_ref_and_access_token(user_name, password)
            .await?;
        let (csrf_token, user_id) = self.get_csrf_token_and_user_id().await?;
        Ok(AuthData::new(
            user_id,
            "wg_desktop_website".to_string(),
            access_token,
            dev_ref,
            csrf_token,
        ))
    }

    async fn get_dev_ref_and_access_token(
        &mut self,
        user_name: &str,
        password: &str,
    ) -> anyhow::Result<(String, String)> {
        parse_dev_ref_and_access_token(
            &self
                .execute_login_request(user_name, password)
                .await?
                .cookies()
                .map(|cookie| (cookie.name().to_string(), cookie.value().to_string()))
                .collect::<HashMap<_, _>>(),
        )
    }

    async fn get_csrf_token_and_user_id(&mut self) -> anyhow::Result<(String, String)> {
        scrape_csrf_token_and_user_id(&Html::parse_document(&String::from_utf8(
            self.client
                .execute(
                    self.client
                        .get(OFFERS_LIST_URL)
                        .header("User-Agent", &self.user_agent)
                        .timeout(self.timeout)
                        .build()?,
                )
                .await?
                .bytes()
                .await?
                .to_vec(),
        )?))
    }

    async fn execute_login_request(
        &mut self,
        user_name: &str,
        password: &str,
    ) -> Result<Response, Error> {
        self.client
            .execute(self.make_login_request(user_name, password)?)
            .await
    }

    fn make_login_request(&self, user_name: &str, password: &str) -> reqwest::Result<Request> {
        self.client
            .post(LOGIN_URL)
            .json(&LoginData::new(user_name, password, true, "de"))
            .header("User-Agent", &self.user_agent)
            .timeout(self.timeout)
            .build()
    }

    fn make_patch_request(&self, ad_id: u32, deactivated: bool) -> anyhow::Result<Request> {
        if let Some(ref auth_data) = self.auth_data {
            Ok(self
                .client
                .patch(format!(
                    "{}/{}/users/{}",
                    OFFER_MODIFY_URL,
                    ad_id,
                    auth_data.user_id()
                ))
                .headers(auth_data.try_into()?)
                .header("User-Agent", &self.user_agent)
                .json(&PatchData::new(deactivated, auth_data.csrf_token()))
                .timeout(self.timeout)
                .build()?)
        } else {
            Err(anyhow!("Not logged in"))
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new(USER_AGENT, TIMEOUT).expect("Could not build client")
    }
}

fn parse_dev_ref_and_access_token(
    cookies: &HashMap<String, String>,
) -> anyhow::Result<(String, String)> {
    let dev_ref = cookies
        .get("X-Dev-Ref-No")
        .ok_or_else(|| anyhow!("X-Dev-Ref-No not found in cookies"))?;
    let access_token = cookies
        .get("X-Access-Token")
        .ok_or_else(|| anyhow!("X-Access-Token not found in cookies"))?;
    Ok((dev_ref.to_string(), access_token.to_string()))
}

fn scrape_csrf_token_and_user_id(html: &Html) -> anyhow::Result<(String, String)> {
    let csrf_token = html
        .select(
            &Selector::parse("a[data-csrf_token]")
                .map_err(|_| anyhow!("Cannot build CSRF token selector"))?,
        )
        .next()
        .ok_or_else(|| anyhow!("Could not find element with CSRF token"))?
        .value()
        .attr("data-csrf_token")
        .ok_or_else(|| anyhow!("Could extract not CSRF token from element"))?;
    let user_id = html
        .select(
            &Selector::parse("a[data-user_id]")
                .map_err(|_| anyhow!("Cannot build user ID selector"))?,
        )
        .next()
        .ok_or_else(|| anyhow!("Could not find element with user ID"))?
        .value()
        .attr("data-user_id")
        .ok_or_else(|| anyhow!("Could not extract user ID from element"))?;
    Ok((csrf_token.to_string(), user_id.to_string()))
}
