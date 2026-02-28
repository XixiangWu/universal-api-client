# Universal API Client

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

一个轻量级的 Rust 库，提供统一接口访问 Anthropic 和 OpenAI 兼容的 API 服务。

## ✨ 特性

- 🚀 **统一接口** - 一套 API 访问多个 AI 服务提供商
- 🔄 **格式转换** - 自动在 Anthropic 和 OpenAI 格式间转换
- 📡 **流式支持** - 完整的 SSE 流式响应处理
- 🧠 **Extended Thinking** - 支持 Claude 的思考过程输出
- ⚡ **异步优先** - 基于 tokio 的高性能异步实现
- 🛠️ **Builder 模式** - 简洁优雅的配置方式

## 📦 安装

详见 [INSTALL.md](INSTALL.md)

**快速开始**:

```bash
git clone https://github.com/你的用户名/universal-api-client.git
cd universal-api-client
```

在 Claude Code 中执行：

```
请按照 docs/plans/implementation.md 实现通用 API 客户端库
```

## 🚀 快速开始

### 基础使用

```rust
use universal_api_client::{ApiClient, ApiFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::builder()
        .base_url("https://api.anthropic.com")
        .api_key("your-api-key")
        .format(ApiFormat::Anthropic)
        .build()?;

    let response = client.chat("Hello!").await?;
    println!("{}", response.content);

    Ok(())
}
```

### 流式响应

```rust
use futures::StreamExt;
use universal_api_client::StreamEvent;

let mut stream = client.chat_stream("Tell me a story").await?;
while let Some(event) = stream.next().await {
    match event? {
        StreamEvent::ContentDelta(text) => print!("{}", text),
        StreamEvent::Done => break,
        _ => {}
    }
}
```

### 自定义配置

```rust
use std::time::Duration;

let client = ApiClient::builder()
    .base_url("https://api.anthropic.com")
    .api_key("your-api-key")
    .format(ApiFormat::Anthropic)
    .model("claude-sonnet-4-6")
    .max_tokens(4096)
    .temperature(0.7)
    .timeout(Duration::from_secs(60))
    .build()?;
```

## 🛠️ CLI 工具

项目包含一个命令行工具用于测试和演示：

```bash
# 安装
cargo install --path universal-api-cli

# 基础使用
universal-api-cli \
  --base-url "https://api.anthropic.com" \
  --api-key "your-key" \
  --format anthropic \
  --prompt "Hello!"

# 流式输出
universal-api-cli \
  --base-url "https://api.anthropic.com" \
  --api-key "your-key" \
  --format anthropic \
  --stream \
  --prompt "Tell me a story"

# JSON 输出（用于脚本）
universal-api-cli \
  --base-url "https://api.anthropic.com" \
  --api-key "your-key" \
  --format anthropic \
  --json \
  --prompt "Hello!"
```

## 📚 文档

- [安装指南](INSTALL.md) - 详细的安装和设置说明
- [快速开始](QUICKSTART.md) - 5分钟上手指南
- [架构设计](docs/plans/design.md) - 技术架构和设计决策
- [实现计划](docs/plans/implementation.md) - 完整的实现步骤

## 🧪 测试

项目包含已验证的测试端点：

```bash
cd universal-api-cli
cargo run -- \
  --base-url "https://code.z-daha.cc" \
  --api-key "sk-53d75f3e9e9bc771af279702663e524adbceb698cf0d45e5ce7db3ee0907efd8" \
  --format anthropic \
  --stream \
  --prompt "Hello, test streaming"
```

## 🏗️ 项目结构

```
universal-api-client/
├── src/
│   ├── lib.rs              # 公共 API
│   ├── client.rs           # ApiClient 实现
│   ├── error.rs            # 错误类型
│   ├── provider/           # Provider 适配器
│   │   ├── mod.rs
│   │   ├── adapter.rs      # Adapter trait
│   │   ├── auth.rs         # 认证策略
│   │   ├── claude.rs       # Anthropic 适配器
│   │   └── openai.rs       # OpenAI 适配器
│   ├── transform.rs        # 格式转换
│   └── streaming.rs        # SSE 流式处理
├── examples/
│   └── simple_chat.rs      # 使用示例
└── Cargo.toml

universal-api-cli/
├── src/
│   └── main.rs             # CLI 实现
└── Cargo.toml
```

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

本项目从 [cc-switch](https://github.com/farion1231/cc-switch) 提取核心 API 通信层，感谢原作者的优秀工作。

## 📮 联系方式

- 提交 Issue: [GitHub Issues](https://github.com/你的用户名/universal-api-client/issues)
- 讨论: [GitHub Discussions](https://github.com/你的用户名/universal-api-client/discussions)

---

**记住**: 只需一句话 `请按照 docs/plans/implementation.md 实现通用 API 客户端库` 即可开始！🚀
