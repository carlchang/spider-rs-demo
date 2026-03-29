# AGENTS.md - Spider-RS-Demo Development Guide

This file provides guidelines for AI agents working in this codebase.

## Build Commands

### Cargo (Recommended for Development)
```bash
cargo check                    # Type checking
cargo build                    # Debug build
cargo build --release          # Release build
cargo fmt                      # Format code
cargo clippy                   # Lint checks
cargo clippy -- -D warnings    # Strict linting
cargo test                     # Run all tests
cargo test <test_name>         # Run specific test
cargo test -- --nocapture      # Show print output
```

### Bazel (Production Builds)
```bash
bazel build //:spider_rs_demo_bin   # Build binary
bazel build //:spider_rs_demo       # Build library
bazel test //:spider_rs_demo_test   # Run tests
bazel run //:spider_rs_demo_bin -- <args>  # Run app
```

## Code Style Guidelines

### Naming Conventions
- **Types/Structs/Enums**: CamelCase (e.g., `Movie`, `CrawlerError`)
- **Functions/Variables**: snake_case (e.g., `crawl_movies`, `base_url`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Enum Variants**: CamelCase (e.g., `NetworkError`, `LoginFailed`)

### Import Organization
Group imports: 1) std library, 2) external crates (alphabetical), 3) local modules.
```rust
use std::collections::HashMap;
use std::time::Duration;
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::module::something;
```

### Error Handling
Use `thiserror` with `#[derive(Error, Debug)]`:
```rust
#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Login failed: {0}")]
    LoginFailed(String),
}

pub fn new(base_url: &str) -> Result<Self, CrawlerError> {
    let url = Url::parse(base_url)
        .map_err(|e| CrawlerError::ParseError(e.to_string()))?;
    Ok(Self { url })
}
```

### Async Code Patterns
- Main entry: `#[tokio::main]`
- Async tests: `#[tokio::test]`
- Functions return `Result<T, Error>`:
```rust
pub async fn crawl_movies(&mut self) -> Result<CrawlResult, CrawlerError> {
    let response = self.client.get(url).send().await
        .map_err(|e| CrawlerError::NetworkError(e.to_string()))?;
    Ok(result)
}
```

### Testing Conventions
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }
}

#[cfg(test)]
mod integration_tests {
    #[tokio::test]
    async fn test_async_function() {
        assert!(async_function().await.is_ok());
    }
}
```

### Type Derives
```rust
// Error types: #[derive(Error, Debug)]
// Data structures: #[derive(Debug, Clone, Serialize, Deserialize)]
// CLI args: #[derive(Parser, Debug)]
```

### Formatting Rules
- Max ~100 characters per line, 4 spaces indentation
- Use `rustfmt` for automatic formatting
- Avoid `println!` in library code; use `log` macros

## Project Structure
```
src/
├── lib.rs          # Library facade (re-exports, formatters)
├── main.rs         # CLI entry point
├── auth.rs         # Authentication logic (login)
├── crawler.rs      # Main crawler (MovieCrawler)
├── error.rs        # Error types (CrawlerError)
├── parser.rs       # HTML parsing logic
├── session.rs      # Session management (HTTP client)
├── types.rs        # Data types (Movie, CrawlResult)
rust/toolchain.bzl  # Bazel Rust toolchain config
```

### Module Responsibilities
| Module | Purpose |
|--------|---------|
| `auth.rs` | Login authentication logic |
| `crawler.rs` | Main crawler orchestration |
| `error.rs` | Error type definitions |
| `parser.rs` | HTML content parsing |
| `session.rs` | HTTP client & session state |
| `types.rs` | Shared data structures |

## Key Dependencies
| Crate | Purpose |
|-------|---------|
| `reqwest` | HTTP client with cookie support |
| `tokio` | Async runtime |
| `serde` | JSON serialization |
| `thiserror` | Error type derivation |
| `clap` | CLI argument parsing |
| `log` / `env_logger` | Logging |

## Common Patterns

### Constructor Pattern
```rust
impl MovieCrawler {
    pub fn new(base_url: &str, username: &str, password: &str) -> Result<Self, CrawlerError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()
            .map_err(|e| CrawlerError::NetworkError(e.to_string()))?;
        Ok(Self { client, ... })
    }
}
```

### URL Handling
```rust
let login_url = self.base_url.join("login")
    .unwrap_or(self.base_url.clone());
```

### Charset-Safe String Handling
```rust
let truncated: String = movie.name.chars().take(20).collect();
```

## Development Workflow
1. Run `cargo check` frequently during development
2. Use `cargo clippy -- -D warnings` before commits
3. Run `cargo fmt` before committing
4. Ensure all tests pass: `cargo test`
5. Build release version: `cargo build --release`
