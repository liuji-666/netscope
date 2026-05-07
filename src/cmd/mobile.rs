use anyhow::Result;
use colored::Colorize;

use crate::cli::Cli;
use crate::core::http_probe::probe_url;
use crate::error::ProbeStatus;
use crate::i18n::I18n;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MobileResult {
    pub url: String,
    pub success: bool,
    pub latency_ms: Option<f64>,
    pub status: String,
}

pub async fn run(target: &str, cli: &Cli, _i18n: &I18n) -> Result<()> {
    if !cli.quiet && !cli.json && !cli.md {
        println!("{}", "📱 Mobile Mode".cyan().bold());
    }

    let start = std::time::Instant::now();
    let probe_result = probe_url(target, Some(10000)).await;
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    let mobile_result = if probe_result.status == ProbeStatus::Ok {
        if let Some(http) = probe_result.data {
            let status = if http.status == 200 {
                "OK"
            } else if http.status >= 300 && http.status < 400 {
                "Redirect"
            } else if http.status >= 400 && http.status < 500 {
                "Client Error"
            } else {
                "Server Error"
            };

            MobileResult {
                url: target.to_string(),
                success: http.status < 400,
                latency_ms: Some(latency_ms),
                status: status.to_string(),
            }
        } else {
            MobileResult {
                url: target.to_string(),
                success: false,
                latency_ms: Some(latency_ms),
                status: "Failed".to_string(),
            }
        }
    } else {
        MobileResult {
            url: target.to_string(),
            success: false,
            latency_ms: Some(latency_ms),
            status: probe_result.error.unwrap_or_else(|| "Failed".to_string()),
        }
    };

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&mobile_result).unwrap_or_default());
    } else {
        print_mobile(&mobile_result);
    }

    Ok(())
}

fn print_mobile(result: &MobileResult) {
    let latency_str = if let Some(ms) = result.latency_ms {
        format!("{:.0}ms", ms)
    } else {
        "-".to_string()
    };

    if result.success {
        println!("{} {} {}", "OK".green(), result.url, latency_str.dimmed());
    } else {
        println!("{} {} {}", "FAIL".red(), result.url, result.status.as_str().red());
    }
}