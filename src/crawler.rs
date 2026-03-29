use log::info;

use crate::auth;
use crate::error::CrawlerError;
use crate::parser;
use crate::session::Session;
use crate::types::CrawlResult;

pub struct MovieCrawler {
    session: Session,
}

impl MovieCrawler {
    pub fn new(base_url: &str, username: &str, password: &str) -> Result<Self, CrawlerError> {
        let session = Session::new(base_url, username, password)?;
        Ok(Self { session })
    }

    pub async fn crawl_movies(&mut self) -> Result<CrawlResult, CrawlerError> {
        info!("Starting to crawl movies from {}", self.session.base_url());

        let (username, password) = self.session.credentials();
        auth::login(self.session.client(), self.session.base_url(), username, password).await?;

        let home_response = self
            .session
            .client()
            .get(self.session.base_url().as_str())
            .send()
            .await
            .map_err(|e| CrawlerError::NetworkError(e.to_string()))?;

        let home_body = home_response.text().await.unwrap_or_default();

        let result = parser::parse_movies(&home_body, self.session.base_url());

        info!("Found {} movies", result.total);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movie_crawler_creation() {
        let crawler = MovieCrawler::new("https://example.com", "admin", "admin");
        assert!(crawler.is_ok());
    }

    #[test]
    fn test_movie_crawler_creation_with_invalid_url() {
        let crawler = MovieCrawler::new("not-a-url", "admin", "admin");
        assert!(crawler.is_err());
    }
}
