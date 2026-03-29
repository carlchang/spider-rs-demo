# AGENTS.md - Spider-RS-Demo 开发指南

本文档为 AI 代理在此代码库中工作提供指导方针。

## 构建命令

### Cargo（推荐用于开发）

```bash
cargo check                    # 类型检查
cargo build                    # Debug 构建
cargo build --release          # Release 构建
cargo fmt                      # 格式化代码
cargo clippy                   # Lint 检查
cargo clippy -- -D warnings    # 严格 Lint
cargo test                     # 运行所有测试
cargo test <test_name>         # 运行特定测试
cargo test -- --nocapture      # 显示 print 输出
```

### Bazel（生产环境）

```bash
bazel build //:spider_rs_demo_bin   # 构建二进制文件
bazel build //:spider_rs_demo       # 构建库
bazel test //:spider_rs_demo_test   # 运行测试
bazel run //:spider_rs_demo_bin -- <args>  # 运行应用
```

## 代码风格指南

### 命名约定

- **类型/结构体/枚举**：CamelCase（例如：`Movie`、`CrawlerError`）
- **函数/变量**：snake_case（例如：`crawl_movies`、`base_url`）
- **常量**：SCREAMING_SNAKE_CASE
- **枚举变体**：CamelCase（例如：`NetworkError`、`LoginFailed`）

### 导入组织

分组导入顺序：1) std 库，2) 外部 crate（字母顺序），3) 本地模块。

```rust
use std::collections::HashMap;
use std::time::Duration;

use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::module::something;
```

### 错误处理

使用 `thiserror` 和 `#[derive(Error, Debug)]`：

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

### 异步代码模式

- 主入口：`#[tokio::main]`
- 异步测试：`#[tokio::test]`
- 函数返回 `Result<T, Error>`：

```rust
pub async fn crawl_movies(&mut self) -> Result<CrawlResult, CrawlerError> {
    let response = self.client.get(url).send().await
        .map_err(|e| CrawlerError::NetworkError(e.to_string()))?;
    Ok(result)
}
```

### 测试约定

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

### 类型派生

```rust
// 错误类型：#[derive(Error, Debug)]
// 数据结构：#[derive(Debug, Clone, Serialize, Deserialize)]
// CLI 参数：#[derive(Parser, Debug)]
```

### 格式化规则

- 每行最多约 100 个字符，4 空格缩进
- 使用 `rustfmt` 自动格式化
- 避免在库代码中使用 `println!`，使用 `log` 宏

## 项目结构

```
src/
├── lib.rs          # 库入口（重导出、格式化函数）
├── main.rs         # CLI 入口
├── auth.rs         # 认证逻辑
├── crawler.rs      # 爬虫主逻辑
├── error.rs        # 错误类型定义
├── parser.rs       # HTML 解析逻辑
├── session.rs      # 会话管理（HTTP 客户端）
├── types.rs        # 数据结构定义
rust/toolchain.bzl  # Bazel Rust 工具链配置
```

### 模块职责

| 模块 | 用途 |
|------|------|
| `auth.rs` | 登录认证逻辑 |
| `crawler.rs` | 爬虫编排 |
| `error.rs` | 错误类型定义 |
| `parser.rs` | HTML 内容解析 |
| `session.rs` | HTTP 客户端和会话状态 |
| `types.rs` | 共享数据结构 |

## 关键依赖

| Crate | 用途 |
|-------|------|
| `reqwest` | HTTP 客户端，支持 Cookie |
| `tokio` | 异步运行时 |
| `serde` | JSON 序列化 |
| `thiserror` | 错误类型派生 |
| `clap` | CLI 参数解析 |
| `log` / `env_logger` | 日志 |

## 常用模式

### 构造函数模式

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

### URL 处理

```rust
let login_url = self.base_url.join("login")
    .unwrap_or(self.base_url.clone());
```

### 字符集安全字符串处理

```rust
let truncated: String = movie.name.chars().take(20).collect();
```

## 开发工作流程

1. 开发期间频繁运行 `cargo check`
2. 提交前使用 `cargo clippy -- -D warnings`
3. 提交前运行 `cargo fmt`
4. 确保所有测试通过：`cargo test`
5. 测试时构建 Release 版本：`cargo build --release`
