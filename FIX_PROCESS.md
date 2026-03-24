# 电影爬虫项目修复过程记录

## 项目概述
本项目是一个使用 Rust 语言和 spider-rs 库开发的电影爬虫，用于登录 `https://login2.scrape.center/` 网站并提取电影名称和详情页URL。

## 修复过程

### 1. 初始问题
- 项目无法成功提取电影信息
- 登录成功但找不到电影链接
- 中文字符处理问题

### 2. 主要修复点

#### 2.1 选择器优化
- **问题**：初始选择器无法匹配电影链接
- **解决方案**：使用更精确的选择器 `a.name` 来匹配电影名称链接
- **代码修改**：
  ```rust
  let movie_links = Selector::parse("a.name")
      .map_err(|e| CrawlerError::ParseError(e.to_string()))?;
  ```

#### 2.2 中文字符边界错误
- **问题**：处理中文字符时出现 "byte index is not a char boundary" 错误
- **原因**：中文字符在 UTF-8 编码中占用多个字节，直接使用字符串切片会破坏字符边界
- **解决方案**：使用 `chars()` 方法和 `take()` 方法来安全处理中文字符
- **代码修改**：
  ```rust
  let name = if movie.name.chars().count() > 20 {
      let chars = movie.name.chars();
      let truncated: String = chars.take(17).collect();
      format!("{}...", truncated)
  } else {
      movie.name.clone()
  };
  ```

#### 2.3 终端编码问题
- **问题**：日志中的中文显示为乱码
- **原因**：终端编码与程序输出编码不匹配
- **解决方案**：使用 `debug!` 宏记录详细信息，使用 `info!` 宏记录截断的信息
- **代码修改**：
  ```rust
  debug!("Extracted movie: '{}' - {}", name, url_str);
  info!("Extracted movie: {} - {}", name.chars().take(20).collect::<String>(), url_str);
  ```

#### 2.4 代码清理
- **问题**：代码中包含调试代码和不必要的等待时间
- **解决方案**：移除调试代码，简化主函数逻辑
- **代码修改**：
  - 移除保存页面内容到文件的调试代码
  - 移除主函数中的 15 秒等待时间

### 3. 验证结果

#### 3.1 构建成功
```bash
cargo build --release
# 输出：Finished `release` profile [optimized] target(s) in 1.68s
```

#### 3.2 运行测试

**JSON 输出**：
```bash
cargo run --release -- --url https://login2.scrape.center/ --username admin --password admin --output json
```

**表格输出**：
```bash
cargo run --release -- --url https://login2.scrape.center/ --username admin --password admin --output table
```

#### 3.3 提取结果
成功提取了 10 部电影：
1. 霸王别姬 - Farewell My Concubine
2. 这个杀手不太冷 - Léon
3. 肖申克的救赎 - The Shawshank Redemption
4. 泰坦尼克号 - Titanic
5. 罗马假日 - Roman Holiday
6. 唐伯虎点秋香 - Flirting Scholar
7. 乱世佳人 - Gone with the Wind
8. 喜剧之王 - The King of Comedy
9. 楚门的世界 - The Truman Show
10. 狮子王 - The Lion King

## 技术栈
- **Rust 语言**：安全、高效的系统编程语言
- **reqwest**：HTTP 客户端，用于登录和爬取数据
- **scraper**：HTML 解析库，用于提取电影信息
- **serde**：JSON 序列化/反序列化库
- **clap**：命令行参数解析库
- **log**：日志记录库
- **thiserror**：错误处理库

## 总结
通过优化选择器、修复中文字符处理问题、解决终端编码问题和清理代码，项目现在能够成功登录网站并提取电影信息。输出格式清晰，支持 JSON 和表格两种格式，满足了用户的需求。