use anyhow::Result;
use colored::Colorize;

use crate::cli::Cli;
use crate::core::speed_test;
use crate::i18n::I18n;
use crate::output;

pub async fn run(cli: &Cli, i18n: &I18n) -> Result<()> {
    let result = speed_test::run_speed_test().await;

    if cli.json {
        output::print_json(&result);
    } else if cli.md {
        println!("# Speed Test");
        println!();
        println!("*Note: Full speed test requires specialized server endpoints*");
        println!();
        if let Some(data) = &result.data {
            println!("**Download:** {} Mbps", data.download_mbps);
            println!("**Upload:** {} Mbps", data.upload_mbps);
            println!("**Latency:** {} ms", data.latency_ms);
            println!("**Jitter:** {} ms", data.jitter_ms);
        }
    } else {
        println!("{}", "⚡ Speed Test".bold());
        println!("{}", "  Note: Full speed test requires specialized server endpoints".dimmed());
        println!();
        match &result.data {
            Some(data) => {
                println!("  Latency: {:.1} ms", data.latency_ms);
                println!("  Jitter: {:.1} ms", data.jitter_ms);
                if data.download_mbps == 0.0 {
                    println!("  Download: N/A (requires speed test server)");
                    println!("  Upload: N/A (requires speed test server)");
                }
            }
            None => {
                println!("  ❌ {}", result.error.as_deref().unwrap_or(i18n.t("unknown_error")));
            }
        }
    }

    Ok(())
}