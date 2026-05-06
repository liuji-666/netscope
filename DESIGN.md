# NetScope — 完整设计文档 (Rust)

> **"一条命令，看清整个网络链路"**
> Version: 0.1.0 | Language: Rust | Status: Design

---

## 一、产品定义

### 1.1 一句话

**NetScope** 是用 Rust 编写的轻量级 CLI 网络诊断工具，单二进制分发，一条命令替代 ping + traceroute + dig + curl + speedtest 的分散组合。

### 1.2 设计原则

| 原则 | 含义 |
|---|---|
| **单二进制** | `netscope` 一个文件，无运行时依赖，复制即用 |
| **3 分钟上手** | `cargo install netscope` 或下载 release，零配置 |
| **一条命令出结果** | `netscope check example.com` 全链路诊断 |
| **快** | Rust 原生性能，端口扫描 100 并发 < 2s，全诊断 < 8s |
| **实用 > 炫技** | 每个命令解决一个真实痛点 |
| **渐进增强** | 核心功能零依赖，增强功能 feature gate |

### 1.3 目标

```
性能目标:
  - check 命令（典型场景）< 8 秒
  - 端口扫描 1000 端口 < 5 秒
  - 启动时间 < 50ms
  - 内存占用 < 30MB
  - 二进制大小 < 5MB（release + strip）

平台支持:
  - Linux x86_64 / aarch64
  - macOS x86_64 / aarch64
  - Windows x86_64
```

---

## 二、命令体系

```
netscope <command> [args] [options]

全局选项:
  --json          JSON 输出（管道友好）
  --md            Markdown 输出
  --no-color      禁用颜色
  --timeout N     全局超时秒数（默认 10）
  -v, --verbose   详细模式
  -q, --quiet     静默模式
  -V, --version   版本
  -h, --help      帮助

命令:
  check <target>    ★ 核心 — 一键全链路诊断
  ping <target>     增强版 ping
  trace <target>    增强版 traceroute
  dns <domain>      DNS 全面诊断
  port <target>     TCP 端口扫描
  http <url>        HTTP 全链路分析
  speed             网速测试
  myip              本机出口 IP 信息
  report <target>   生成 Markdown 完整报告
```

---

## 三、技术架构

### 3.1 Crate 依赖选型

```toml
[dependencies]
# CLI 框架
clap = { version = "4", features = ["derive", "color", "wrap_help"] }

# 终端渲染
colored = "2"                    # 彩色文本
tabled = "0.16"                  # 表格渲染
indicatif = "0.17"               # 进度条/Spinner

# 异步运行时
tokio = { version = "1", features = ["full"] }

# 网络
trust-dns-resolver = "0.23"      # DNS 解析（异步）
dns-lookup = "2"                 # 系统 DNS
pnet = "0.35"                    # ICMP / 原始包（可选，feature gate）

# HTTP
reqwest = { version = "0.12", features = ["rustls-tls", "json"] }
# rustls 而非 openssl → 跨平台编译无痛

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# 时间
chrono = "0.4"

# 工具
anyhow = "1"                     # 错误处理
thiserror = "2"                  # 自定义错误
ipnet = "2"                      # IP 网段处理
url = "2"                        # URL 解析

# 可选增强
maxminddb = "0.24"               # GeoLite2 地理位置（feature: geo）

[features]
default = []                     # 最小依赖集
geo = ["maxminddb"]              # 地理位置增强
full = ["geo"]                   # 全功能
```

### 3.2 项目结构

