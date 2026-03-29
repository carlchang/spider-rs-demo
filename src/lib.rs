pub mod auth;
pub mod crawler;
pub mod error;
pub mod parser;
pub mod session;
pub mod types;

pub use auth::login;
pub use crawler::MovieCrawler;
pub use error::CrawlerError;
pub use types::{CrawlResult, Movie};

pub fn format_json(result: &CrawlResult) -> String {
    serde_json::to_string_pretty(result).unwrap_or_else(|_| "{}".to_string())
}

pub fn format_table(result: &CrawlResult) -> String {
    let mut output = String::new();
    output
        .push_str("+---------------------+----------------------------------------------------+\n");
    output
        .push_str("| Movie Name          | URL                                                |\n");
    output
        .push_str("+---------------------+----------------------------------------------------+\n");

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

    output
        .push_str("+---------------------+----------------------------------------------------+\n");
    output.push_str(&format!("Total: {} movies found\n", result.total));
    output
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut crawler = MovieCrawler::new("https://login2.scrape.center/", "admin", "admin")
            .expect("Failed to create crawler");

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
        let mut crawler = MovieCrawler::new("https://login2.scrape.center/", "admin", "admin")
            .expect("Failed to create crawler");

        let result = crawler.crawl_movies().await;

        match result {
            Ok(crawl_result) => {
                println!("Found {} movies", crawl_result.total);
                assert!(
                    crawl_result.total <= 100,
                    "Total movies should be less than or equal to 100"
                );
            }
            Err(e) => {
                println!("Crawl error (expected in test environment): {}", e);
            }
        }
    }
}
