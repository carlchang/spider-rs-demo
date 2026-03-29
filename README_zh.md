# Spider-RS-Demo 电影爬虫

[English](README.md) | [中文](README_zh.md)

一个基于 Rust 语言开发的电影信息爬虫工具，支持登录认证、会话管理和多种输出格式。

## 功能特性

- **登录认证**：支持用户名/密码登录，自动维护会话状态
- **电影信息提取**：从目标网站提取电影名称和详情页 URL
- **多种输出格式**：支持 JSON 和表格两种输出格式
- **完善的错误处理**：包括网络错误、登录失败、解析错误等
- **日志记录**：支持多级别日志输出，便于调试和监控
- **命令行界面**：友好的命令行参数解析，支持自定义配置

## 技术栈

| 技术 | 版本 | 用途 |
|------|------|------|
| Rust | 2021 Edition | 编程语言 |
| reqwest | 0.12 | HTTP 客户端，支持 Cookie 管理 |
| tokio | 1.x | 异步运行时 |
| serde | 1.0 | 序列化/反序列化 |
| clap | 4.5 | 命令行参数解析 |
| log | 0.4 | 日志记录 |
| thiserror | 2.0 | 错误处理 |

## 项目结构

```
spider-rs-demo/
├── src/
│   ├── lib.rs          # 库入口（重导出、格式化函数）
│   ├── main.rs         # 命令行入口
│   ├── auth.rs         # 认证逻辑
│   ├── crawler.rs      # 爬虫主逻辑
│   ├── error.rs        # 错误类型
│   ├── parser.rs       # HTML 解析逻辑
│   ├── session.rs      # 会话管理
│   └── types.rs        # 数据结构
├── Cargo.toml          # Rust 包配置
├── BUILD              # Bazel 构建文件
├── MODULE.bazel       # Bazel 模块配置
├── WORKSPACE          # Bazel 工作空间
├── README.md          # 英文说明文档
├── README_zh.md       # 中文说明文档
├── AGENTS.md          # AI 代理开发指南（英文）
├── AGENTS_zh.md       # AI 代理开发指南（中文）
├── CHANGELOG.md       # 变更日志（英文）
├── CHANGELOG_zh.md    # 变更日志（中文）
├── FIX_PROCESS.md      # 修复过程记录（英文）
├── FIX_PROCESS_zh.md  # 修复过程记录（中文）
└── OPTIMIZE_LOG.md    # 优化日志
```

## 安装与构建

### 前置要求

- Rust 1.70 或更高版本
- Cargo 包管理器

### 使用 Cargo 构建（推荐用于开发）

```bash
# Debug 构建
cargo build

# Release 构建（优化）
cargo build --release
```

### 使用 Bazel 构建（生产环境）

```bash
# 构建二进制文件
bazel build //:spider_rs_demo_bin

# 运行程序
bazel run //:spider_rs_demo_bin -- --output table
```

## 使用方法

### 基本用法

```bash
# 使用默认参数（JSON 输出）
cargo run --release

# 指定参数
cargo run --release -- --url https://login2.scrape.center/ --username admin --password admin --output json
```

### 命令行参数

| 参数 | 简写 | 默认值 | 说明 |
|------|------|--------|------|
| `--url` | `-u` | `https://login2.scrape.center/` | 目标网站 URL |
| `--username` | `-U` | `admin` | 登录用户名 |
| `--password` | `-p` | `admin` | 登录密码 |
| `--output` | `-o` | `json` | 输出格式（`json` 或 `table`） |

### 输出示例

#### JSON 输出

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

#### 表格输出

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

## 模块架构

### 数据类型 (`types.rs`)

```rust
pub struct Movie {
    pub name: String,  // 电影名称
    pub url: String,   // 详情页 URL
}

pub struct CrawlResult {
    pub movies: Vec<Movie>,  // 电影列表
    pub total: usize,       // 电影总数
}
```

### 会话管理 (`session.rs`)

管理 HTTP 客户端配置和会话状态：

```rust
pub struct Session {
    pub client: Client,
    pub base_url: Url,
    pub username: String,
    pub password: String,
}
```

### 认证模块 (`auth.rs`)

处理登录认证：

```rust
pub async fn login(
    client: &Client,
    base_url: &Url,
    username: &str,
    password: &str,
) -> Result<(), CrawlerError>
```

### HTML 解析 (`parser.rs`)

从 HTML 中提取电影信息：

```rust
pub fn parse_movies(html: &str, base_url: &Url) -> CrawlResult
```

### 爬虫模块 (`crawler.rs`)

爬虫主逻辑编排：

```rust
pub struct MovieCrawler {
    session: Session,
}

impl MovieCrawler {
    pub async fn crawl_movies(&mut self) -> Result<CrawlResult, CrawlerError>
}
```

### 错误处理 (`error.rs`)

使用 `thiserror` 定义错误类型：

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

## 库使用示例

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

## 日志系统

支持多级别日志输出：

```bash
# INFO 级别（默认）
RUST_LOG=info cargo run --release

# DEBUG 级别（详细信息）
RUST_LOG=debug cargo run --release

# 仅显示错误
RUST_LOG=error cargo run --release
```

## 开发指南

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_crawler_creation

# 显示 print 输出
cargo test -- --nocapture
```

### 代码质量

```bash
# 格式化代码
cargo fmt

# Lint 检查
cargo clippy
cargo clippy -- -D warnings  # 严格模式
```

## 常见问题

### 1. 登录失败

**可能原因**：用户名/密码错误或网络问题

**解决方案**：
- 验证凭据是否正确
- 检查网络连接
- 查看日志输出了解详细错误信息

### 2. 无法提取电影信息

**可能原因**：网站结构变化或选择器不匹配

**解决方案**：
- 检查目标网站 HTML 结构
- 更新解析逻辑
- 使用 `RUST_LOG=debug` 查看详细日志

### 3. 中文显示乱码

**原因**：终端编码不匹配

**解决方案**：
- 使用支持 UTF-8 的终端
- 在 Windows 上使用 PowerShell 或 Windows Terminal
- 设置终端编码为 UTF-8

## 文档

- [README.md](README.md) - 英文说明文档
- [README_zh.md](README_zh.md) - 中文说明文档
- [AGENTS.md](AGENTS.md) / [AGENTS_zh.md](AGENTS_zh.md) - AI 代理开发指南
- [CHANGELOG.md](CHANGELOG.md) / [CHANGELOG_zh.md](CHANGELOG_zh.md) - 变更日志
- [FIX_PROCESS.md](FIX_PROCESS.md) / [FIX_PROCESS_zh.md](FIX_PROCESS_zh.md) - 修复过程记录
- [OPTIMIZE_LOG.md](OPTIMIZE_LOG.md) - 优化日志
- [BUILD_GUIDE.md](BUILD_GUIDE.md) - 构建指南

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request