```
netscope/
├── Cargo.toml                   # 项目配置 + 依赖
├── Cargo.lock
├── README.md                    # 用户文档
├── DESIGN.md                    # 本文件
├── LICENSE-MIT
├── build.rs                     # 构建脚本（版本注入等）
│
├── src/
│   ├── main.rs                  # 入口：解析 CLI → 路由到子命令
│   ├── cli.rs                   # clap derive 定义所有命令和参数
│   ├── config.rs                # 默认配置 + 用户配置加载
│   ├── error.rs                 # 统一错误类型
│   ├── output.rs                # 输出格式化（终端/JSON/Markdown）
│   │
│   ├── cmd/                     # 子命令实现
│   │   ├── mod.rs
│   │   ├── check.rs             # ★ check 命令编排器
│   │   ├── ping.rs
│   │   ├── trace.rs
│   │   ├── dns.rs
│   │   ├── port.rs
│   │   ├── http.rs
│   │   ├── speed.rs
│   │   ├── myip.rs
│   │   └── report.rs
│   │
│   ├── core/                    # 核心网络模块（纯逻辑，无 UI）
│   │   ├── mod.rs
│   │   ├── resolver.rs          # DNS 解析
│   │   ├── pinger.rs            # Ping（TCP/ICMP）
│   │   ├── traceroute.rs        # 路由追踪
│   │   ├── scanner.rs           # 端口扫描
│   │   ├── http_probe.rs        # HTTP 探测
│   │   ├── speed_test.rs        # 网速测试
│   │   ├── whois.rs             # WHOIS 查询
│   │   └── ipinfo.rs            # IP 信息查询
│   │
│   └── scoring.rs               # 评分系统
│
└── tests/
    ├── integration_test.rs      # 集成测试
    └── fixtures/                # 测试数据
```

### 3.3 模块职责划分

```
┌─────────────────────────────────────────────────────────┐
│                        main.rs                          │
│                  CLI 解析 → 命令路由                      │
└──────────────────────┬──────────────────────────────────┘
                       │
          ┌────────────┼────────────┐
          │            │            │
       cli.rs      cmd/*.rs     output.rs
    (参数定义)    (命令实现)    (格式化输出)
                       │
                  ┌────┼────┐
                  │         │
              core/*.rs  scoring.rs
             (网络逻辑)   (评分模型)


核心设计：core/ 和 cmd/ 分离

  core/  — 纯数据层
    - 接收参数，返回结构化结果
    - 不关心输出格式
    - 可被其他 crate 复用
    - 所有函数 async

  cmd/  — 编排层
    - 调用 core/ 模块
    - 组装结果
    - 调用 output/ 格式化
    - 处理进度显示
```

### 3.4 核心数据结构

