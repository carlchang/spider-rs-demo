# Spider-RS-Demo

A Rust-based movie information crawler with login authentication, session management, and multiple output formats.

## Features

- **Login Authentication**: Username/password login with automatic session state maintenance
- **Movie Information Extraction**: Extract movie names and detail page URLs from target websites
- **Multiple Output Formats**: Support for JSON and table output formats
- **Robust Error Handling**: Comprehensive error handling including network errors, login failures, and parsing errors
- **Logging System**: Multi-level log output for debugging and monitoring
- **CLI Interface**: User-friendly command-line argument parsing

## Tech Stack

| Technology | Version | Purpose |
|------------|---------|---------|
| Rust | 2021 Edition | Programming Language |
| reqwest | 0.12 | HTTP Client with Cookie Support |
| tokio | 1.x | Async Runtime |
| serde | 1.0 | Serialization/Deserialization |
| clap | 4.5 | CLI Argument Parsing |
| log | 0.4 | Logging |
| thiserror | 2.0 | Error Handling |

## Project Structure

```
spider-rs-demo/
├── src/
│   ├── lib.rs          # Library facade (re-exports, formatters)
│   ├── main.rs         # CLI entry point
│   ├── auth.rs         # Authentication logic
│   ├── crawler.rs      # Main crawler
│   ├── error.rs        # Error types
│   ├── parser.rs       # HTML parsing logic
│   ├── session.rs      # Session management
│   └── types.rs        # Data structures
├── Cargo.toml          # Rust package configuration
├── BUILD              # Bazel build file
├── MODULE.bazel       # Bazel module configuration
├── WORKSPACE          # Bazel workspace
├── AGENTS.md          # Development guidelines for AI agents
├── CHANGELOG.md       # Change history
├── FIX_PROCESS.md     # Bug fixes and improvements
└── OPTIMIZE_LOG.md    # Optimization records
```

## Installation & Building

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager

### Build with Cargo (Recommended for Development)

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Build with Bazel (Production)

```bash
# Build binary
bazel build //:spider_rs_demo_bin

# Run application
bazel run //:spider_rs_demo_bin -- --output table
```

## Usage

### Basic Usage

```bash
# Default parameters (JSON output)
cargo run --release

# With custom parameters
cargo run --release -- --url https://login2.scrape.center/ --username admin --password admin --output json
```

### Command Line Arguments

| Argument | Short | Default | Description |
|----------|-------|---------|-------------|
| `--url` | `-u` | `https://login2.scrape.center/` | Target website URL |
| `--username` | `-U` | `admin` | Login username |
| `--password` | `-p` | `admin` | Login password |
| `--output` | `-o` | `json` | Output format (`json` or `table`) |

### Output Examples

#### JSON Output

```bash
cargo run --release -- --output json
```

```json
{
  "movies": [
    {
      "name": "霸王别姬 - Farewell My Concubine",
      "url": "https://login2.scrape.center/detail/1"
    }
  ],
  "total": 10
}
```

#### Table Output

```bash
cargo run --release -- --output table
```

```
+---------------------+----------------------------------------------------+
| Movie Name          | URL                                                |
+---------------------+----------------------------------------------------+
| 霸王别姬 - Farewell M... | https://login2.scrape.center/detail/1              |
+---------------------+----------------------------------------------------+
Total: 10 movies found
```

## Module Architecture

### Data Types (`types.rs`)

```rust
pub struct Movie {
    pub name: String,  // Movie name
    pub url: String,   // Detail page URL
}

pub struct CrawlResult {
    pub movies: Vec<Movie>,  // Movie list
    pub total: usize,       // Total count
}
```

### Session Management (`session.rs`)

Manages HTTP client configuration and session state:

```rust
pub struct Session {
    pub client: Client,
    pub base_url: Url,
    pub username: String,
    pub password: String,
}
```

### Authentication (`auth.rs`)

Handles login authentication:

```rust
pub async fn login(
    client: &Client,
    base_url: &Url,
    username: &str,
    password: &str,
) -> Result<(), CrawlerError>
```

### HTML Parsing (`parser.rs`)

Extracts movie information from HTML:

```rust
pub fn parse_movies(html: &str, base_url: &Url) -> CrawlResult
```

### Crawler (`crawler.rs`)

Main crawler orchestration:

```rust
pub struct MovieCrawler {
    session: Session,
}

impl MovieCrawler {
    pub async fn crawl_movies(&mut self) -> Result<CrawlResult, CrawlerError>
}
```

### Error Handling (`error.rs`)

Uses `thiserror` for comprehensive error types:

```rust
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
}
```

## Library Usage

```rust
use spider_rs_demo::{format_json, format_table, MovieCrawler};

#[tokio::main]
async fn main() {
    let mut crawler = MovieCrawler::new(
        "https://login2.scrape.center/",
        "admin",
        "admin",
    )
    .expect("Failed to create crawler");

    let result = crawler.crawl_movies().await.expect("Crawl failed");
    println!("Found {} movies", result.total);
}
```

## Logging

Multi-level log output:

```bash
# INFO level (default)
RUST_LOG=info cargo run --release

# DEBUG level (detailed output)
RUST_LOG=debug cargo run --release

# Error only
RUST_LOG=error cargo run --release
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_crawler_creation

# Show print output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint checks
cargo clippy
cargo clippy -- -D warnings  # Strict mode
```

## Troubleshooting

### Login Failed

**Possible causes**: Incorrect username/password or network issues

**Solutions**:
- Verify credentials
- Check network connection
- Review log output for details

### No Movies Extracted

**Possible causes**: Website structure changes or selector mismatch

**Solutions**:
- Inspect target website HTML structure
- Update parsing logic
- Use `RUST_LOG=debug` for detailed logs

### Chinese Character Display Issues

**Cause**: Terminal encoding mismatch

**Solutions**:
- Use a UTF-8 compatible terminal
- Use PowerShell or Windows Terminal on Windows
- Set terminal encoding to UTF-8

## Documentation

- [AGENTS.md](AGENTS.md) - Development guidelines for AI agents
- [CHANGELOG.md](CHANGELOG.md) - Change history
- [FIX_PROCESS.md](FIX_PROCESS.md) - Bug fixes and improvements
- [OPTIMIZE_LOG.md](OPTIMIZE_LOG.md) - Optimization records
- [BUILD_GUIDE.md](BUILD_GUIDE.md) - Build instructions

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request
