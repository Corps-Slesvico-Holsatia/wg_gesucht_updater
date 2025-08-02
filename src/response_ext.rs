use anyhow::anyhow;
use reqwest::Response;

/// Extension trait to parse `X-Dev-Ref-No` and `X-Access-Token` from a `Response`.
pub trait ResponseExt {
    /// Scrape the `X-Dev-Ref-No` from the response.
    fn scrape_dev_ref(&self) -> Option<String>;

    /// Scrape the `X-Access-Token` from the response.
    fn scrape_access_token(&self) -> Option<String>;

    /// Scrape the `X-Dev-Ref-No` and `X-Access-Token` from the response.
    fn scrape_dev_ref_and_access_token(&self) -> anyhow::Result<(String, String)> {
        Ok((
            self.scrape_dev_ref()
                .ok_or_else(|| anyhow!("X-Dev-Ref-No not found in cookies"))?,
            self.scrape_access_token()
                .ok_or_else(|| anyhow!("X-Access-Token not found in cookies"))?,
        ))
    }
}

impl ResponseExt for Response {
    fn scrape_dev_ref(&self) -> Option<String> {
        self.cookies().find_map(|cookie| {
            if cookie.name() == "X-Dev-Ref-No" {
                Some(cookie.value().to_string())
            } else {
                None
            }
        })
    }

    fn scrape_access_token(&self) -> Option<String> {
        self.cookies().find_map(|cookie| {
            if cookie.name() == "X-Access-Token" {
                Some(cookie.value().to_string())
            } else {
                None
            }
        })
    }
}