```rust
// ========== 通用 ==========

/// 任何检测模块的返回值
pub struct ProbeResult<T> {
    pub status: ProbeStatus,
    pub data: Option<T>,
    pub error: Option<String>,
    pub duration: Duration,
}

pub enum ProbeStatus {
    Ok,
    Partial,   // 部分成功
    Error,
    Timeout,
    Skipped,
}

// ========== DNS ==========

pub struct DnsResult {
    pub domain: String,
    pub records: Vec<DnsRecord>,
    pub resolver_comparison: Vec<ResolverResult>,
    pub consistent: bool,
    pub has_aaaa: bool,
    pub dnssec: bool,
}

pub struct DnsRecord {
    pub record_type: String,  // A / AAAA / CNAME / MX / NS / TXT / SOA
    pub value: String,
    pub ttl: u32,
}

pub struct ResolverResult {
    pub resolver: String,     // "8.8.8.8 (Google)"
    pub ip: String,
    pub latency_ms: f64,
}

// ========== Ping ==========

pub struct PingResult {
    pub target: String,
    pub resolved_ip: String,
    pub packets_sent: u32,
    pub packets_recv: u32,
    pub loss_pct: f64,
    pub rtt_min_ms: f64,
    pub rtt_avg_ms: f64,
    pub rtt_max_ms: f64,
    pub rtt_p95_ms: f64,
    pub rtt_stddev_ms: f64,
    pub hops: Vec<PingHop>,     // 每包结果（用于 ●/○ 可视化）
}

pub struct PingHop {
    pub seq: u32,
    pub success: bool,
    pub rtt_ms: Option<f64>,
}

// ========== Traceroute ==========

pub struct TraceResult {
    pub target: String,
    pub hops: Vec<TraceHop>,
    pub total_hops: u32,
}

pub struct TraceHop {
    pub hop: u32,
    pub ip: Option<String>,
    pub hostname: Option<String>,
    pub avg_ms: Option<f64>,
    pub loss_pct: f64,
    pub asn: Option<String>,       // "AS4134"
    pub org: Option<String>,       // "ChinaTelecom"
    pub geo: Option<String>,       // "Shanghai, CN"
}

// ========== Port Scan ==========

pub struct PortScanResult {
    pub target: String,
    pub ip: String,
    pub ports: Vec<PortInfo>,
    pub open_count: u32,
    pub closed_count: u32,
    pub filtered_count: u32,
    pub scan_duration: Duration,
}

pub struct PortInfo {
    pub port: u16,
    pub state: PortState,
    pub service: Option<String>,    // "HTTP" / "SSH" / "MySQL"
    pub banner: Option<String>,
}

pub enum PortState {
    Open,
    Closed,
    Filtered,
    Timeout,
}

// ========== HTTP ==========

pub struct HttpResult {
    pub url: String,
    pub final_url: String,
    pub status: u16,
    pub status_text: String,
    pub timing: HttpTiming,
    pub tls: Option<TlsInfo>,
    pub redirects: Vec<Redirect>,
    pub headers: HttpHeaders,
}

pub struct HttpTiming {
    pub dns_ms: f64,
    pub tcp_ms: f64,
    pub tls_ms: Option<f64>,
    pub ttfb_ms: f64,
    pub total_ms: f64,
}

pub struct TlsInfo {
    pub version: String,
    pub cipher: String,
    pub cert_issuer: String,
    pub cert_subject: String,
    pub cert_valid_from: String,
    pub cert_valid_to: String,
    pub cert_days_remaining: i64,
    pub cert_chain: Vec<CertEntry>,
}

pub struct Redirect {
    pub from: String,
    pub to: String,
    pub status: u16,
}

pub struct HttpHeaders {
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
    pub server: Option<String>,
    pub cache_control: Option<String>,
    pub cors: Option<CorsInfo>,
}

// ========== Speed Test ==========

pub struct SpeedResult {
    pub server: SpeedServer,
    pub download_mbps: f64,
    pub upload_mbps: f64,
    pub latency_ms: f64,
    pub jitter_ms: f64,
}

pub struct SpeedServer {
    pub name: String,
    pub location: String,
    pub distance_km: f64,
}

// ========== Check（综合）==========

pub struct CheckResult {
    pub target: String,
    pub ip: String,
    pub timestamp: DateTime<Utc>,
    pub score: u32,               // 0-100
    pub grade: String,            // Excellent / Good / Fair / Poor
    pub dns: ProbeResult<DnsResult>,
    pub ping: ProbeResult<PingResult>,
    pub trace: ProbeResult<TraceResult>,
    pub ports: ProbeResult<PortScanResult>,
    pub http: ProbeResult<HttpResult>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
    pub total_duration: Duration,
}

// ========== 输出格式 ==========

/// 统一输出枚举，由 output.rs 处理
pub enum Output {
    Terminal(CheckResult),    // Rich TUI 渲染
    Json(CheckResult),        // serde_json
    Markdown(CheckResult),    // Markdown 模板
}
```

### 3.5 错误处理设计

```rust
// error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetScopeError {
    #[error("DNS resolution failed for {target}: {reason}")]
    DnsFailed { target: String, reason: String },

    #[error("Host {target} is unreachable (100% packet loss)")]
    Unreachable { target: String },

    #[error("Connection to {target}:{port} timed out after {timeout}s")]
    ConnectionTimeout {
        target: String,
        port: u16,
        timeout: u64,
    },

    #[error("HTTP request failed: {0}")]
    HttpFailed(String),

    #[error("TLS error: {0}")]
    TlsError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Permission denied: {0}. Try running with sudo/root")]
    PermissionDenied(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

// 每个 core 模块返回 Result<T, NetScopeError>
// cmd 层 catch 错误，转为 ProbeStatus::Error，继续执行其他模块
```

### 3.6 异步编排（check 命令）

