use anyhow::anyhow;
use scraper::{Html, Selector};
use std::sync::LazyLock;

static CSRF_TOKEN_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("a[data-csrf_token]").expect("Could not create CSRF token selector")
});
static USER_ID_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("a[data-user_id]").expect("Could not create user ID selector")
});

/// Extension trait for `Html`.
pub trait HtmlExt {
    /// Scrape the CSRF token from the HTML page.
    fn scrape_csrf_token(&self) -> Option<&str>;

    /// Scrape the user ID from the HTML page.
    fn scrape_user_id(&self) -> Option<&str>;

    /// Scrape the CSRF token and user ID from the HTML page.
    fn scrape_csrf_token_and_user_id(&self) -> anyhow::Result<(&str, &str)> {
        Ok((
            self.scrape_csrf_token()
                .ok_or_else(|| anyhow!("Could not find element with CSRF token"))?,
            self.scrape_user_id()
                .ok_or_else(|| anyhow!("Could not find element with user ID"))?,
        ))
    }
}

impl HtmlExt for Html {
    fn scrape_csrf_token(&self) -> Option<&str> {
        self.select(&CSRF_TOKEN_SELECTOR)
            .find_map(|element| element.value().attr("data-csrf_token"))
    }
    fn scrape_user_id(&self) -> Option<&str> {
        self.select(&USER_ID_SELECTOR)
            .find_map(|element| element.value().attr("data-user_id"))
    }
}
