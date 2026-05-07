use clap::{Parser, Subcommand};
use crate::i18n::Language;

#[derive(Parser)]
#[command(
    name = "netscope",
    version,
    about = "Modern network diagnostics — one command, full picture",
    long_about = "NetScope is a lightweight CLI tool that replaces ping + traceroute +\n\
                   dig + curl + speedtest with a single command.\n\n\
                   Quick start:\n                     netscope check example.com"
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

    /// Language: en/zh (English/Chinese)
    #[arg(long, default_value = "en", global = true)]
    pub lang: Language,
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

    /// ★ DNS Optimizer - Find the fastest DNS server
    DnsOptimizer {
        /// Target domain to test DNS resolution (default: www.google.com)
        #[arg(default_value = "www.google.com")]
        domain: Option<String>,
    },

    /// ★ Mirror Speed Test - Test and compare mirror speeds
    Mirror {
        /// Mirror category to test (github, docker, pypi, npm, crates, go)
        /// If not specified, tests all categories
        category: Option<String>,
    },

    /// ★ Site Entry Tester - Test multiple entry points for any website
    Site {
        /// Site name (predefined) or URL to test
        /// If not specified, tests all supported sites
        target: Option<String>,
    },

    /// ★ One-click DNS optimization - Find and configure the best DNS
    Optimize,

    /// ★ Batch site testing - Test multiple URLs from file or arguments
    Batch {
        /// File containing URLs to test (one per line)
        #[arg(short = 'f', long)]
        file: Option<String>,

        /// URLs to test
        #[arg(trailing_var_arg = true)]
        urls: Vec<String>,
    },

    /// ★ TCP connection analyzer - Analyze TCP handshake and TLS
    Tcp {
        /// Target hostname or IP (e.g., google.com:443)
        target: String,
    },

    /// ★ Mobile-friendly output mode
    Mobile {
        /// URL to check
        url: String,
    },

    /// ★ Interactive mode - Guided network diagnosis
    Interactive,

    /// ★ Configuration management
    Config {
        /// Config action: show, set, add, remove, clear
        action: String,

        /// Configuration key or site name
        key: Option<String>,

        /// Configuration value
        value: Option<String>,
    },
}