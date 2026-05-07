use anyhow::Result;
use std::fs::File;
use std::io::Write;

use crate::cli::Cli;
use crate::config::Config;
use crate::core::{resolver, pinger, scanner, traceroute, http_probe};

use crate::i18n::I18n;
use crate::output;
use crate::scoring;

pub async fn run(target: &str, output_path: Option<&str>, _cli: &Cli, i18n: &I18n) -> Result<()> {
    let config = Config::default();

    let dns_result = resolver::full_diagnosis(target, &config.dns_resolvers).await;

    let ip = dns_result
        .data
        .as_ref()
        .and_then(|d| d.records.first())
        .map(|r| r.value.clone())
        .unwrap_or_else(|| target.to_string());

    let (ping_result, port_result, trace_result) = tokio::join!(
        pinger::tcp_ping(target, 5, config.ping_timeout.as_secs()),
        scanner::scan_top_ports(target, 20, config.port_concurrency),
        traceroute::trace(target, config.trace_max_hops),
    );

    let http_result = http_probe::analyze(&format!("https://{}", target)).await;

    let score_result = scoring::evaluate(&dns_result, &ping_result, &trace_result, &port_result, &http_result, i18n);

    let report_data = output::ReportData {
        target,
        ip: &ip,
        score: &score_result,
        dns: &dns_result,
        ping: &ping_result,
        trace: &trace_result,
        ports: &port_result,
        http: &http_result,
        i18n,
    };

    if let Some(path) = output_path {
        let mut file = File::create(path)?;
        let mut output = String::new();
        
        output.push_str("# NetScope Diagnosis Report\n\n");
        output.push_str(&format!("**{}:** {} ({})\n", i18n.t("target"), target, ip));
        output.push_str(&format!("**{}:** {}\n", i18n.t("time"), chrono::Local::now().format("%Y-%m-%d %H:%M:%S %Z")));
        output.push_str(&format!("**{}:** {}/100 — {}\n\n", i18n.t("score"), score_result.score, score_result.grade));

        // DNS section
        output.push_str(&format!("## {}\n\n", i18n.t("dns_resolution")));
        if let Some(dns_data) = &dns_result.data {
            output.push_str("| Type | Value |\n");
            output.push_str("|------|-------|\n");
            for r in &dns_data.records {
                output.push_str(&format!("| {} | {} |\n", r.record_type, r.value));
            }
            output.push_str("\n");
            if dns_data.consistent {
                output.push_str(&format!("✅ {}\n", i18n.t("dns_all_agree")));
            } else {
                output.push_str(&format!("⚠️ {}\n", i18n.t("warn_dns_hijack")));
            }
        }
        output.push_str("\n");

        // Connectivity section
        output.push_str(&format!("## {}\n\n", i18n.t("connectivity")));
        if let Some(ping_data) = &ping_result.data {
            output.push_str(&format!("- **{}:** {:.1}%\n", i18n.t("ping_loss"), ping_data.loss_pct));
            output.push_str(&format!("- **RTT:** min={:.1} avg={:.1} max={:.1} ms\n", ping_data.rtt_min_ms, ping_data.rtt_avg_ms, ping_data.rtt_max_ms));
        }
        output.push_str("\n");

        // Warnings
        let warnings: Vec<_> = score_result.diagnosis.iter()
            .filter(|d| d.severity == crate::scoring::Severity::Critical || d.severity == crate::scoring::Severity::Warning)
            .collect();
        if !warnings.is_empty() {
            output.push_str(&format!("## ⚠️ {}\n\n", i18n.t("warnings")));
            for w in warnings {
                output.push_str(&format!("- {}\n", w.issue));
            }
            output.push_str("\n");
        }

        // Suggestions
        let suggestions: Vec<_> = score_result.diagnosis.iter()
            .filter(|d| d.severity == crate::scoring::Severity::Info)
            .collect();
        if !suggestions.is_empty() {
            output.push_str(&format!("## 💡 {}\n\n", i18n.t("suggestions")));
            for s in suggestions {
                output.push_str(&format!("- {}\n", s.solution));
            }
            output.push_str("\n");
        }

        file.write_all(output.as_bytes())?;
        println!("{}: {}", i18n.t("suggestions"), path);
    } else {
        output::print_markdown(&report_data);
    }

    Ok(())
}

