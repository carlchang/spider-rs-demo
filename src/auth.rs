use log::info;
use reqwest::Client;
use url::Url;

use crate::error::CrawlerError;

pub async fn login(
    client: &Client,
    base_url: &Url,
    username: &str,
    password: &str,
) -> Result<(), CrawlerError> {
    let login_url = base_url.join("login").unwrap_or(base_url.clone());

    let params = [("username", username), ("password", password)];

    let response = client
        .post(login_url.as_str())
        .form(&params)
        .send()
        .await
        .map_err(|e| CrawlerError::NetworkError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(CrawlerError::LoginFailed(format!(
            "Login failed with status: {}",
            response.status()
        )));
    }

    let login_body = response.text().await.unwrap_or_default();

    if login_body.contains("用户名或密码错误")
        || login_body.contains("Invalid username or password")
    {
        return Err(CrawlerError::LoginFailed(
            "Login failed: Invalid username or password".to_string(),
        ));
    }

    info!("Login successful");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_login_params() {
        let username = "admin";
        let password = "admin";
        let params = [("username", username), ("password", password)];
        assert_eq!(params[0].1, "admin");
        assert_eq!(params[1].1, "admin");
    }
}
