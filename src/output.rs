use crate::core::HttpResult;
use crate::core::PingResult;
use crate::core::DnsResult;
use crate::core::PortScanResult;
use crate::core::PortState;
use crate::core::TraceResult;
use crate::error::{ProbeResult, ProbeStatus};
use crate::i18n::I18n;
use crate::scoring::ScoreResult;
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct ReportData<'a> {
    pub target: &'a str,
    pub ip: &'a str,
    pub score: &'a ScoreResult,
    pub dns: &'a ProbeResult<DnsResult>,
    pub ping: &'a ProbeResult<PingResult>,
    pub trace: &'a ProbeResult<TraceResult>,
    pub ports: &'a ProbeResult<PortScanResult>,
    pub http: &'a ProbeResult<HttpResult>,
    pub i18n: &'a I18n,
}

pub fn print_check_header(target: &str, ip: &str, score: &ScoreResult, i18n: &I18n) {
    let score_color = match score.score {
        90..=100 => "green",
        75..=89 => "yellow",
        60..=74 => "bright yellow",
        _ => "red",
    };

    let bar = render_progress_bar(score.score, 25);
    let grade_display = match score.score {
        90..=100 => format!("✅ {}", score.grade),
        75..=89 => format!("⚠️  {}", score.grade),
        60..=74 => format!("⚠️  {}", score.grade),
        _ => format!("❌ {}", score.grade),
    };

    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════╗"
            .cyan()
    );
    println!(
        "{}  {}",
        "║".cyan(),
        "🔭 NetScope Diagnosis".bold()
    );
    println!(
        "{}  {} {} ({})",
        "║".cyan(),
        format!("{}:", i18n.t("target")).bold(),
        target,
        ip
    );
    println!(
        "{}  {} {}",
        "║".cyan(),
        format!("{}:", i18n.t("time")).bold(),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S %Z")
    );

    let score_line = format!("{}/100  {}  {}", score.score, bar, grade_display);
    let colored_score = match score_color {
        "green" => score_line.green().bold(),
        "yellow" => score_line.yellow().bold(),
        "bright yellow" => score_line.bright_yellow().bold(),
        _ => score_line.red().bold(),
    };
    println!("{}  {} {}", "║".cyan(), format!("{}:", i18n.t("score")).bold(), colored_score);

    println!(
        "{}",
        "╠══════════════════════════════════════════════════════════════╣"
            .cyan()
    );
}

pub fn print_dns_section(result: &ProbeResult<DnsResult>, i18n: &I18n) {
    println!("{}  {}", "║".cyan(), format!("📍 {}", i18n.t("dns_resolution")).bold());

    match &result.data {
        Some(dns) => {
            if dns.records.is_empty() {
                println!("{}  {}", "║".cyan(), format!("  {}", i18n.t("dns_no_records")).red());
            } else {
                for record in &dns.records {
                    let rtype = format!("{:>5}", record.record_type).cyan();
                    println!("{}    {}  {}", "║".cyan(), rtype, record.value);
                }
            }

            if !dns.resolver_comparison.is_empty() {
                let label = if dns.consistent {
                    i18n.t("dns_all_agree").green()
                } else {
                    i18n.t("dns_disagree").red().bold()
                };
                println!(
                    "{}  {}",
                    "║".cyan(),
                    label
                );
            }
        }
        None => {
            print_probe_error(result, i18n);
        }
    }
    println!("{}  {}", "║".cyan(), "");
}

