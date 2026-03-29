# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.0] - 2026-03-29

### Added

- **Modular Architecture**: Refactored `lib.rs` into focused modules
  - `src/auth.rs`: Authentication logic
  - `src/session.rs`: HTTP client & session management
  - `src/parser.rs`: HTML parsing logic
  - `src/crawler.rs`: Main crawler orchestration
  - `src/error.rs`: Error type definitions
  - `src/types.rs`: Data structures
- **AGENTS.md**: Development guidelines for AI agents
- **OPTIMIZE_LOG.md**: Optimization process documentation

### Fixed

- **Login Status Verification**: Added content-based login verification to detect failed logins even when HTTP status is 200
- **Nested If Statements**: Collapsed nested if statements for better readability (clippy fix)
- **Default Implementation**: Replaced manual `Default` trait implementation with `#[derive(Default)]`
- **Unused Code**: Removed unused `extract_movie_name_from_url` function and `crawl_with_login` method

### Changed

- **Bazel Configuration**: Updated for Bazel 9.x compatibility
  - Updated `rules_rust` to version 0.51.0
  - Changed `BUILD` to use `glob(["src/*.rs"])` for module discovery
  - Removed invalid dependencies (`spider`, `scraper`)
- **lib.rs**: Now serves as a thin facade with re-exports

### Removed

- `crawl_with_login` method (redundant)
- `extract_movie_name_from_url` function (unused)
- `spider` and `scraper` dependencies (unused)

### Tests

- Increased test coverage from 7 to 18 tests
- Added tests for each new module (auth, session, parser, crawler)

## [1.0.0] - 2026-03-24

### Added

- **Movie Crawler**: Initial implementation
  - Login authentication with cookie session management
  - Movie information extraction from target website
  - JSON and table output formats
- **Core Components**:
  - `MovieCrawler` struct with `crawl_movies()` method
  - `Movie` and `CrawlResult` data structures
  - `CrawlerError` enum for error handling
- **CLI Interface**: Command-line argument parsing with clap
- **Logging System**: Multi-level logging with env_logger

### Technical Stack

- Rust 2021 Edition
- reqwest 0.12 (HTTP client with cookie support)
- tokio 1.x (async runtime)
- serde 1.0 (serialization)
- thiserror 2.0 (error handling)
- clap 4.5 (CLI parsing)
- log 0.4 (logging)

### Bug Fixes (Initial Release)

- Fixed HTML parsing for movie links
- Fixed Chinese character handling (UTF-8 boundary issues)
- Fixed terminal encoding issues for Chinese output

---

## Commit History

| Commit | Description |
|--------|-------------|
| `0b30821` | Update Bazel configuration for modular structure and Bazel 9.x |
| `c13443b` | Refactor into modular architecture |
| `2a8a2ec` | Add OPTIMIZE_LOG.md documenting code quality improvements |
| `f871c61` | Add AGENTS.md and code quality improvements |
| `f6eebfb` | Update documentation |
| `f5cbfe3` | Initial bug fixes and FIX_PROCESS.md |
| `0c3a91b` | Add README.md |
| `bd7b09b` | Initial code implementation |
| `b468122` | Initial commit |
