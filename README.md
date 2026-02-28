# Universal API Client

一个轻量级的 Rust 库，提供统一接口访问 Anthropic 和 OpenAI 兼容的 API 服务。

## 特性

- ✅ 支持 Anthropic Messages API
- ✅ 支持 OpenAI Chat Completions API
- ✅ SSE 流式响应
- ✅ Extended Thinking 支持
- ✅ 格式自动转换（Anthropic ↔ OpenAI）
- ✅ Builder 模式配置

## 快速开始

### 安装

```bash
cargo add universal-api-client
```

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

let mut stream = client.chat_stream("Tell me a story").await?;
while let Some(event) = stream.next().await {
    match event? {
        StreamEvent::ContentDelta(text) => print!("{}", text),
        StreamEvent::Done => break,
        _ => {}
    }
}
```

## CLI 工具

```bash
# 安装
cargo install --path universal-api-cli

# 使用
universal-api-cli \
  --base-url "https://api.anthropic.com" \
  --api-key "your-key" \
  --format anthropic \
  --stream \
  --prompt "Hello!"
```

## 实现

在新的 Claude Code 会话中执行：

```
请按照 INSTALL.md 执行
```

## 项目结构

```
universal-api-client-project/
├── README.md                  # 本文件
├── INSTALL.md                 # 一句话安装指令
├── docs/
│   └── plans/
│       ├── design.md          # 架构设计文档
│       └── implementation.md  # 详细实现计划
└── .gitignore
```

## 测试端点

项目包含已验证的测试端点配置：
- Base URL: `https://code.z-daha.cc`
- 支持 Anthropic Messages API
- 支持 Extended Thinking

## License

MIT