```rust
// cmd/check.rs — 核心编排逻辑

pub async fn run(target: &str, opts: &CheckOpts) -> Result<CheckResult> {
    let start = Instant::now();

    // ── Phase 1: DNS 解析（必须先完成） ──
    let dns_result = resolver::full_diagnosis(target, &opts.dns_resolvers).await;
    let ip = match &dns_result.data {
        Some(dns) => dns.records.iter()
            .find(|r| r.record_type == "A")
            .map(|r| r.value.clone())
            .unwrap_or_else(|| target.to_string()),
        None => target.to_string(), // 可能直接是 IP
    };

    // ── Phase 2: 并发探测（Ping + 端口 + 路由） ──
    let (ping_result, port_result, trace_result) = tokio::join!(
        pinger::tcp_ping(&ip, opts.ping_count, opts.ping_timeout),
        scanner::scan_top_ports(&ip, opts.top_ports, opts.port_concurrency),
        traceroute::trace(&ip, opts.trace_max_hops),
    );

    // ── Phase 3: HTTP 分析（依赖端口信息） ──
    let needs_http = port_result.as_ref()
        .ok()
        .map(|p| p.ports.iter().any(|pi| pi.state == PortState::Open && (pi.port == 80 || pi.port == 443)))
        .unwrap_or(false);

    let http_result = if needs_http {
        let scheme = if port_result.as_ref().ok()
            .map(|p| p.ports.iter().any(|pi| pi.port == 443 && pi.state == PortState::Open))
            .unwrap_or(false) { "https" } else { "http" };
        http_probe::analyze(&format!("{}://{}", scheme, target)).await
    } else {
        ProbeResult::skipped("No HTTP ports open")
    };

    // ── Phase 4: 综合评估 ──
    let (score, grade, warnings, suggestions) = scoring::evaluate(
        &dns_result, &ping_result, &trace_result, &port_result, &http_result,
    );

    Ok(CheckResult {
        target: target.to_string(),
        ip,
        timestamp: Utc::now(),
        score,
        grade,
        dns: dns_result,
        ping: ping_result,
        trace: trace_result,
        ports: port_result,
        http: http_result,
        warnings,
        suggestions,
        total_duration: start.elapsed(),
    })
}
```

### 3.7 输出渲染设计

```rust
// output.rs — 三种输出格式

/// 终端输出（默认）
pub fn render_terminal(result: &CheckResult) {
    // 使用 tabled + colored 渲染
    //
    // 1. 头部：目标、IP、时间、评分
    // 2. 分区：DNS / Ping / Route / Ports / HTTP
    // 3. 每个分区用表格 + 颜色编码
    // 4. 底部：warnings + suggestions
    //
    // 颜色规则：
    //   延迟 < 50ms  → 绿色
    //   延迟 50-150  → 黄色
    //   延迟 > 150   → 红色
    //   端口 open    → 绿色
    //   端口 closed  → 灰色
    //   丢包 > 0%    → 红色
}

/// JSON 输出
pub fn render_json(result: &CheckResult) -> String {
    serde_json::to_string_pretty(result).unwrap()
}

/// Markdown 输出
pub fn render_markdown(result: &CheckResult) -> String {
    // 模板渲染
    // # NetScope Diagnosis Report
    // ## DNS Resolution
    // | Record | Value | TTL |
    // ...
}
```

### 3.8 Ping 实现策略

```rust
// core/pinger.rs

/// TCP Ping（默认，无需 root）
/// 连接目标的常见端口（80/443），测量连接时间
/// 优点：不需要 root 权限
/// 缺点：目标必须有开放端口
pub async fn tcp_ping(target: &str, count: u32, timeout: Duration) -> ProbeResult<PingResult> {
    // 对每个包：
    //   1. TcpStream::connect_timeout(target:80)
    //   2. 记录 RTT
    //   3. 统计 min/avg/max/stddev/loss%
}

/// ICMP Ping（需要 root，feature gate）
/// 原始 socket 构造 ICMP Echo Request
/// 只在 root 权限时使用，否则 fallback 到 TCP
#[cfg(feature = "icmp")]
pub async fn icmp_ping(target: &str, count: u32, timeout: Duration) -> ProbeResult<PingResult> {
    // 使用 pnet 构造 ICMP 包
}

/// 自动选择：有 root 用 ICMP，否则 TCP
pub async fn auto_ping(target: &str, count: u32, timeout: Duration) -> ProbeResult<PingResult> {
    if is_root() {
        icmp_ping(target, count, timeout).await
    } else {
        tcp_ping(target, count, timeout).await
    }
}
```

