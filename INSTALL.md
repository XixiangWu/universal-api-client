# Universal API Client - Installation Guide

## Prerequisites

- Git
- Rust toolchain (will be installed if needed)

## Installation

### Clone the repository

```bash
git clone https://github.com/XixiangWu/universal-api-client.git
cd universal-api-client
```

### Execute the implementation plan

Open a new Claude Code session in the cloned directory and say:

```
Fetch and follow instructions from https://raw.githubusercontent.com/XixiangWu/universal-api-client/main/INSTALL.md
```

Or simply:

```
请按照 docs/plans/implementation.md 实现通用 API 客户端库
```

The AI will:
1. Read the complete implementation plan
2. Execute all 8 tasks sequentially
3. Run tests after each task
4. Commit changes with proper messages
5. Generate final documentation

## Verification

After implementation completes, verify the installation:

```bash
# Check library builds
cd universal-api-client
cargo test

# Check CLI tool works
cd ../universal-api-cli
cargo run -- --help
```

You should see:
- All tests passing in `universal-api-client`
- CLI help message displaying available options

## Testing with Live Endpoint

Test the implementation with the provided endpoint:

```bash
cd universal-api-cli
cargo run -- \
  --base-url "https://code.z-daha.cc" \
  --api-key "sk-53d75f3e9e9bc771af279702663e524adbceb698cf0d45e5ce7db3ee0907efd8" \
  --format anthropic \
  --stream \
  --prompt "Hello, please respond with a short greeting"
```

You should see streaming output with real-time text generation.

## Using the Library

After implementation, add to your `Cargo.toml`:

```toml
[dependencies]
universal-api-client = { path = "../universal-api-client" }
```

Or publish to crates.io and use:

```toml
[dependencies]
universal-api-client = "0.1.0"
```

## Updates

The implementation plan is versioned in the repository. To update:

```bash
git pull
```

Then re-run the implementation command in a new Claude Code session if the plan has changed.

## Troubleshooting

**Issue: Compilation errors**
- Ensure Rust toolchain is up to date: `rustup update`
- Check that all dependencies are available: `cargo check`

**Issue: API endpoint not responding**
- Verify network connectivity
- Check API key is correct
- Ensure base URL is accessible

**Issue: Tests failing**
- Review test output for specific failures
- Check that test endpoint is accessible
- Verify Extended Thinking support is enabled

## What Gets Created

After successful implementation:

```
universal-api-client/          # Library crate
├── src/
│   ├── lib.rs                 # Public API
│   ├── client.rs              # ApiClient implementation
│   ├── error.rs               # Error types
│   ├── provider/              # Provider adapters
│   ├── transform.rs           # Format conversion
│   └── streaming.rs           # SSE streaming
├── examples/
│   └── simple_chat.rs         # Usage example
└── Cargo.toml

universal-api-cli/             # CLI tool
├── src/
│   └── main.rs                # CLI implementation
└── Cargo.toml
```

## Uninstalling

To remove the implementation:

```bash
rm -rf universal-api-client universal-api-cli
```

To remove the entire project:

```bash
cd ..
rm -rf universal-api-client
```

---

**Quick Start**: Just say `请按照 docs/plans/implementation.md 实现通用 API 客户端库` in Claude Code 🚀
