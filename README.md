# 🔭 NetScope

**Modern network diagnostics CLI — one command, full picture.**

Written in Rust. Single binary. Zero config.

```bash
$ netscope check example.com

╔══════════════════════════════════════════════════════════╗
║  🔭 NetScope Diagnosis                                   ║
║  Target:  example.com (93.184.216.34)                   ║
║  Score:   85/100  ████████████████████░░░░  ⚠ Good    ║
╠══════════════════════════════════════════════════════════╣
║  📍 DNS        ✅ 2 records, all resolvers agree         ║
║  📶 Ping       ✅ 12.3ms avg, 0% loss                  ║
║  🛤️  Route      ⚠ Limited (admin privileges required)   ║
║  🔍 Ports      ✅ 80/443 open                          ║
║  🌐 HTTP       ✅ 200 OK, 234ms total                   ║
║  🔒 TLS        ⚠️  Expires in 258 days                   ║
╠══════════════════════════════════════════════════════════╣
║  ⚠️  1 warning  •  💡 1 suggestion                      ║
╚══════════════════════════════════════════════════════════╝
```

## Install

### Pre-built Binaries

```bash
# Linux (x86_64)
curl -L https://github.com/liuji666/netscope/releases/latest/download/netscope-x86_64-unknown-linux-musl.tar.gz | tar xz
sudo mv netscope /usr/local/bin/

# macOS (x86_64)
curl -L https://github.com/liuji666/netscope/releases/latest/download/netscope-x86_64-apple-darwin.tar.gz | tar xz
mv netscope /usr/local/bin/

# Windows (x86_64)
# Download from: https://github.com/liuji666/netscope/releases/latest/download/netscope-x86_64-pc-windows-msvc.zip
```

### From Source

```bash
cargo build --release
cargo install --path .
```

## Quick Start

```bash
# One-command full diagnosis
netscope check example.com

# Individual commands
netscope ping example.com         # Enhanced ping
netscope dns example.com           # DNS records + resolver comparison
netscope port example.com          # Port scan
netscope http https://example.com  # HTTP timing breakdown
netscope optimize                  # Find the best DNS server
netscope site github              # Test multiple entry points
netscope mirror github            # Test mirror speeds

# JSON output
netscope check example.com --json | jq '.score'

# Markdown report
netscope check example.com --md > report.md
```

## Commands

| Command | Description | Status |
|---------|-------------|--------|
| `check` | ★ Full-chain diagnosis (DNS + Ping + Route + Ports + HTTP) | ✅ |
| `ping` | Enhanced ping with stats and colored output | ✅ |
| `trace` | Traceroute (limited without admin) | ⚠️ Basic |
| `dns` | DNS records + multi-resolver comparison | ✅ |
| `port` | TCP port scan + service identification | ✅ |
| `http` | HTTP timing breakdown (DNS→TCP→TLS→TTFB→Total) | ✅ |
| `speed` | Latency/jitter test (bandwidth requires server) | ⚠️ Limited |
| `myip` | Public IP information | ✅ |
| `report` | Full Markdown diagnosis report | ✅ |
| `optimize` | ★ Find and configure the best DNS server | ✅ |
| `mirror` | ★ Test mirror speeds for GitHub/Docker/PyPI/etc | ✅ |
| `site` | ★ Test multiple entry points for any website | ✅ |
| `batch` | ★ Batch test multiple URLs | ✅ |
| `interactive` | ★ Guided network diagnosis | ✅ |
| `config` | ★ Configuration management | ✅ |

**Legend**:
- ✅ Fully functional
- ⚠️ Limited functionality (see notes below)

### Limitations

- **`trace`**: Full traceroute requires administrator/root privileges. Current implementation provides basic connectivity test.
- **`speed`**: Bandwidth download/upload test requires a speed test server endpoint. Currently provides latency and jitter measurement only.

### Security

- **Port scanning**: Private IP address scanning is blocked by default to prevent unauthorized network scanning.
- **Concurrency limits**: Port scanning has built-in concurrency limits (max 200) to prevent network abuse.
- **No root required**: Most operations work without administrator privileges.

## Why NetScope?

**Before:** `ping` → `traceroute` → `dig` → `curl` → manually correlate

**After:** `netscope check example.com` — one command, full picture

## Features

- 🚀 **Fast**: Rust-powered, sub-second results for most commands
- 🔒 **Secure**: No root required for most operations, private IP scanning blocked
- 📊 **Comprehensive**: DNS, ping, port scan, HTTP analysis, TLS检查
- 🎯 **Intelligent**: Automatic scoring and diagnostic suggestions
- 🌐 **Practical**: DNS optimization, mirror testing, site entry testing
- 📱 **Multi-platform**: Linux, macOS, Windows supported
- 🌍 **Multi-language**: English/Chinese support with first-run setup wizard
- 💾 **Config persistence**: Save preferences and cached results

## Configuration

NetScope works out of the box with sensible defaults. First-time users will see a setup wizard to configure language preferences.

```bash
# View current configuration
netscope config show

# Set preferred DNS
netscope config set dns 1.1.1.1

# Add favorite site
netscope config add https://github.com

# Clear all configuration
netscope config clear
```

## Shell Completion

NetScope supports shell auto-completion for bash, zsh, and fish.

```bash
# Bash
netscope --completions bash > ~/.netscope-completion.bash
source ~/.netscope-completion.bash

# Zsh
netscope --completions zsh > ~/.netscope-completion.zsh
source ~/.netscope-completion.zsh

# Fish
netscope --completions fish > ~/.config/fish/completions/netscope.fish
```

## CI/CD

- ✅ **Continuous Integration**: GitHub Actions runs on every push/pull request
- ✅ **Automated Releases**: Tagged releases automatically build binaries for Linux/macOS/Windows
- ✅ **Format & Lint Checks**: Enforces code quality standards

## License

MIT