### 3.9 端口扫描实现策略

```rust
// core/scanner.rs

/// TCP Connect 扫描（不需要 root）
/// 标准三次握手，SYN→SYN-ACK→RST
pub async fn scan_port(target: &str, port: u16, timeout: Duration) -> PortInfo {
    let addr = format!("{}:{}", target, port);
    match tokio::time::timeout(timeout, TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => PortInfo {
            port,
            state: PortState::Open,
            service: guess_service(port),
            banner: None,  // 可选：连接后读取 banner
        },
        Ok(Err(_)) => PortInfo { port, state: PortState::Closed, .. },
        Err(_) => PortInfo { port, state: PortState::Timeout, .. },
    }
}

/// 批量扫描（并发控制）
pub async fn scan_top_ports(
    target: &str,
    top: usize,
    concurrency: usize,
) -> ProbeResult<PortScanResult> {
    let ports = common_ports(top);  // 常用端口列表
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let mut handles = Vec::new();

    for port in ports {
        let sem = semaphore.clone();
        let target = target.to_string();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            scan_port(&target, port, Duration::from_secs(2)).await
        }));
    }

    // 收集结果...
}

/// 常用端口列表（按频率排序）
fn common_ports(top: usize) -> Vec<u16> {
    let ports = vec![
        80, 443, 22, 21, 25, 53, 110, 143, 993, 995,
        3306, 5432, 6379, 8080, 8443, 8888, 9090, 27017,
        // ... top 100 / top 1000
    ];
    ports.into_iter().take(top).collect()
}
```

### 3.10 HTTP 分析实现策略

```rust
// core/http_probe.rs

/// HTTP 全链路分析
/// 手动控制每个阶段的计时
pub async fn analyze(url: &str) -> ProbeResult<HttpResult> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .redirect(Policy::none())  // 手动追踪重定向
        .build()?;

    // 1. 手动 DNS 解析，记录耗时
    let dns_start = Instant::now();
    let addrs = tokio::net::lookup_host(host).await?;
    let dns_ms = dns_start.elapsed().as_secs_f64() * 1000.0;

    // 2. TCP 连接，记录耗时
    let tcp_start = Instant::now();
    let stream = TcpStream::connect(addr).await?;
    let tcp_ms = tcp_start.elapsed().as_secs_f64() * 1000.0;

    // 3. TLS 握手（HTTPS），记录耗时 + 证书信息
    let tls_start = Instant::now();
    let tls_info = if scheme == "https" {
        let connector = tokio_rustls::TlsConnector::from(tls_config);
        let stream = connector.connect(host, stream).await?;
        let tls_ms = tls_start.elapsed().as_secs_f64() * 1000.0;
        // 提取证书信息...
        Some(TlsInfo { ... })
    } else {
        None
    };

    // 4. 发送请求，记录 TTFB
    let req_start = Instant::now();
    let response = client.get(url).send().await?;
    let ttfb_ms = req_start.elapsed().as_secs_f64() * 1000.0;

    // 5. 读取完整响应
    let body = response.bytes().await?;
    let total_ms = (dns_start.elapsed()).as_secs_f64() * 1000.0;

    // 6. 提取 headers
    let headers = extract_headers(response.headers());

    Ok(HttpResult {
        url: url.to_string(),
        final_url: response.url().to_string(),
        status: response.status().as_u16(),
        timing: HttpTiming { dns_ms, tcp_ms, tls_ms, ttfb_ms, total_ms },
        tls: tls_info,
        redirects: track_redirects(&client, url).await,
        headers,
    })
}
```

---

## 四、评分系统