pub fn print_ping_section(result: &ProbeResult<PingResult>, i18n: &I18n) {
    println!("{}  {}", "║".cyan(), format!("📶 {}", i18n.t("connectivity")).bold());

    match &result.data {
        Some(ping) => {
            let hops_display: String = ping
                .hops
                .iter()
                .map(|h| if h.success { "●" } else { "○" })
                .collect();
            let loss_color = if ping.loss_pct == 0.0 {
                hops_display.green()
            } else if ping.loss_pct < 10.0 {
                hops_display.yellow()
            } else {
                hops_display.red()
            };
            let received_str = i18n.t("ping_received")
                .replace("{}", &ping.packets_recv.to_string())
                .replace("{}", &ping.packets_sent.to_string());
            let loss_str = i18n.t("ping_loss").replace("{}", &format!("{:.1}", ping.loss_pct));
            println!(
                "{}    {}  {} ({})",
                "║".cyan(),
                loss_color,
                received_str,
                loss_str
            );

            let color_fn = color_latency_fn(ping.rtt_avg_ms);
            let rtt_line = i18n.t("ping_rtt")
                .replace("{}", &format!("{:.1}", ping.rtt_min_ms))
                .replace("{}", &format!("{:.1}", ping.rtt_avg_ms))
                .replace("{}", &format!("{:.1}", ping.rtt_max_ms))
                .replace("{}", &format!("{:.1}", ping.rtt_p95_ms))
                .replace("{}", &format!("{:.1}", ping.rtt_stddev_ms));
            println!("{}    {}", "║".cyan(), color_fn(rtt_line));
        }
        None => {
            print_probe_error(result, i18n);
        }
    }
    println!("{}  {}", "║".cyan(), "");
}

pub fn print_trace_section(result: &ProbeResult<TraceResult>, i18n: &I18n) {
    println!("{}  {}", "║".cyan(), format!("🛤️ {}", i18n.t("route")).bold());

    match &result.data {
        Some(trace) => {
            if trace.hops.is_empty() {
                println!("{}    {}", "║".cyan(), i18n.t("route_no_hops").dimmed());
            } else {
                println!(
                    "{}    {}",
                    "║".cyan(),
                    format!(
                        "{:<4} {:<18} {:<8} {:<6} {}",
                        i18n.t("route_hop"), i18n.t("route_ip"), i18n.t("route_avg_ms"), i18n.t("route_loss"), i18n.t("route_bar")
                    )
                    .dimmed()
                );

                for hop in &trace.hops {
                    let ip_str = hop.ip.as_deref().unwrap_or("*");
                    let avg_str = hop
                        .avg_ms
                        .map(|v| format!("{:.1}", v))
                        .unwrap_or_else(|| "-".to_string());
                    let loss_str = if hop.loss_pct > 0.0 {
                        format!("{:.0}%", hop.loss_pct).red().to_string()
                    } else {
                        "0%".to_string()
                    };
                    let bar = hop
                        .avg_ms
                        .map(|v| render_latency_bar(v, 15))
                        .unwrap_or_else(|| "".to_string());

                    let line = format!(
                        "  {:<4} {:<18} {:<8} {:<6} {}",
                        hop.hop, ip_str, avg_str, loss_str, bar
                    );

                    if let Some(avg) = hop.avg_ms {
                        if avg > 150.0 {
                            println!("{}    {}", "║".cyan(), line.red());
                        } else if avg > 50.0 {
                            println!("{}    {}", "║".cyan(), line.yellow());
                        } else {
                            println!("{}    {}", "║".cyan(), line);
                        }
                    } else {
                        println!("{}    {}", "║".cyan(), line.dimmed());
                    }
                }
            }
        }
        None => {
            print_probe_error(result, i18n);
        }
    }
    println!("{}  {}", "║".cyan(), "");
}

