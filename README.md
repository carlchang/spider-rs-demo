# Spider-RS-Demo 电影爬虫

一个基于 Rust 语言开发的电影信息爬虫工具，支持登录认证、会话管理和多种输出格式。

## 功能特性

- **登录认证**：支持用户名/密码登录，自动维护会话状态
- **电影信息提取**：从目标网站提取电影名称和详情页URL
- **多种输出格式**：支持 JSON 和表格两种输出格式
- **错误处理**：完善的错误处理机制，包括网络错误、登录失败、解析错误等
- **日志记录**：支持多级别日志输出，便于调试和监控
- **命令行界面**：友好的命令行参数解析，支持自定义配置

## 技术栈

| 技术 | 版本 | 用途 |
|------|------|------|
| Rust | 2021 Edition | 编程语言 |
| reqwest | 0.12 | HTTP 客户端，支持 Cookie 管理 |
| scraper | 0.21 | HTML 解析和 CSS 选择器 |
| tokio | 1.x | 异步运行时 |
| serde | 1.0 | 序列化/反序列化 |
| clap | 4.5 | 命令行参数解析 |
| log | 0.4 | 日志记录 |
| thiserror | 2.0 | 错误处理 |

## 项目结构

```
spider-rs-demo/
├── src/
│   ├── lib.rs           # 核心库代码
│   └── main.rs          # 主程序入口
├── Cargo.toml           # Rust 项目配置
├── BUILD                # Bazel 构建文件
├── WORKSPACE            # Bazel 工作空间
├── BUILD_GUIDE.md       # 构建指南
├── FIX_PROCESS.md       # 修复过程记录
└── README.md            # 项目说明文档
```

## 安装与构建

### 前置要求

- Rust 1.70 或更高版本
- Cargo 包管理器

### 使用 Cargo 构建

```bash
# 开发模式构建
cargo build

# 发布模式构建（优化性能）
cargo build --release
```

### 使用 Bazel 构建

```bash
# 构建
bazel build //:spider-rs-demo

# 运行
bazel run //:spider-rs-demo
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
| `--output` | `-o` | `json` | 输出格式（json/table） |

### 使用示例

#### JSON 输出

```bash
cargo run --release -- --output json
```

输出示例：
```json
{
  "movies": [
    {
      "name": "霸王别姬 - Farewell My Concubine",
      "url": "https://login2.scrape.center/detail/1"
    },
    {
      "name": "这个杀手不太冷 - Léon",
      "url": "https://login2.scrape.center/detail/2"
    }
  ],
  "total": 10
}
```

#### 表格输出

```bash
cargo run --release -- --output table
```

输出示例：
```
+---------------------+----------------------------------------------------+
| Movie Name          | URL                                                |
+---------------------+----------------------------------------------------+
| 霸王别姬 - Farewell M... | https://login2.scrape.center/detail/1              |
| 这个杀手不太冷 - Léon       | https://login2.scrape.center/detail/2              |
+---------------------+----------------------------------------------------+
Total: 10 movies found
```

## 功能模块说明

### 1. 核心库 (lib.rs)

#### 1.1 数据结构

**Movie 结构体**
```rust
pub struct Movie {
    pub name: String,  // 电影名称
    pub url: String,   // 详情页 URL
}
```

**CrawlResult 结构体**
```rust
pub struct CrawlResult {
    pub movies: Vec<Movie>,  // 电影列表
    pub total: usize,        // 电影总数
}
```

#### 1.2 MovieCrawler 爬虫类

**主要方法**：

| 方法 | 说明 |
|------|------|
| `new(base_url, username, password)` | 创建爬虫实例，初始化 HTTP 客户端 |
| `login()` | 执行登录操作，建立会话 |
| `crawl_movies()` | 爬取电影信息 |
| `crawl_with_login()` | 一站式登录并爬取（推荐使用） |

**使用示例**：
```rust
use spider_rs_demo::MovieCrawler;

#[tokio::main]
async fn main() {
    let mut crawler = MovieCrawler::new(
        "https://login2.scrape.center/",
        "admin",
        "admin"
    ).unwrap();
    
    let result = crawler.crawl_with_login().await.unwrap();
    println!("Found {} movies", result.total);
}
```

#### 1.3 输出格式化函数

| 函数 | 说明 |
|------|------|
| `format_json(result)` | 将结果格式化为 JSON 字符串 |
| `format_table(result)` | 将结果格式化为表格字符串 |

### 2. 主程序 (main.rs)

主程序负责：
- 解析命令行参数
- 初始化日志系统
- 创建爬虫实例
- 执行爬取任务
- 输出结果

### 3. 错误处理

项目使用 `thiserror` 库定义了完善的错误类型：

```rust
pub enum CrawlerError {
    NetworkError(reqwest::Error),    // 网络错误
    LoginFailed(String),              // 登录失败
    ParseError(String),               // 解析错误
    SessionError(String),             // 会话错误
}
```

### 4. 日志系统

支持多级别日志输出：

```bash
# INFO 级别（默认）
RUST_LOG=info cargo run --release

# DEBUG 级别（显示详细信息）
RUST_LOG=debug cargo run --release

# 只显示错误
RUST_LOG=error cargo run --release
```

## 配置选项

### HTTP 客户端配置

在 `lib.rs` 中可以调整 HTTP 客户端配置：

```rust
let client = Client::builder()
    .timeout(Duration::from_secs(30))  // 超时时间
    .cookie_store(true)                 // 启用 Cookie 存储
    .build()?;
```

### 选择器配置

电影信息提取使用 CSS 选择器：

```rust
let movie_links = Selector::parse("a.name")?;
```

可根据目标网站结构调整选择器。

## 开发指南

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_crawler_creation

# 显示测试输出
cargo test -- --nocapture
```

### 代码规范

```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy
```

### 添加新功能

1. 在 `lib.rs` 中添加核心逻辑
2. 在 `main.rs` 中添加命令行支持
3. 添加相应的测试用例
4. 更新文档

## 常见问题

### 1. 登录失败

**原因**：用户名/密码错误或网络问题

**解决方案**：
- 检查用户名和密码是否正确
- 检查网络连接
- 查看日志输出了解详细错误信息

### 2. 无法提取电影信息

**原因**：网站结构变化或选择器不匹配

**解决方案**：
- 检查目标网站 HTML 结构
- 更新 CSS 选择器
- 使用 `RUST_LOG=debug` 查看详细日志

### 3. 中文乱码

**原因**：终端编码问题

**解决方案**：
- 使用支持 UTF-8 的终端
- 在 Windows 上使用 PowerShell 或 Windows Terminal
- 设置终端编码为 UTF-8

## 许可证

本项目采用 MIT 许可证，详见 [LICENSE](LICENSE) 文件。

## 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 联系方式

如有问题或建议，请提交 Issue。