```rust
// scoring.rs

/// 5 维度加权评分
pub fn evaluate(
    dns: &ProbeResult<DnsResult>,
    ping: &ProbeResult<PingResult>,
    trace: &ProbeResult<TraceResult>,
    ports: &ProbeResult<PortScanResult>,
    http: &ProbeResult<HttpResult>,
) -> (u32, String, Vec<String>, Vec<String>) {
    let mut score: f64 = 100.0;
    let mut warnings = Vec::new();
    let mut suggestions = Vec::new();

    // ── DNS（权重 15%）──
    if let Some(dns_data) = &dns.data {
        if !dns_data.consistent {
            score -= 15.0 * 0.15;
            warnings.push("DNS resolvers disagree — possible hijacking".into());
        }
        if !dns_data.dnssec {
            score -= 5.0 * 0.15;
            suggestions.push("Consider enabling DNSSEC".into());
        }
    } else {
        score -= 100.0 * 0.15;
    }

    // ── 连通性（权重 25%）──
    if let Some(ping_data) = &ping.data {
        if ping_data.loss_pct > 5.0 {
            score -= 30.0 * 0.25;
            warnings.push(format!("{}% packet loss", ping_data.loss_pct));
        }
        if ping_data.rtt_avg_ms > 200.0 {
            score -= 20.0 * 0.25;
        } else if ping_data.rtt_avg_ms > 100.0 {
            score -= 10.0 * 0.25;
        }
    } else {
        score -= 100.0 * 0.25;
    }

    // ── 路由（权重 15%）──
    // ── HTTP（权重 30%）──
    // ── 安全/TLS（权重 15%）──
    // ... 同理

    let grade = match score as u32 {
        90..=100 => "Excellent",
        75..=89  => "Good",
        60..=74  => "Fair",
        _        => "Poor",
    };

    (score as u32, grade.to_string(), warnings, suggestions)
}
```

---

## 五、CLI 参数设计（clap derive）

```rust
// cli.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "netscope",
    version,
    about = "Modern network diagnostics — one command, full picture",
    long_about = "NetScope is a lightweight CLI tool that replaces ping + traceroute + dig + curl + speedtest with a single command."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,

    /// Output as Markdown
    #[arg(long, global = true)]
    pub md: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Global timeout in seconds
    #[arg(long, default_value = "10", global = true)]
    pub timeout: u64,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Quiet mode (results only)
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// ★ Full-chain network diagnosis (DNS + Ping + Route + Ports + HTTP)
    Check {
        /// Target hostname, IP, or URL
        target: String,

        /// Number of ping packets
        #[arg(long, default_value = "5")]
        ping_count: u32,

        /// Top N ports to scan
        #[arg(long, default_value = "20")]
        top_ports: usize,
    },

    /// Enhanced ping with statistics and colored output
    Ping {
        /// Target hostname or IP
        target: String,

        /// Number of packets to send
        #[arg(short = 'c', long, default_value = "10")]
        count: u32,

        /// Timeout per packet in seconds
        #[arg(short = 'W', long, default_value = "3")]
        timeout: u64,
    },

    /// Traceroute with ASN and geo information
    Trace {
        /// Target hostname or IP
        target: String,

        /// Maximum hops
        #[arg(long, default_value = "30")]
        max_hops: u32,
    },

    /// DNS diagnosis with multi-resolver comparison
    Dns {
        /// Domain name
        domain: String,
    },

    /// TCP port scan with service detection
    Port {
        /// Target hostname or IP
        target: String,

        /// Specific ports (comma-separated)
        #[arg(long, value_delimiter = ',')]
        ports: Option<Vec<u16>>,

        /// Scan top N common ports
        #[arg(long, default_value = "20")]
        top: usize,

        /// Concurrent connections
        #[arg(long, default_value = "100")]
        concurrency: usize,
    },

    /// HTTP request analysis with timing breakdown
    Http {
        /// URL to analyze
        url: String,
    },

    /// Bandwidth speed test
    Speed,

    /// Show your public IP and location
    Myip,

    /// Generate a full Markdown diagnosis report
    Report {
        /// Target hostname, IP, or URL
        target: String,

        /// Output file path (default: stdout)
        #[arg(short = 'o', long)]
        output: Option<String>,
    },
}
```

---

## 六、终端输出设计

### 6.1 check 命令输出布局

