use anyhow::Result;

use crate::cli::Cli;
use crate::config::Config;
use crate::core::{resolver, pinger, scanner, traceroute, http_probe};
use crate::error::ProbeResult;
use crate::i18n::I18n;
use crate::output;
use crate::scoring;

pub async fn run(target: &str, ping_count: u32, top_ports: usize, cli: &Cli, i18n: &I18n) -> Result<()> {
    let config = Config::default();

    if !cli.quiet && !cli.json && !cli.md {
        println!("{}", i18n.t("starting_diagnosis").cyan());
        println!();
    }

    let dns_result = resolver::full_diagnosis(target, &config.dns_resolvers).await;

    let ip = dns_result
        .data
        .as_ref()
        .and_then(|d| d.records.first())
        .map(|r| r.value.clone())
        .unwrap_or_else(|| target.to_string());

    let (ping_result, port_result, trace_result) = tokio::join!(
        pinger::tcp_ping(target, ping_count, config.ping_timeout.as_secs()),
        scanner::scan_top_ports(target, top_ports, config.port_concurrency),
        traceroute::trace(target, config.trace_max_hops),
    );

    let needs_http = port_result
        .data
        .as_ref()
        .map(|p| {
            p.ports.iter().any(|pi| {
                pi.state == crate::core::PortState::Open
                    && (pi.port == 80 || pi.port == 443)
            })
        })
        .unwrap_or(false);

    let http_result = if needs_http {
        let scheme = if port_result
            .data
            .as_ref()
            .map(|p| {
                p.ports
                    .iter()
                    .any(|pi| pi.port == 443 && pi.state == crate::core::PortState::Open)
            })
            .unwrap_or(false)
        {
            "https"
        } else {
            "http"
        };
        http_probe::analyze(&format!("{}://{}", scheme, target)).await
    } else {
        ProbeResult::skipped(i18n.t("skipped"))
    };

    let score_result = scoring::evaluate(&dns_result, &ping_result, &trace_result, &port_result, &http_result, i18n);

    if cli.json {
        let json_output = serde_json::json!({
            "netscope_version": env!("CARGO_PKG_VERSION"),
            "command": "check",
            "target": target,
            "ip": ip,
            "timestamp": chrono::Local::now().to_rfc3339(),
            "score": score_result.score,
            "grade": score_result.grade,
            "dns": dns_result,
            "ping": ping_result,
            "trace": trace_result,
            "ports": port_result,
            "http": http_result,

        });
        output::print_json(&json_output);
    } else if cli.md {
        output::print_markdown(&output::ReportData {
            target,
            ip: &ip,
            score: &score_result,
            dns: &dns_result,
            ping: &ping_result,
            trace: &trace_result,
            ports: &port_result,
            http: &http_result,
            i18n,
        });
    } else {
        output::print_check_header(target, &ip, &score_result, i18n);
        output::print_dns_section(&dns_result, i18n);
        output::print_ping_section(&ping_result, i18n);
        output::print_trace_section(&trace_result, i18n);
        output::print_port_section(&port_result, i18n);
        output::print_http_section(&http_result, i18n);
        output::print_footer(&score_result, i18n);
    }

    Ok(())
}

use colored::Colorize;