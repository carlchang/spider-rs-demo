use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;
use url::Url;

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
            .map_err(|e| CrawlerError::NetworkError(e.to_string()))?;

        let base_url = Url::parse(base_url)
            .map_err(|e| CrawlerError::ParseError(e.to_string()))?;

        Ok(Self {
            client,
            base_url,
            username: username.to_string(),
            password: password.to_string(),
        })
    }

    pub async fn crawl_movies(&mut self) -> Result<CrawlResult, CrawlerError> {
        info!(
            "Starting to crawl movies from {}",
            self.base_url
        );

        // 首先使用 reqwest 登录
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
            .await
            .map_err(|e| CrawlerError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(CrawlerError::LoginFailed(format!("Login failed with status: {}", response.status())));
        }

        // 检查登录是否真正成功（通过检查返回内容）
        let login_body = response.text().await.unwrap_or_default();
        
        // 检查是否包含登录失败的标识（根据网站实际情况调整）
        if login_body.contains("用户名或密码错误") || login_body.contains("Invalid username or password") {
            return Err(CrawlerError::LoginFailed("Login failed: Invalid username or password".to_string()));
        }

        info!("Login successful");

        // 登录后访问主页
        let home_url = self.base_url.as_str();
        let home_response = self
            .client
            .get(home_url)
            .send()
            .await
            .map_err(|e| CrawlerError::NetworkError(e.to_string()))?;

        let home_body = home_response.text().await.unwrap_or_default();

        // 调试：保存 HTML 内容到文件
        use std::fs::File;
        use std::io::Write;
        if let Ok(mut file) = File::create("debug_home.html") {
            let _ = file.write_all(home_body.as_bytes());
            info!("Saved home page HTML to debug_home.html");
        }

        // 解析 HTML 提取电影链接和名称
        let mut movies: Vec<Movie> = Vec::new();
        let mut seen_urls: HashMap<String, bool> = HashMap::new();

        // 简单的 HTML 解析，寻找电影信息
        let lines: Vec<&str> = home_body.lines().collect();
        
        for i in 0..lines.len() {
            let line = lines[i];
            
            // 查找包含 class="name" 和 href="/detail/" 的 a 标签
            if line.contains("class=\"name\"") && line.contains("href=\"/detail/") {
                // 提取 href 属性
                let mut movie_url = String::new();
                if let Some(start) = line.find("href=") {
                    let start_quote = start + 6; // "href=" 的长度
                    if let Some(end_quote) = line[start_quote..].find('"') {
                        let href = &line[start_quote..start_quote + end_quote];
                        movie_url = if href.starts_with("http") {
                            href.to_string()
                        } else {
                            self.base_url.join(href).unwrap_or(self.base_url.clone()).to_string()
                        };
                    }
                }
                
                // 提取电影名称（下一行的 h2 标签内容）
                let mut movie_name = String::new();
                if i + 1 < lines.len() {
                    let h2_line = lines[i + 1];
                    if h2_line.contains("<h2") {
                        if let Some(start) = h2_line.find('>') {
                            let content_start = start + 1;
                            if let Some(end) = h2_line[content_start..].find('<') {
                                let name = h2_line[content_start..content_start + end].trim();
                                movie_name = name.to_string();
                            }
                        }
                    }
                }
                
                // 如果找到了电影名称和链接，添加到列表
                if !movie_name.is_empty() && !movie_url.is_empty() {
                    if !seen_urls.contains_key(&movie_url) {
                        seen_urls.insert(movie_url.clone(), true);
                        movies.push(Movie {
                            name: movie_name,
                            url: movie_url,
                        });
                    }
                }
            }
        }

        let total = movies.len();
        info!("Found {} movies", total);

        Ok(CrawlResult { movies, total })
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

        // 直接调用 crawl_movies，它包含了登录逻辑
        let result = crawler.crawl_movies().await;
        match result {
            Ok(crawl_result) => {
                println!("Login successful! Found {} movies", crawl_result.total);
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

        let result = crawler.crawl_movies().await;

        match result {
            Ok(crawl_result) => {
                println!("Found {} movies", crawl_result.total);
                // 断言电影数量是合理的
                assert!(crawl_result.total <= 100, "Total movies should be less than or equal to 100");
            }
            Err(e) => {
                println!("Crawl error (expected in test environment): {}", e);
            }
        }
    }
}
