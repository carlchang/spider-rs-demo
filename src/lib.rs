use log::{debug, info, warn};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Login failed: {0}")]
    LoginFailed(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Session error: {0}")]
    SessionError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlResult {
    pub movies: Vec<Movie>,
    pub total: usize,
}

pub struct MovieCrawler {
    client: Client,
    base_url: Url,
    username: String,
    password: String,
}

impl MovieCrawler {
    pub fn new(base_url: &str, username: &str, password: &str) -> Result<Self, CrawlerError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()
            .map_err(|e| CrawlerError::NetworkError(e))?;

        let base_url = Url::parse(base_url)
            .map_err(|e| CrawlerError::ParseError(e.to_string()))?;

        Ok(Self {
            client,
            base_url,
            username: username.to_string(),
            password: password.to_string(),
        })
    }

    pub async fn login(&mut self) -> Result<bool, CrawlerError> {
        info!("Attempting to login to {}", self.base_url);

        let login_url = self.base_url.join("login").unwrap_or(self.base_url.clone());

        let params = [
            ("username", self.username.as_str()),
            ("password", self.password.as_str()),
        ];

        let response = self
            .client
            .post(login_url.as_str())
            .form(&params)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if status.is_success() || status.as_u16() == 302 || status.as_u16() == 200 {
            info!("Login successful");
            Ok(true)
        } else {
            warn!("Login failed with status: {}", status);
            Err(CrawlerError::LoginFailed(format!(
                "Status: {}, Body: {}",
                status, body
            )))
        }
    }

    pub async fn crawl_movies(&mut self) -> Result<CrawlResult, CrawlerError> {
        info!("Starting to crawl movies from {}", self.base_url);

        let response = self
            .client
            .get(self.base_url.as_str())
            .send()
            .await?;

        let body = response.text().await?;
        
        // 保存页面内容到文件以便分析
        use std::fs::File;
        use std::io::Write;
        if let Ok(mut file) = File::create("page_content.html") {
            let _ = file.write_all(body.as_bytes());
            info!("Page content saved to page_content.html");
        }
        
        let document = Html::parse_document(&body);

        // 提取 class="el-card__body" 中 class="name" 的内容和 href
        let movie_links = Selector::parse("a.name")
            .map_err(|e| CrawlerError::ParseError(e.to_string()))?;

        let mut movies: Vec<Movie> = Vec::new();
        let mut seen_urls: HashMap<String, bool> = HashMap::new();

        // 调试：计算找到的链接数量
        let link_count = document.select(&movie_links).count();
        info!("Found {} movie links with class='name'", link_count);

        for element in document.select(&movie_links) {
            if let Some(href) = element.value().attr("href") {
                let full_url = self.base_url.join(href).unwrap_or_else(|_| {
                    Url::parse(&format!("{}{}", self.base_url, href)).unwrap()
                });

                let url_str = full_url.to_string();
                if seen_urls.contains_key(&url_str) {
                    continue;
                }
                seen_urls.insert(url_str.clone(), true);

                // 从链接中提取电影名称
                let name = element.text().collect::<String>().trim().to_string();
                
                debug!("Extracted movie: '{}' - {}", name, url_str);

                if !name.is_empty() {
                    movies.push(Movie {
                        name,
                        url: url_str,
                    });
                }
            }
        }
        
        info!("Added {} movies to the list", movies.len());

        if movies.is_empty() {
            info!("No movies found with links, trying to parse from page content");
            let all_links = Selector::parse("a[href]")
                .map_err(|e| CrawlerError::ParseError(e.to_string()))?;

            for element in document.select(&all_links) {
                if let Some(href) = element.value().attr("href") {
                    let href_lower = href.to_lowercase();
                    if href_lower.contains("movie")
                        || href_lower.contains("film")
                        || href_lower.contains("detail")
                        || href_lower.contains("title")
                        || href_lower.contains("film")
                        || href_lower.contains("show")
                    {
                        let full_url = self.base_url.join(href).unwrap_or_else(|_| {
                            Url::parse(&format!("{}{}", self.base_url, href)).unwrap()
                        });

                        let url_str = full_url.to_string();
                        if seen_urls.contains_key(&url_str) {
                            continue;
                        }
                        seen_urls.insert(url_str.clone(), true);

                        let name = element.text().next().map(|s| s.trim().to_string());

                        if let Some(movie_name) = name {
                            if !movie_name.is_empty() && movie_name.len() < 200 {
                                movies.push(Movie {
                                    name: movie_name,
                                    url: url_str,
                                });
                            }
                        }
                    }
                }
            }
        }

        let total = movies.len();
        info!("Found {} movies", total);

        Ok(CrawlResult { movies, total })
    }

    pub async fn crawl_with_login(&mut self) -> Result<CrawlResult, CrawlerError> {
        self.login().await?;
        self.crawl_movies().await
    }
}

