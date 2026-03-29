use std::collections::HashMap;

use url::Url;

use crate::types::{CrawlResult, Movie};

pub fn parse_movies(html: &str, base_url: &Url) -> CrawlResult {
    let mut movies: Vec<Movie> = Vec::new();
    let mut seen_urls: HashMap<String, bool> = HashMap::new();

    let lines: Vec<&str> = html.lines().collect();

    for i in 0..lines.len() {
        let line = lines[i];

        if line.contains("class=\"name\"") && line.contains("href=\"/detail/") {
            if let Some(movie) = extract_movie(line, i, lines.len(), &lines, base_url, &seen_urls) {
                seen_urls.insert(movie.url.clone(), true);
                movies.push(movie);
            }
        }
    }

    let total = movies.len();
    CrawlResult { movies, total }
}

fn extract_movie(
    line: &str,
    index: usize,
    total_lines: usize,
    lines: &[&str],
    base_url: &Url,
    seen_urls: &HashMap<String, bool>,
) -> Option<Movie> {
    let movie_url = extract_url(line, base_url)?;
    let movie_name = extract_name_from_lines(lines, index, total_lines)?;

    if movie_name.is_empty() || movie_url.is_empty() {
        return None;
    }
    if seen_urls.contains_key(&movie_url) {
        return None;
    }

    Some(Movie {
        name: movie_name,
        url: movie_url,
    })
}

fn extract_url(line: &str, base_url: &Url) -> Option<String> {
    if let Some(start) = line.find("href=") {
        let start_quote = start + 6;
        if let Some(end_quote) = line[start_quote..].find('"') {
            let href = &line[start_quote..start_quote + end_quote];
            return Some(if href.starts_with("http") {
                href.to_string()
            } else {
                base_url.join(href).unwrap_or(base_url.clone()).to_string()
            });
        }
    }
    None
}

pub fn extract_name_from_lines(
    lines: &[&str],
    current_index: usize,
    total_lines: usize,
) -> Option<String> {
    if current_index + 1 >= total_lines {
        return None;
    }

    let h2_line = lines[current_index + 1];
    if h2_line.contains("<h2") {
        if let Some(start) = h2_line.find('>') {
            let content_start = start + 1;
            if let Some(end) = h2_line[content_start..].find('<') {
                let name = h2_line[content_start..content_start + end].trim();
                return Some(name.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_movies_empty() {
        let html = "<html><body></body></html>";
        let base_url = Url::parse("https://example.com").unwrap();
        let result = parse_movies(html, &base_url);
        assert_eq!(result.total, 0);
        assert!(result.movies.is_empty());
    }

    #[test]
    fn test_parse_movies_with_data() {
        let html = r#"
<html>
<body>
<a class="name" href="/detail/1"></a>
<h2>Test Movie</h2>
</body>
</html>
"#;
        let base_url = Url::parse("https://example.com").unwrap();
        let result = parse_movies(html, &base_url);
        assert_eq!(result.total, 1);
        assert_eq!(result.movies[0].name, "Test Movie");
        assert_eq!(result.movies[0].url, "https://example.com/detail/1");
    }

    #[test]
    fn test_extract_name_from_lines() {
        let lines = ["<a href>", "<h2>Movie Title</h2>"];
        let result = extract_name_from_lines(&lines, 0, 2);
        assert_eq!(result, Some("Movie Title".to_string()));
    }

    #[test]
    fn test_extract_name_no_h2() {
        let lines = ["<a href>", "<p>No movie</p>"];
        let result = extract_name_from_lines(&lines, 0, 2);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_url_absolute() {
        let line = r#"<a class="name" href="https://other.com/movie/1"></a>"#;
        let base_url = Url::parse("https://example.com").unwrap();
        let result = extract_url(line, &base_url);
        assert_eq!(result, Some("https://other.com/movie/1".to_string()));
    }

    #[test]
    fn test_extract_url_relative() {
        let line = r#"<a class="name" href="/detail/1"></a>"#;
        let base_url = Url::parse("https://example.com").unwrap();
        let result = extract_url(line, &base_url);
        assert_eq!(result, Some("https://example.com/detail/1".to_string()));
    }
}