```
╔══════════════════════════════════════════════════════════╗
║  🔭 NetScope Diagnosis                                   ║
║  Target:  example.com (93.184.216.34)                    ║
║  Time:    2026-05-02 23:14:00 CST                        ║
║  Score:   85/100  ████████████████████░░░░  ⚠ Good       ║
╠══════════════════════════════════════════════════════════╣
║                                                          ║
║  📍 DNS Resolution                                       ║
║  ┌────────┬──────────────────────────────────┬─────┐     ║
║  │ Type   │ Value                            │ TTL │     ║
║  ├────────┼──────────────────────────────────┼─────┤     ║
║  │ A      │ 93.184.216.34                    │ 3600│     ║
║  │ AAAA   │ 2606:2800:220:1:248:1893:25c8:.. │ 3600│     ║
║  │ NS     │ a.iana-servers.net               │ 86400│    ║
║  └────────┴──────────────────────────────────┴─────┘     ║
║  Resolvers: ✅ All 4 agree                               ║
║                                                          ║
║  📶 Connectivity                                         ║
║  Packets: ●●●●●  5/5 received (0% loss)                 ║
║  RTT:     min=11.2  avg=12.3  max=14.1  p95=13.8  ms    ║
║                                                          ║
║  🛤️  Route (14 hops)                                      ║
║  Hop  IP              ASN       Geo           Avg   Bar  ║
║   1   192.168.1.1      -        Local         1.2ms ██   ║
║   2   10.0.0.1         -        Local         3.4ms ███  ║
║   3   202.97.12.34    AS4134   Shanghai,CN    8.1ms █████║
║  ...                                                     ║
║  14   93.184.216.34   AS15133  Ashburn,US    45.2ms █████║
║                                                          ║
║  🔍 Port Scan (20 ports, 1.2s)                           ║
║  Port   State    Service                                  ║
║   22    closed   SSH                                      ║
║   80    open     HTTP       ←                              ║
║   443   open     HTTPS      ←                              ║
║   3306  closed   MySQL                                     ║
║                                                          ║
║  🌐 HTTP Analysis (https://example.com)                   ║
║  DNS ████░░░░░░░░░░░░░░  45ms                            ║
║  TCP ██░░░░░░░░░░░░░░░░  12ms                            ║
║  TLS ████████░░░░░░░░░░  89ms                            ║
║  TTFB████████████████░░  156ms                           ║
║  Total: 234ms | Status: 200 OK                           ║
║  TLS: v1.3 | AES_256_GCM | Expires: 2027-01-15          ║
║                                                          ║
╠══════════════════════════════════════════════════════════╣
║  ⚠️  Warnings                                             ║
║  • TLS certificate expires in 258 days                   ║
║                                                          ║
║  💡 Suggestions                                           ║
║  • Consider enabling HTTP/2 for better performance       ║
╚══════════════════════════════════════════════════════════╝
```

### 6.2 颜色规则

```rust
// 延迟着色
fn color_latency(ms: f64) -> Color {
    if ms < 50.0 { Color::Green }
    else if ms < 150.0 { Color::Yellow }
    else { Color::Red }
}

// 状态着色
fn color_state(state: &str) -> Color {
    match state {
        "open" => Color::Green,
        "closed" => Color::BrightBlack,  // 灰色
        "filtered" => Color::Yellow,
        _ => Color::Red,
    }
}

// 丢包着色
fn color_loss(pct: f64) -> Color {
    if pct == 0.0 { Color::Green }
    else if pct < 5.0 { Color::Yellow }
    else { Color::Red }
}

// 评分着色
fn color_score(score: u32) -> Color {
    match score {
        90..=100 => Color::Green,
        75..=89 => Color::Yellow,
        60..=74 => Color::BrightYellow,
        _ => Color::Red,
    }
}
```

### 6.3 进度指示

```rust
// check 命令执行时显示进度
// 使用 indicatif 的 spinner

🔍 Resolving DNS...          [====        ] 2/5
📶 Testing connectivity...   [========    ] 4/5
🔍 Scanning ports...         [============] 5/5
```

---

## 七、配置文件

### 7.1 内置默认值

