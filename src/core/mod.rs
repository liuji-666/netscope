pub mod resolver;
pub mod pinger;
pub mod traceroute;
pub mod scanner;
pub mod http_probe;
pub mod ipinfo;
pub mod speed_test;
pub mod dns_optimizer;
pub mod mirror;
pub mod site_tester;
pub mod utils;
pub mod config;

pub use utils::{guess_service, resolve_ip, format_socket_addr};

use serde::{Serialize, Deserialize};

// ═══════════════════════════════════════════
//  DNS 模块数据结构
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsResult {
    pub domain: String,
    pub records: Vec<DnsRecord>,
    pub resolver_comparison: Vec<ResolverResult>,
    pub consistent: bool,
    pub has_aaaa: bool,
    pub dnssec: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    pub record_type: String,
    pub value: String,
    pub ttl: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverResult {
    pub resolver: String,
    pub ip: String,
    pub latency_ms: f64,
}

// ═══════════════════════════════════════════
//  Ping 模块数据结构
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub hops: Vec<PingHop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingHop {
    pub seq: u32,
    pub success: bool,
    pub rtt_ms: Option<f64>,
}

// ═══════════════════════════════════════════
//  Traceroute 模块数据结构
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceResult {
    pub target: String,
    pub hops: Vec<TraceHop>,
    pub total_hops: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceHop {
    pub hop: u32,
    pub ip: Option<String>,
    pub hostname: Option<String>,
    pub avg_ms: Option<f64>,
    pub loss_pct: f64,
    pub asn: Option<String>,
    pub org: Option<String>,
    pub geo: Option<String>,
}

// ═══════════════════════════════════════════
//  Port Scanner 模块数据结构
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortScanResult {
    pub target: String,
    pub ip: String,
    pub ports: Vec<PortInfo>,
    pub open_count: u32,
    pub closed_count: u32,
    pub filtered_count: u32,
    pub scan_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub state: PortState,
    pub service: Option<String>,
    pub banner: Option<String>,
}

// ═══════════════════════════════════════════
//  HTTP 模块数据结构
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpTiming {
    pub dns_ms: f64,
    pub tcp_ms: f64,
    pub tls_ms: Option<f64>,
    pub ttfb_ms: f64,
    pub total_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertEntry {
    pub subject: String,
    pub issuer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redirect {
    pub from: String,
    pub to: String,
    pub status: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpHeaders {
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
    pub server: Option<String>,
    pub cache_control: Option<String>,
    pub cors: Option<CorsInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsInfo {
    pub allow_origin: Option<String>,
    pub allow_methods: Option<String>,
    pub allow_headers: Option<String>,
}

// ═══════════════════════════════════════════
//  Speed Test 模块数据结构
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedResult {
    pub server: SpeedServer,
    pub download_mbps: f64,
    pub upload_mbps: f64,
    pub latency_ms: f64,
    pub jitter_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedServer {
    pub name: String,
    pub location: String,
    pub distance_km: f64,
}

// ═══════════════════════════════════════════
//  IP Info 模块数据结构
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpInfoResult {
    pub ip: String,
    pub hostname: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub org: Option<String>,
    pub timezone: Option<String>,
}