pub fn print_port_section(result: &ProbeResult<PortScanResult>, i18n: &I18n) {
    println!("{}  {}", "║".cyan(), format!("🔍 {}", i18n.t("port_scan")).bold());

    match &result.data {
        Some(scan) => {
            let timeout_count = scan.ports.iter().filter(|p| p.state == PortState::Timeout).count() as u32;
            let summary = i18n.t("port_summary")
                .replace("{}", &scan.open_count.to_string())
                .replace("{}", &scan.closed_count.to_string())
                .replace("{}", &scan.filtered_count.to_string())
                .replace("{}", &timeout_count.to_string())
                .replace("{}", &scan.scan_duration_ms.to_string());
            println!("{}    {}", "║".cyan(), summary.dimmed());

            for port_info in &scan.ports {
                if port_info.state == PortState::Closed {
                    continue;
                }
                let state_str = match port_info.state {
                    PortState::Open => i18n.t("port_open").green().bold().to_string(),
                    PortState::Closed => i18n.t("port_closed").dimmed().to_string(),
                    PortState::Filtered => i18n.t("port_filtered").yellow().to_string(),
                    PortState::Timeout => i18n.t("port_timeout").red().to_string(),
                };
                let service = port_info.service.as_deref().unwrap_or("-");
                let line = format!(
                    "  {:<6} {:<10} {}",
                    port_info.port, state_str, service
                );
                println!("{}    {}", "║".cyan(), line);
            }
        }
        None => {
            print_probe_error(result, i18n);
        }
    }
    println!("{}  {}", "║".cyan(), "");
}

pub fn print_http_section(result: &ProbeResult<HttpResult>, i18n: &I18n) {
    println!("{}  {}", "║".cyan(), format!("🌐 {}", i18n.t("http_analysis")).bold());

    match &result.data {
        Some(http) => {
            let status_color = if http.status < 300 {
                "green"
            } else if http.status < 400 {
                "yellow"
            } else {
                "red"
            };
            let status_line = format!(
                "{}: {} {}",
                i18n.t("http_status"), http.status,
                http.status_text
            );
            let colored_status = match status_color {
                "green" => status_line.green(),
                "yellow" => status_line.yellow(),
                _ => status_line.red(),
            };
            println!("{}    {}", "║".cyan(), colored_status);

            println!("{}    {}", "║".cyan(), format!("{}:", i18n.t("http_timing")).dimmed());
            print_timing_bar(i18n, "DNS", http.timing.dns_ms, http.timing.total_ms);
            print_timing_bar(i18n, "TCP", http.timing.tcp_ms, http.timing.total_ms);
            if let Some(tls_ms) = http.timing.tls_ms {
                print_timing_bar(i18n, "TLS", tls_ms, http.timing.total_ms);
            }
            print_timing_bar(i18n, "TTFB", http.timing.ttfb_ms, http.timing.total_ms);
            println!(
                "{}    {}  {}",
                "║".cyan(),
                format!("{}:", i18n.t("http_total")).bold(),
                format!("{:.0}ms", http.timing.total_ms).cyan()
            );

            if let Some(ct) = &http.headers.content_type {
                println!("{}    {}  {}", "║".cyan(), format!("{}:", i18n.t("http_type")).dimmed(), ct);
            }
            if let Some(server) = &http.headers.server {
                println!("{}    {}  {}", "║".cyan(), format!("{}:", i18n.t("http_server")).dimmed(), server);
            }

            if let Some(tls) = &http.tls {
                println!(
                    "{}    {}  {} | {}",
                    "║".cyan(),
                    format!("{}:", i18n.t("http_tls")).dimmed(),
                    tls.version,
                    tls.cert_subject
                );
            }

            if !http.redirects.is_empty() {
                println!(
                    "{}    {}  {} {}",
                    "║".cyan(),
                    format!("{}:", i18n.t("http_redirects")).dimmed(),
                    http.redirects.len(),
                    i18n.t("http_redirects")
                );
            }
        }
        None => {
            print_probe_error(result, i18n);
        }
    }
    println!("{}  {}", "║".cyan(), "");
}

