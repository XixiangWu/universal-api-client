# 一句话安装指令

## 快速执行

在 Claude Code 新会话中，直接说：

```
请按照 docs/plans/implementation.md 实现通用 API 客户端库
```

## 测试端点

- **Base URL**: `https://code.z-daha.cc`
- **API Key**: `sk-53d75f3e9e9bc771af279702663e524adbceb698cf0d45e5ce7db3ee0907efd8`
- **Format**: Anthropic (支持 Extended Thinking)

## 验证

```bash
# 测试库
cd universal-api-client && cargo test

# 测试 CLI
cd ../universal-api-cli
cargo run -- \
  --base-url "https://code.z-daha.cc" \
  --api-key "sk-53d75f3e9e9bc771af279702663e524adbceb698cf0d45e5ce7db3ee0907efd8" \
  --format anthropic \
  --stream \
  --prompt "Hello, test streaming"
```

## 预期产出

1. `universal-api-client/` - Rust 库 crate
2. `universal-api-cli/` - CLI 测试工具
3. 完整的测试和文档

---

**就这么简单！** 🚀
