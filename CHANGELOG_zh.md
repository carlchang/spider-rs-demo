# 变更日志

本文档记录项目中所有重要的变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
项目遵循[语义化版本](https://semver.org/lang/zh-CN/spec/v2.0.0.html)。

## [未发布]

## [1.1.0] - 2026-03-29

### 新增

- **模块化架构**：将 `lib.rs` 重构为专注的模块
  - `src/auth.rs`：认证逻辑
  - `src/session.rs`：HTTP 客户端与会话管理
  - `src/parser.rs`：HTML 解析逻辑
  - `src/crawler.rs`：爬虫主逻辑编排
  - `src/error.rs`：错误类型定义
  - `src/types.rs`：数据结构
- **AGENTS.md**：AI 代理开发指南
- **OPTIMIZE_LOG.md**：优化过程文档

### 修复

- **登录状态验证**：添加基于内容的登录验证，即使 HTTP 状态码为 200 也能检测登录失败
- **嵌套 if 语句**：合并嵌套 if 语句以提高可读性（clippy 修复）
- **Default 实现**：使用 `#[derive(Default)]` 替换手动实现
- **未使用代码**：移除未使用的 `extract_movie_name_from_url` 函数和 `crawl_with_login` 方法

### 变更

- **Bazel 配置**：更新以兼容 Bazel 9.x
  - 更新 `rules_rust` 至版本 0.51.0
  - 修改 `BUILD` 使用 `glob(["src/*.rs"])` 进行模块发现
  - 移除无效依赖（`spider`、`scraper`）
- **lib.rs**：现在作为薄封装层，提供重导出

### 移除

- `crawl_with_login` 方法（冗余）
- `extract_movie_name_from_url` 函数（未使用）
- `spider` 和 `scraper` 依赖（未使用）

### 测试

- 测试覆盖率从 7 增加到 18
- 为每个新模块添加测试（auth、session、parser、crawler）

## [1.0.0] - 2026-03-24

### 新增

- **电影爬虫**：初始实现
  - 使用 Cookie 会话管理的登录认证
  - 从目标网站提取电影信息
  - JSON 和表格输出格式
- **核心组件**：
  - `MovieCrawler` 结构体及 `crawl_movies()` 方法
  - `Movie` 和 `CrawlResult` 数据结构
  - `CrawlerError` 枚举用于错误处理
- **CLI 界面**：使用 clap 进行命令行参数解析
- **日志系统**：使用 env_logger 支持多级别日志

### 技术栈

- Rust 2021 Edition
- reqwest 0.12（支持 Cookie 的 HTTP 客户端）
- tokio 1.x（异步运行时）
- serde 1.0（序列化）
- thiserror 2.0（错误处理）
- clap 4.5（CLI 解析）
- log 0.4（日志）

### Bug 修复（初始版本）

- 修复电影链接的 HTML 解析
- 修复中文字符处理（UTF-8 边界问题）
- 修复中文输出的终端编码问题

---

## 提交历史

| 提交 | 描述 |
|------|------|
| `0b30821` | 更新 Bazel 配置以支持模块化架构和 Bazel 9.x |
| `c13443b` | 重构为模块化架构 |
| `2a8a2ec` | 添加 OPTIMIZE_LOG.md 记录代码质量改进 |
| `f871c61` | 添加 AGENTS.md 和代码质量改进 |
| `f6eebfb` | 更新文档 |
| `f5cbfe3` | 初始 bug 修复和 FIX_PROCESS.md |
| `0c3a91b` | 添加 README.md |
| `bd7b09b` | 初始代码实现 |
| `b468122` | 初始提交 |