pub fn print_footer(score: &ScoreResult, i18n: &I18n) {
    println!(
        "{}",
        "╠══════════════════════════════════════════════════════════════╣"
            .cyan()
    );

    let warnings: Vec<_> = score.diagnosis.iter()
        .filter(|d| d.severity == crate::scoring::Severity::Critical || d.severity == crate::scoring::Severity::Warning)
        .collect();
    
    let suggestions: Vec<_> = score.diagnosis.iter()
        .filter(|d| d.severity == crate::scoring::Severity::Info)
        .collect();

    if !warnings.is_empty() {
        println!("{}  {}", "║".cyan(), format!("⚠️  {}", i18n.t("warnings")).bold().yellow());
        for w in warnings {
            println!("{}    {} {}", "║".cyan(), "•".yellow(), w.issue);
        }
    }

    if !suggestions.is_empty() {
        println!("{}  {}", "║".cyan(), format!("💡 {}", i18n.t("suggestions")).bold().cyan());
        for s in suggestions {
            println!("{}    {} {}", "║".cyan(), "•".cyan(), s.solution);
        }
    }

    if !score.diagnosis.is_empty() {
        println!("{}  {}", "║".cyan(), "🔧 诊断分析".bold().green());
        for (i, item) in score.diagnosis.iter().enumerate() {
            let severity_color = match item.severity {
                crate::scoring::Severity::Critical => "red",
                crate::scoring::Severity::Warning => "yellow",
                crate::scoring::Severity::Info => "cyan",
            };
            let severity_label = match item.severity {
                crate::scoring::Severity::Critical => "严重",
                crate::scoring::Severity::Warning => "警告",
                crate::scoring::Severity::Info => "信息",
            };
            
            let category_line = format!("{}. [{}] {} - {}", i + 1, severity_label, item.category, item.issue);
            let colored_category = match severity_color {
                "red" => category_line.red().bold(),
                "yellow" => category_line.yellow().bold(),
                _ => category_line.cyan(),
            };
            println!("{}    {}", "║".cyan(), colored_category);
            println!("{}      📌 {}", "║".cyan(), format!("原因: {}", item.cause).dimmed());
            println!("{}      💡 {}", "║".cyan(), format!("解决方案: {}", item.solution).green());
            println!("{}      ⚡ {}", "║".cyan(), format!("影响: {}", item.impact).yellow().dimmed());
        }
    }

    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════╝"
            .cyan()
    );
}

fn print_probe_error<T: serde::Serialize>(result: &ProbeResult<T>, i18n: &I18n) {
    let msg = match &result.error {
        Some(e) => e.clone(),
        None => i18n.t("unknown_error").to_string(),
    };
    match result.status {
        ProbeStatus::Error => println!("{}    {}", "║".cyan(), format!("❌ {}", msg).red()),
        ProbeStatus::Timeout => println!("{}    {}", "║".cyan(), format!("⏱️  {}", i18n.t("timeout")).yellow()),
        ProbeStatus::Skipped => println!("{}    {}", "║".cyan(), format!("⏭️  {}", i18n.t("skipped")).dimmed()),
        _ => {}
    }
}

fn render_progress_bar(value: u32, width: usize) -> String {
    let filled = (value as f64 / 100.0 * width as f64) as usize;
    let empty = width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn render_latency_bar(ms: f64, max_width: usize) -> String {
    let blocks = ((ms / 200.0) * max_width as f64).min(max_width as f64) as usize;
    let color_fn = color_latency_fn(ms);
    color_fn("█".repeat(blocks)).to_string()
}

fn color_latency_fn(ms: f64) -> fn(String) -> colored::ColoredString {
    if ms < 50.0 {
        |s: String| s.green()
    } else if ms < 150.0 {
        |s: String| s.yellow()
    } else {
        |s: String| s.red()
    }
}

fn print_timing_bar(_i18n: &I18n, label: &str, ms: f64, total_ms: f64) {
    let pct = if total_ms > 0.0 { ms / total_ms } else { 0.0 };
    let bar_width = 20;
    let filled = (pct * bar_width as f64) as usize;
    let empty = bar_width - filled;

    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
    let colored_bar = if ms < 50.0 {
        bar.green()
    } else if ms < 200.0 {
        bar.yellow()
    } else {
        bar.red()
    };

    println!(
        "{}    {:<6} {} {:>6.0}ms",
        "║".cyan(),
        label.dimmed(),
        colored_bar,
        ms
    );
}

pub fn print_json<T: serde::Serialize>(data: &T) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("JSON serialization error: {}", e),
    }
}

