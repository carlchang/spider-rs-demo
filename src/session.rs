use reqwest::Client;
use std::time::Duration;
use url::Url;

use crate::error::CrawlerError;

pub struct Session {
    pub client: Client,
    pub base_url: Url,
    pub username: String,
    pub password: String,
}

impl Session {
    pub fn new(base_url: &str, username: &str, password: &str) -> Result<Self, CrawlerError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()
            .map_err(|e| CrawlerError::NetworkError(e.to_string()))?;

        let base_url = Url::parse(base_url).map_err(|e| CrawlerError::ParseError(e.to_string()))?;

        Ok(Self {
            client,
            base_url,
            username: username.to_string(),
            password: password.to_string(),
        })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn credentials(&self) -> (&str, &str) {
        (&self.username, &self.password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new("https://example.com", "user", "pass");
        assert!(session.is_ok());

        let session = session.unwrap();
        assert_eq!(session.username, "user");
        assert_eq!(session.password, "pass");
    }

    #[test]
    fn test_session_credentials() {
        let session = Session::new("https://example.com", "admin", "admin").unwrap();
        let (user, pass) = session.credentials();
        assert_eq!(user, "admin");
        assert_eq!(pass, "admin");
    }

    #[test]
    fn test_invalid_url() {
        let result = Session::new("not-a-url", "user", "pass");
        assert!(result.is_err());
    }
}
