# 🔭 NetScope

**Modern network diagnostics CLI** — One command for full network analysis.

Built with Rust. Single binary. Zero configuration.

---

## Why NetScope?

**Before:** `ping` → `traceroute` → `dig` → `curl` → manually correlate

**After:** `netscope check example.com` — One command, full picture

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

### ✨ Key Features

- 🚀 **Blazing Fast**: Rust-powered for sub-second results
- 🔒 **Secure**: No root required, private IP protection
- 📊 **Comprehensive**: DNS, ping, port scan, HTTP, TLS analysis
- 🎯 **Intelligent**: 100-point scoring with actionable suggestions
- 🌐 **Practical**: DNS optimization, mirror testing, site entry testing
- 📱 **Cross-platform**: Linux, macOS, Windows native support
- 🌍 **Multi-language**: English/Chinese with setup wizard
- 💾 **Persistent**: Save preferences and cached results

### 🎯 Killer Features

| Feature | Description |
|---------|-------------|
| **DNS Optimizer** | One-click find and configure the fastest DNS |
| **Mirror Speed Test** | Test GitHub/Docker/PyPI mirror speeds in seconds |
| **Site Entry Test** | Find the fastest entrance for any website |
| **Smart Diagnosis** | Automated scoring with detailed suggestions |

---

## Install

### 📦 Pre-built Binaries (Recommended)

**Linux (x86_64)**
```bash
curl -L https://github.com/liuji-666/netscope/releases/latest/download/netscope-x86_64-unknown-linux-musl.tar.gz | tar xz
sudo mv netscope /usr/local/bin/
```

**macOS (x86_64)**
```bash
curl -L https://github.com/liuji-666/netscope/releases/latest/download/netscope-x86_64-apple-darwin.tar.gz | tar xz
mv netscope /usr/local/bin/
```

**Windows (x86_64)**

**PowerShell (Recommended)**
```powershell
# Create install directory
mkdir -p "$env:USERPROFILE\.local\bin"

# Download and extract
Invoke-WebRequest -Uri "https://github.com/liuji-666/netscope/releases/latest/download/netscope-x86_64-pc-windows-msvc.zip" -OutFile "netscope.zip"
Expand-Archive -Path "netscope.zip" -DestinationPath "$env:USERPROFILE\.local\bin" -Force
Remove-Item "netscope.zip"

# Add to PATH (temporary)
$env:PATH += ";$env:USERPROFILE\.local\bin"

# Add to PATH (permanent, requires terminal restart)
[Environment]::SetEnvironmentVariable("PATH", $env:PATH + ";$env:USERPROFILE\.local\bin", "User")
```

**Manual Installation**
1. Download: https://github.com/liuji-666/netscope/releases/latest/download/netscope-x86_64-pc-windows-msvc.zip
2. Extract to any directory
3. Add the directory to system PATH

### 🛠️ From Source (Requires Rust)

**Prerequisites**
- Rust 1.70+ (install via rustup)
- Cargo (included with Rust)

**Install Rust (first time)**
```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows (PowerShell)
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe -y
```

**Build and Install**
```bash
# Clone repository
git clone https://github.com/liuji-666/netscope.git
cd netscope

# Build release
cargo build --release

# Install to system
cargo install --path .

# Or run directly
./target/release/netscope --help
```

---

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

---

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

---

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

---

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

---

## CI/CD

- ✅ **Continuous Integration**: GitHub Actions runs on every push/pull request
- ✅ **Automated Releases**: Tagged releases automatically build binaries for Linux/macOS/Windows
- ✅ **Format & Lint Checks**: Enforces code quality standards

---

## 🍵 Support This Project

If NetScope saves you time and headaches, consider buying me a coffee!


- **GitHub Sponsors**: [github.com/sponsors/liuji-666](https://github.com/sponsors/liuji-666)

Your support keeps this project going! ☕

---

## License

MIT