```rust
// config.rs

pub struct Config {
    pub ping_count: u32,          // 5
    pub ping_timeout: Duration,   // 3s
    pub trace_max_hops: u32,      // 30
    pub trace_timeout: Duration,  // 5s per hop
    pub port_concurrency: usize,  // 100
    pub port_timeout: Duration,   // 2s
    pub http_timeout: Duration,   // 10s
    pub top_ports: usize,         // 20
    pub dns_resolvers: Vec<String>,
    pub speed_download_mb: usize, // 10
    pub speed_upload_mb: usize,   // 5
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ping_count: 5,
            ping_timeout: Duration::from_secs(3),
            trace_max_hops: 30,
            trace_timeout: Duration::from_secs(5),
            port_concurrency: 100,
            port_timeout: Duration::from_secs(2),
            http_timeout: Duration::from_secs(10),
            top_ports: 20,
            dns_resolvers: vec![
                "8.8.8.8".into(),      // Google
                "1.1.1.1".into(),      // Cloudflare
                "223.5.5.5".into(),    // AliDNS
                "119.29.29.29".into(), // DNSPod
            ],
            speed_download_mb: 10,
            speed_upload_mb: 5,
        }
    }
}
```

### 7.2 用户配置文件（可选）

位置：`~/.config/netscope/config.toml`

```toml
[ping]
count = 10
timeout_secs = 5

[dns]
resolvers = ["8.8.8.8", "1.1.1.1", "223.5.5.5"]

[ports]
concurrency = 200
top = 100

[http]
timeout_secs = 15

[output]
default = "terminal"  # terminal | json | md
```

---

## 八、发布与分发

### 8.1 构建

```bash
# 开发构建
cargo build

# Release 构建（优化 + strip）
cargo build --release
strip target/release/netscope

# 交叉编译
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-pc-windows-gnu
```

### 8.2 CI/CD（GitHub Actions）

```yaml
# .github/workflows/release.yml
# 触发：tag push (v*)
# 矩阵构建：linux/macos/windows × x86_64/aarch64
# 产物：netscope-{os}-{arch}.tar.gz
# 自动发布到 GitHub Releases
```

### 8.3 安装方式

```bash
# cargo install
cargo install netscope

# Homebrew
brew install netscope

# 直接下载
curl -L https://github.com/.../netscope-linux-x86_64.tar.gz | tar xz
sudo mv netscope /usr/local/bin/

# Arch Linux (AUR)
yay -S netscope
```

---

## 九、开发路线图

### Phase 1 — MVP (v0.1.0) ← 首要目标
- [ ] `check` 命令（5 模块编排 + 评分 + 终端输出）
- [ ] `ping` 命令（TCP ping + 彩色统计）
- [ ] `dns` 命令（多解析器对比）
- [ ] `port` 命令（TCP connect 扫描）
- [ ] `http` 命令（耗时分解 + TLS 信息）
- [ ] JSON 导出
- [ ] README + 基本文档

### Phase 2 — 增强 (v0.2.0)
- [ ] `trace` 命令（系统 traceroute 解析 + ASN）
- [ ] `speed` 命令
- [ ] `myip` 命令
- [ ] Markdown 导出
- [ ] 用户配置文件

### Phase 3 — 完善 (v0.3.0)
- [ ] `report` 命令
- [ ] GeoLite2 地理位置（feature gate）
- [ ] Shell 补全
- [ ] 历史记录对比

### Phase 4 — 发布
- [ ] CI/CD + 交叉编译
- [ ] crates.io 发布
- [ ] GitHub Release + Homebrew

---

## 十、竞品对比

| 维度 | ping | mtr | dig | curl | Wireshark | **NetScope** |
|---|---|---|---|---|---|---|
| 一键诊断 | ❌ | ❌ | ❌ | ❌ | ❌ | **✅** |
| 单二进制 | ✅ | ✅ | ✅ | ✅ | ❌ | **✅** |
| 无需 root | ✅ | ❌ | ✅ | ✅ | ❌ | **✅** |
| 终端 TUI | ❌ | ❌ | ❌ | ❌ | GUI | **✅** |
| JSON 输出 | ❌ | ❌ | ❌ | ✅ | ❌ | **✅** |
| 评分系统 | ❌ | ❌ | ❌ | ❌ | ❌ | **✅** |
| 启动速度 | <1ms | <10ms | <10ms | <10ms | >1s | **<50ms** |
| 二进制大小 | 50KB | 200KB | 100KB | 200KB | 100MB+ | **<5MB** |
