use anyhow::Result;

use crate::cli::Cli;
use crate::core::http_probe;
use crate::i18n::I18n;
use crate::output;

pub async fn run(url: &str, cli: &Cli, i18n: &I18n) -> Result<()> {
    let result = http_probe::analyze(url).await;

    if cli.json {
        output::print_json(&result);
    } else if cli.md {
        println!("# {}: {}", i18n.t("http_analysis"), url);
        println!();
        if let Some(data) = &result.data {
            println!("**{}:** {} {}", i18n.t("http_status"), data.status, data.status_text);
            println!("**{}:** {:.0}ms", i18n.t("http_total"), data.timing.total_ms);
            println!();
            println!("| Phase | Time (ms) |");
            println!("|-------|-----------|");
            println!("| DNS | {:.0} |", data.timing.dns_ms);
            println!("| TCP | {:.0} |", data.timing.tcp_ms);
            if let Some(tls) = data.timing.tls_ms {
                println!("| TLS | {:.0} |", tls);
            }
            println!("| TTFB | {:.0} |", data.timing.ttfb_ms);
        }
    } else {
        println!("{}: {}", i18n.t("http_analysis"), url);
        println!();
        match &result.data {
            Some(data) => {
                println!("  {}: {} {}", i18n.t("http_status"), data.status, data.status_text);
                println!("  {}: {:.0}ms", i18n.t("http_total"), data.timing.total_ms);
                println!("    DNS: {:.0}ms", data.timing.dns_ms);
                println!("    TCP: {:.0}ms", data.timing.tcp_ms);
                if let Some(tls) = data.timing.tls_ms {
                    println!("    TLS: {:.0}ms", tls);
                }
                println!("    TTFB: {:.0}ms", data.timing.ttfb_ms);
            }
            None => {
                println!("  ❌ {}", result.error.as_deref().unwrap_or(i18n.t("unknown_error")));
            }
        }
    }

    Ok(())
}