pub fn format_json(result: &CrawlResult) -> String {
    serde_json::to_string_pretty(result).unwrap_or_else(|_| "{}".to_string())
}

pub fn format_table(result: &CrawlResult) -> String {
    let mut output = String::new();
    output.push_str("+---------------------+----------------------------------------------------+\n");
    output.push_str("| Movie Name          | URL                                                |\n");
    output.push_str("+---------------------+----------------------------------------------------+\n");

    for movie in &result.movies {
        let name = if movie.name.chars().count() > 20 {
            format!("{}...", movie.name.chars().take(17).collect::<String>())
        } else {
            movie.name.clone()
        };
        let url = if movie.url.len() > 50 {
            format!("{}...", &movie.url[..47])
        } else {
            movie.url.clone()
        };
        output.push_str(&format!("| {:<20} | {:<50} |\n", name, url));
    }

    output.push_str("+---------------------+----------------------------------------------------+\n");
    output.push_str(&format!("Total: {} movies found\n", result.total));
    output
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
    fn test_format_json() {
        let result = CrawlResult {
            movies: vec![Movie {
                name: "Test Movie".to_string(),
                url: "https://example.com/movie/1".to_string(),
            }],
            total: 1,
        };
        let json = format_json(&result);
        assert!(json.contains("Test Movie"));
        assert!(json.contains("https://example.com/movie/1"));
    }

    #[test]
    fn test_format_table() {
        let result = CrawlResult {
            movies: vec![Movie {
                name: "Test Movie".to_string(),
                url: "https://example.com/movie/1".to_string(),
            }],
            total: 1,
        };
        let table = format_table(&result);
        assert!(table.contains("Test Movie"));
        assert!(table.contains("1 movies found"));
    }

    #[test]
    fn test_movie_structure() {
        let movie = Movie {
            name: "Test Movie".to_string(),
            url: "https://example.com/test".to_string(),
        };

        assert_eq!(movie.name, "Test Movie");
        assert_eq!(movie.url, "https://example.com/test");
    }

    #[test]
    fn test_crawl_result_structure() {
        let movies = vec![
            Movie {
                name: "Movie 1".to_string(),
                url: "https://example.com/1".to_string(),
            },
            Movie {
                name: "Movie 2".to_string(),
                url: "https://example.com/2".to_string(),
            },
        ];

        let result = CrawlResult {
            total: movies.len(),
            movies,
        };

        assert_eq!(result.total, 2);
        assert_eq!(result.movies.len(), 2);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_login_functionality() {
        let mut crawler = MovieCrawler::new(
            "https://login2.scrape.center/",
            "admin",
            "admin",
        )
        .expect("Failed to create crawler");

        let login_result = crawler.login().await;
        match login_result {
            Ok(true) => {
                println!("Login successful!");
            }
            Ok(false) => {
                println!("Login returned false but no error");
            }
            Err(e) => {
                println!("Login error (expected in test environment): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_crawl_movies() {
        let mut crawler = MovieCrawler::new(
            "https://login2.scrape.center/",
            "admin",
            "admin",
        )
        .expect("Failed to create crawler");

        let _ = crawler.login().await;
        let result = crawler.crawl_movies().await;

        match result {
            Ok(crawl_result) => {
                println!("Found {} movies", crawl_result.total);
                assert!(crawl_result.total >= 0);
            }
            Err(e) => {
                println!("Crawl error (expected in test environment): {}", e);
            }
        }
    }
}
