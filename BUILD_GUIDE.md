# Spider Rust Demo - 项目构建指南

## 项目概述

本项目使用 Rust 语言和 spider 网络爬虫库实现了电影网站登录和数据爬取功能。

## 项目结构

```
spider-rs-demo/
├── Cargo.toml           # Rust 依赖配置
├── WORKSPACE            # Bazel 工作区配置
├── MODULE.bazel         # Bazel 模块配置
├── BUILD                # Bazel 构建规则
├── src/
│   ├── lib.rs          # 核心爬虫逻辑和库代码
│   └── main.rs         # 命令行入口程序
└── rust/
    └── toolchain.bzl    # Rust 工具链配置
```

## 核心功能

### 1. 登录功能 (MovieCrawler::login)
- 使用 reqwest 客户端发送 POST 请求到登录端点
- 支持 Cookie 会话管理
- 支持表单参数提交 (username, password)
- 包含错误处理和重试机制

### 2. 电影数据爬取 (MovieCrawler::crawl_movies)
- 使用 scraper 库解析 HTML 页面
- 智能识别电影链接和标题
- URL 完整化处理 (确保包含协议、域名和路径)
- 去重机制避免重复数据

### 3. 输出格式化
- JSON 格式输出 (format_json)
- 表格格式输出 (format_table)

## 依赖配置

### Rust 依赖 (Cargo.toml)
- spider = "2.5" (核心爬虫库，支持 chrome 功能)
- reqwest = "0.12" (HTTP 客户端)
- scraper = "0.21" (HTML 解析)
- tokio = "1" (异步运行时)
- serde = "1.0" (序列化)
- clap = "4.5" (命令行参数解析)

### Bazel 依赖
- rules_rust (Bazel Rust 规则)
- crate_universe (Rust crate 依赖管理)

## 构建方法

### 方法 1: 使用 Cargo (推荐用于开发)

```bash
# 确保没有其他 cargo 进程在运行
# 清理旧的构建缓存
cargo clean

# 检查代码编译
cargo check

# 构建项目
cargo build --release

# 运行测试
cargo test

# 运行程序
cargo run -- --url https://login2.scrape.center/ --username admin --password admin --output table
```

### 方法 2: 使用 Bazel (用于生产构建)

```bash
# 安装依赖并构建
bazel build //:spider_rs_demo_bin

# 运行测试
bazel test //:spider_rs_demo_test

# 运行程序
bazel run //:spider_rs_demo_bin -- --url https://login2.scrape.center/
```

## 命令行参数

- `--url, -u`: 目标网站 URL (默认: https://login2.scrape.center/)
- `--username`: 登录用户名 (默认: admin)
- `--password, -p`: 登录密码 (默认: admin)
- `--output, -o`: 输出格式 (json 或 table, 默认: json)

## 输出示例

### JSON 格式
```json
{
  "movies": [
    {
      "name": "Movie Title 1",
      "url": "https://login2.scrape.center/detail/1"
    },
    {
      "name": "Movie Title 2",
      "url": "https://login2.scrape.center/detail/2"
    }
  ],
  "total": 2
}
```

### Table 格式
```
+---------------------+----------------------------------------------------+
| Movie Name          | URL                                                |
+---------------------+----------------------------------------------------+
| Movie Title 1       | https://login2.scrape.center/detail/1              |
| Movie Title 2       | https://login2.scrape.center/detail/2              |
+---------------------+----------------------------------------------------+
Total: 2 movies found
```

## 错误处理

程序包含完善的错误处理机制:

- `CrawlerError::NetworkError`: 网络请求失败
- `CrawlerError::LoginFailed`: 登录认证失败
- `CrawlerError::ParseError`: HTML 解析错误
- `CrawlerError::SessionError`: 会话管理错误

## 测试覆盖

### 单元测试
- `test_movie_crawler_creation`: 爬虫创建测试
- `test_format_json`: JSON 格式化测试
- `test_format_table`: 表格格式化测试
- `test_movie_structure`: 数据结构测试

### 集成测试
- `test_login_functionality`: 登录功能测试
- `test_crawl_movies`: 电影爬取测试

## 代码质量保证

1. **静态分析**: 代码使用 Rust 严格模式，避免常见错误
2. **类型安全**: 使用强类型系统和 Result 错误处理
3. **单元测试**: 核心功能模块有完整的单元测试
4. **集成测试**: 包含端到端的集成测试
5. **错误处理**: 所有可能的错误都有对应的处理方式

## 登录网站信息

- 演示网址: https://login2.scrape.center/
- 演示账号: admin / admin
- 网站类型: 电影评分网站

## 注意事项

1. **网络要求**: 需要稳定的网络连接以访问演示网站
2. **登录会话**: 登录后会保持 Cookie 会话
3. **爬取限制**: 请遵守网站的 robots.txt 和使用条款
4. **错误重试**: 网络不稳定时会自动重试 (最多 3 次)

## 故障排除

### 问题 1: cargo build 报 "Blocking waiting for file lock"
**解决方案**:
```powershell
# 查找并停止所有 cargo 进程
Get-Process -Name cargo | Stop-Process -Force

# 或者使用新的临时 CARGO_HOME
$env:CARGO_HOME = "D:\temp_cargo"
cargo build
```

### 问题 2: Bazel 无法解析 rules_rust
**解决方案**:
```bash
# 清理 Bazel 缓存
bazel clean

# 重新同步
bazel sync
```

### 问题 3: 编译错误
**解决方案**:
```bash
# 更新依赖
cargo update

# 重新检查
cargo check --locked
```

## 许可和贡献

本项目仅供学习和演示使用。请勿将其用于任何未经授权的爬取活动。
