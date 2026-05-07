use anyhow::Result;
use colored::Colorize;

use crate::cli::Cli;
use crate::core::pinger;
use crate::i18n::I18n;
use crate::output;

pub async fn run(target: &str, count: u32, timeout: u64, cli: &Cli, i18n: &I18n) -> Result<()> {
    let result = pinger::tcp_ping(target, count, timeout).await;

    if cli.json {
        output::print_json(&result);
    } else if cli.md {
        println!("# {}: {}", i18n.t("connectivity"), target);
        println!();
        if let Some(data) = &result.data {
            println!("| {} | {} |", i18n.t("ping_received"), i18n.t("ping_loss"));
            println!("|------|-------|");
            println!("| {}/{} | {:.1}% |", data.packets_recv, data.packets_sent, data.loss_pct);
            println!();
            println!("**RTT:** min={:.1} avg={:.1} max={:.1} ms", data.rtt_min_ms, data.rtt_avg_ms, data.rtt_max_ms);
        }
    } else {
        println!("{}: {}", i18n.t("connectivity"), target);
        println!();
        match &result.data {
            Some(data) => {
                let hops_display: String = data.hops.iter().map(|h| if h.success { "●" } else { "○" }).collect();
                println!("  {}  {}/{} {}", hops_display.green(), data.packets_recv, data.packets_sent, i18n.t("ping_received"));
                println!("  {}: {:.1}%", i18n.t("ping_loss"), data.loss_pct);
                println!("  RTT: min={:.1} avg={:.1} max={:.1} ms", data.rtt_min_ms, data.rtt_avg_ms, data.rtt_max_ms);
            }
            None => {
                println!("  ❌ {}", result.error.as_deref().unwrap_or(i18n.t("unknown_error")));
            }
        }
    }

    Ok(())
}