pub fn print_markdown(data: &ReportData) {
    let i18n = data.i18n;
    
    println!("# NetScope Diagnosis Report");
    println!();
    println!("**{}:** {} ({})", i18n.t("target"), data.target, data.ip);
    println!(
        "**{}:** {}",
        i18n.t("time"),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S %Z")
    );
    println!("**{}:** {}/100 — {}", i18n.t("score"), data.score.score, data.score.grade);
    println!();

    println!("## {}", i18n.t("dns_resolution"));
    println!();
    if let Some(dns_data) = &data.dns.data {
        println!("| {} | {} |", i18n.t("route_hop"), i18n.t("route_ip"));
        println!("|------|-------|");
        for r in &dns_data.records {
            println!("| {} | {} |", r.record_type, r.value);
        }
        println!();
        if dns_data.consistent {
            println!("✅ {}", i18n.t("dns_all_agree"));
        } else {
            println!("⚠️ {}", i18n.t("warn_dns_hijack"));
        }
    }
    println!();

    println!("## {}", i18n.t("connectivity"));
    println!();
    if let Some(ping_data) = &data.ping.data {
        println!("- **{}:** {:.1}%", i18n.t("ping_loss"), ping_data.loss_pct);
        println!(
            "- **RTT:** min={:.1} avg={:.1} max={:.1} ms",
            ping_data.rtt_min_ms, ping_data.rtt_avg_ms, ping_data.rtt_max_ms
        );
    }
    println!();

    println!("## {}", i18n.t("port_scan"));
    println!();
    if let Some(scan_data) = &data.ports.data {
        println!("| {} | {} | {} |", i18n.t("route_hop"), i18n.t("route_ip"), i18n.t("http_server"));
        println!("|------|-------|---------|");
        for p in &scan_data.ports {
            if p.state == PortState::Closed {
                continue;
            }
            let state = match p.state {
                PortState::Open => i18n.t("port_open"),
                PortState::Closed => i18n.t("port_closed"),
                PortState::Filtered => i18n.t("port_filtered"),
                PortState::Timeout => i18n.t("port_timeout"),
            };
            println!(
                "| {} | {} | {} |",
                p.port,
                state,
                p.service.as_deref().unwrap_or("-")
            );
        }
    }
    println!();

    println!("## {}", i18n.t("http_analysis"));
    println!();
    if let Some(http_data) = &data.http.data {
        println!("- **{}:** {} {}", i18n.t("http_status"), http_data.status, http_data.status_text);
        println!("- **{}:** {:.0}ms", i18n.t("http_total"), http_data.timing.total_ms);
        println!("  - DNS: {:.0}ms", http_data.timing.dns_ms);
        println!("  - TCP: {:.0}ms", http_data.timing.tcp_ms);
        if let Some(tls) = http_data.timing.tls_ms {
            println!("  - TLS: {:.0}ms", tls);
        }
        println!("  - TTFB: {:.0}ms", http_data.timing.ttfb_ms);
    }
    println!();

    let warnings: Vec<_> = data.score.diagnosis.iter()
        .filter(|d| d.severity == crate::scoring::Severity::Critical || d.severity == crate::scoring::Severity::Warning)
        .collect();
    if !warnings.is_empty() {
        println!("## ⚠️ {}", i18n.t("warnings"));
        println!();
        for w in warnings {
            println!("- {}", w.issue);
        }
        println!();
    }

    let suggestions: Vec<_> = data.score.diagnosis.iter()
        .filter(|d| d.severity == crate::scoring::Severity::Info)
        .collect();
    if !suggestions.is_empty() {
        println!("## 💡 {}", i18n.t("suggestions"));
        println!();
        for s in suggestions {
            println!("- {}", s.solution);
        }
        println!();
    }
}