use std::borrow::Cow;
use std::time::Duration;

use log::debug;
use reqwest::{Client, Request, Response, Url};

use crate::auth_data::AuthData;
use crate::patch_data::PatchData;

const OFFER_MODIFY_URL: &str = "https://www.wg-gesucht.de/api/offers";

/// Session with the wg-gesucht web API
#[derive(Debug)]
pub struct Session {
    client: Client,
    auth_data: AuthData,
    timeout: Duration,
    user_agent: String,
}

impl Session {
    /// Create a new session to the "WG gesucht" API.
    ///
    /// # Attributes
    /// * `timeout` - The HTTP request timeout.
    /// * `user_agent` - The HTTP user agent to send with the requests.
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] if the session client could not be constructed.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub const fn new(
        client: Client,
        auth_data: AuthData,
        timeout: Duration,
        user_agent: String,
    ) -> Self {
        Self {
            client,
            auth_data,
            timeout,
            user_agent,
        }
    }

    /// Bump an offer.
    ///
    /// This is equivalent of deactivating and then re-activating an offer.
    ///
    /// # Attributes
    /// * `id` - The offer ID (also referred to as "ad id").
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on request errors.
    pub async fn bump(&self, id: u32) -> anyhow::Result<Response> {
        self.deactivate(id).await?;
        self.activate(id).await
    }

    /// Deactivate an offer
    ///
    /// # Attributes
    /// * `id` - The offer ID (also referred to as "ad id").
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on request errors.
    pub async fn deactivate(&self, id: u32) -> anyhow::Result<Response> {
        Ok(self
            .client
            .execute(self.build_patch_request(id, true)?)
            .await?
            .error_for_status()?)
    }

    /// Activate an offer
    ///
    /// # Attributes
    /// * `id` - The offer ID (also referred to as "ad id").
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on request errors.
    pub async fn activate(&self, id: u32) -> anyhow::Result<Response> {
        Ok(self
            .client
            .execute(self.build_patch_request(id, false)?)
            .await?
            .error_for_status()?)
    }

    fn build_patch_request(&self, id: u32, deactivated: bool) -> anyhow::Result<Request> {
        Ok(self
            .client
            .patch(build_patch_url(id, self.auth_data.user_id()))
            .headers((&self.auth_data).try_into()?)
            .header("User-Agent", &self.user_agent)
            .json(&PatchData::new(deactivated, self.auth_data.csrf_token()))
            .timeout(self.timeout)
            .build()?)
    }
}

fn build_patch_url(offer_id: u32, user_id: &str) -> Url {
    let mut url = Url::parse(OFFER_MODIFY_URL).expect("Default URL should be valid.");
    url.path_segments_mut()
        .expect("Path segments should be accessible.")
        .push(&offer_id.to_string())
        .push("users")
        .push(user_id);
    debug!("Patch URL: {url}");
    url
}
