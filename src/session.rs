use crate::auth_data::AuthData;
use crate::login_data::LoginData;
use crate::patch_data::PatchData;
use reqwest::{Client, Error, Request, Response, StatusCode};
use scraper::{Html, Selector};
use std::collections::HashMap;

const LOGIN_URL: &str = "https://www.wg-gesucht.de/ajax/sessions.php?action=login";
const OFFERS_LIST_URL: &str = "https://www.wg-gesucht.de/meine-anzeigen.html";
const OFFER_MODIFY_URL: &str = "https://www.wg-gesucht.de/api/offers";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Session {
    user_name: String,
    password: String,
    client: Client,
    auth_data: Option<AuthData>,
}

impl Session {
    /// Create a new session to the "WG gesucht" API
    ///
    /// # Errors
    /// Returns an `[reqwest::Error]` if the session client could not be constructed
    pub fn new(user_name: &str, password: &str) -> Result<Self, Error> {
        Client::builder()
            .cookie_store(true)
            .build()
            .map(|client| Self {
                client,
                user_name: user_name.to_string(),
                password: password.to_string(),
                auth_data: None,
            })
    }

    /// Initiate API session
    ///
    /// # Errors
    /// Returns an `[anyhow::Error]` on request errors
    pub async fn login(&mut self) -> anyhow::Result<()> {
        self.auth_data = Some(self.get_auth_data().await?);
        Ok(())
    }

    /// Update a single advertisement
    ///
    /// # Errors
    /// Returns an `[anyhow::Error]` on request errors
    pub async fn update(&mut self, ad_id: u32) -> anyhow::Result<()> {
        if let Some(ref auth_data) = self.auth_data {
            if self
                .client
                .execute(self.make_patch_request(ad_id, auth_data, true)?)
                .await?
                .status()
                != StatusCode::from_u16(200)?
            {
                return Err(anyhow::Error::msg("Could not deactivate ad"));
            }

            if self
                .client
                .execute(self.make_patch_request(ad_id, auth_data, false)?)
                .await?
                .status()
                != StatusCode::from_u16(200)?
            {
                return Err(anyhow::Error::msg("Could not reactivate ad"));
            }

            Ok(())
        } else {
            Err(anyhow::Error::msg("Not logged in"))
        }
    }

    async fn get_auth_data(&mut self) -> anyhow::Result<AuthData> {
        let (dev_ref, access_token) = self.get_dev_ref_and_access_token().await?;
        let (csrf_token, user_id) = self.get_csrf_token_and_user_id().await?;
        Ok(AuthData::new(
            user_id,
            "wg_desktop_website".to_string(),
            access_token,
            dev_ref,
            csrf_token,
        ))
    }
    async fn get_dev_ref_and_access_token(&mut self) -> anyhow::Result<(String, String)> {
        let response = self.execute_login_request().await?;
        let cookies: HashMap<_, _> = response
            .cookies()
            .map(|cookie| (cookie.name().to_string(), cookie.value().to_string()))
            .collect();
        let dev_ref = cookies
            .get("X-Dev-Ref-No")
            .ok_or_else(|| anyhow::Error::msg("X-Dev-Ref-No not found in cookies"))?;
        let access_token = cookies
            .get("X-Access-Token")
            .ok_or_else(|| anyhow::Error::msg("X-Access-Token not found in cookies"))?;
        Ok((dev_ref.to_string(), access_token.to_string()))
    }

    async fn get_csrf_token_and_user_id(&mut self) -> anyhow::Result<(String, String)> {
        let html = Html::parse_document(&String::from_utf8(
            self.client
                .execute(
                    self.client
                        .get(OFFERS_LIST_URL)
                        .header("User-Agent", USER_AGENT)
                        .build()?,
                )
                .await?
                .bytes()
                .await?
                .to_vec(),
        )?);
        let csrf_token = html
            .select(
                &Selector::parse("a[data-csrf_token]")
                    .map_err(|_| anyhow::Error::msg("Cannot build CSRF token selector"))?,
            )
            .next()
            .ok_or_else(|| anyhow::Error::msg("Could not find element with CSRF token"))?
            .value()
            .attr("data-csrf_token")
            .ok_or_else(|| anyhow::Error::msg("Could extract not CSRF token from element"))?;
        let user_id = html
            .select(
                &Selector::parse("a[data-user_id]")
                    .map_err(|_| anyhow::Error::msg("Cannot build user ID selector"))?,
            )
            .next()
            .ok_or_else(|| anyhow::Error::msg("Could not find element with user ID token"))?
            .value()
            .attr("data-user_id")
            .ok_or_else(|| anyhow::Error::msg("Could not extract user ID token from element"))?;
        Ok((csrf_token.to_string(), user_id.to_string()))
    }

    async fn execute_login_request(&mut self) -> Result<Response, Error> {
        self.client
            .execute(
                self.client
                    .post(LOGIN_URL)
                    .json(&LoginData::new(
                        self.user_name.as_str(),
                        self.password.as_str(),
                        true,
                        "de",
                    ))
                    .header("User-Agent", USER_AGENT)
                    .build()?,
            )
            .await
    }

    fn make_patch_request(
        &self,
        ad_id: u32,
        auth_data: &AuthData,
        deactivated: bool,
    ) -> anyhow::Result<Request> {
        Ok(self
            .client
            .patch(format!(
                "{}/{}/users/{}",
                OFFER_MODIFY_URL,
                ad_id,
                auth_data.user_id()
            ))
            .headers(auth_data.try_into()?)
            .header("User-Agent", USER_AGENT)
            .json(&PatchData::new(deactivated, auth_data.csrf_token()))
            .build()?)
    }
}
