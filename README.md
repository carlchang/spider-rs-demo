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

### 电影信息提取

电影信息提取使用简单的字符串匹配：

```rust
// 简单的 HTML 解析，寻找电影链接
let lines: Vec<&str> = home_body.lines().collect();
for line in lines {
    if line.contains("href=") && (line.contains("detail") || line.contains("movie")) {
        // 提取 href 属性
        // ...
    }
}
```

可根据目标网站结构调整提取逻辑。

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

## 变更历史

### v1.1.0 - 电影名称提取优化

#### 变更内容

1. **电影名称提取逻辑重构**
   - 从 `class="el-card__body"` 中提取 `class="name"` 的内容作为电影名称
   - 修复了电影名称与 URL 对应关系不正确的问题
   - 使用逐行解析 HTML 的方式，确保 a 标签的 href 属性与相邻的 h2 标签内容正确匹配

2. **HTML 解析优化**
   - 查找包含 `class="name"` 和 `href="/detail/"` 的 a 标签
   - 从 a 标签提取 href 属性作为电影详情页 URL
   - 从相邻的 h2 标签提取电影名称

#### 修复过程

**问题**：原实现中电影名称与 URL 对应关系不正确

**原因分析**：
- 原代码在提取 URL 和名称时使用了不同的遍历方式
- URL 提取和名称提取没有正确关联

**解决方案**：
```rust
for i in 0..lines.len() {
    let line = lines[i];
    
    // 查找包含 class="name" 和 href="/detail/" 的 a 标签
    if line.contains("class=\"name\"") && line.contains("href=\"/detail/") {
        // 提取 href 属性
        let mut movie_url = String::new();
        // ... URL 提取逻辑
        
        // 提取电影名称（下一行的 h2 标签内容）
        let mut movie_name = String::new();
        if i + 1 < lines.len() {
            let h2_line = lines[i + 1];
            if h2_line.contains("<h2") {
                // ... 名称提取逻辑
            }
        }
        
        // 确保名称和 URL 正确对应
        if !movie_name.is_empty() && !movie_url.is_empty() {
            movies.push(Movie {
                name: movie_name,
                url: movie_url,
            });
        }
    }
}
```

**验证结果**：
- 成功提取 10 部电影
- 电影名称与 URL 正确对应
- 示例输出：
  - 霸王别姬 - Farewell My Concubine -> /detail/1
  - 这个杀手不太冷 - Léon -> /detail/2
  - 肖申克的救赎 - The Shawshank Redemption -> /detail/3

### v1.0.0 - 初始版本

#### 功能特性

- 基于 reqwest 的 HTTP 客户端
- 登录认证功能
- 电影信息提取
- JSON 和表格输出格式

#### 技术选型

- 使用 reqwest 替代 scraper 库
- 使用 reqwest 的 cookie_store 功能管理会话
- 使用简单的字符串匹配进行 HTML 解析

## Bug 审查与修复记录

### 修复版本 v1.1.1

#### 审查发现的问题

| Bug ID | 问题描述 | 严重程度 |
|--------|----------|----------|
| Bug1 | 登录状态检查不充分：仅检查 HTTP 状态码，未验证登录是否真正成功 | 中 |
| Bug2 | 未使用的函数：`extract_movie_name_from_url` 函数定义但未被使用 | 低 |
| Bug3 | 冗余方法：`crawl_with_login` 方法仅调用 `crawl_movies`，无额外功能 | 低 |

#### 修复方案

**Bug1: 登录状态检查增强**
- **问题**：某些网站在登录失败时仍返回 200 状态码
- **解决方案**：添加对登录后页面内容的验证
  - 检查返回内容是否包含登录失败的标识
  - 确保真正的登录成功

**Bug2: 未使用的函数清理**
- **问题**：`extract_movie_name_from_url` 函数未被使用
- **解决方案**：删除该函数及其相关测试用例，减少代码冗余

**Bug3: 冗余方法清理**
- **问题**：`crawl_with_login` 方法仅调用 `crawl_movies`，无实际用途
- **解决方案**：删除该方法，直接调用 `crawl_movies`

#### 修复实现

**登录状态检查增强**：
```rust
// 检查登录是否真正成功（通过检查返回内容）
let login_body = response.text().await.unwrap_or_default();

// 检查是否包含登录失败的标识
if login_body.contains("用户名或密码错误") || login_body.contains("Invalid username or password") {
    return Err(CrawlerError::LoginFailed("Login failed: Invalid username or password".to_string()));
}
```

**代码清理**：
- 删除未使用的 `extract_movie_name_from_url` 函数
- 删除冗余的 `crawl_with_login` 方法
- 更新 `main.rs` 中的调用方式

#### 验证结果

所有 7 个测试用例全部通过：
- 单元测试：爬虫创建、JSON 格式化、表格格式化、数据结构
- 集成测试：登录功能、电影爬取

#### 修复效果

- **安全性提升**：登录验证更加可靠，避免了仅依赖状态码的风险
- **代码质量**：减少了冗余代码，提高了代码可维护性
- **性能**：移除了未使用的函数，减少了编译时间和运行时开销

## 联系方式

如有问题或建议，请提交 Issue。