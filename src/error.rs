use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Login failed: {0}")]
    LoginFailed(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Session error: {0}")]
    SessionError(String),

    #[error("Spider error: {0}")]
    SpiderError(String),
}
