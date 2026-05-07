use anyhow::Result;

use crate::cli::Cli;
use crate::core::traceroute;
use crate::i18n::I18n;
use crate::output;

pub async fn run(target: &str, max_hops: u32, cli: &Cli, i18n: &I18n) -> Result<()> {
    let result = traceroute::trace(target, max_hops).await;

    if cli.json {
        output::print_json(&result);
    } else if cli.md {
        println!("# {}: {}", i18n.t("route"), target);
        println!();
        if let Some(data) = &result.data {
            println!("| {} | {} | {} |", i18n.t("route_hop"), i18n.t("route_ip"), i18n.t("route_avg_ms"));
            println!("|------|-------|-----------|");
            for hop in &data.hops {
                let ip = hop.ip.as_deref().unwrap_or("*");
                let avg = hop.avg_ms.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "-".to_string());
                println!("| {} | {} | {} |", hop.hop, ip, avg);
            }
        }
    } else {
        println!("{}: {}", i18n.t("route"), target);
        println!();
        match &result.data {
            Some(data) => {
                println!("  {}  {}  {}", i18n.t("route_hop"), i18n.t("route_ip"), i18n.t("route_avg_ms"));
                for hop in &data.hops {
                    let ip = hop.ip.as_deref().unwrap_or("*");
                    let avg = hop.avg_ms.map(|v| format!("{:.1}ms", v)).unwrap_or_else(|| "-".to_string());
                    println!("  {:<4} {} {}", hop.hop, ip, avg);
                }
            }
            None => {
                println!("  ❌ {}", result.error.as_deref().unwrap_or(i18n.t("unknown_error")));
            }
        }
    }

    Ok(())
}