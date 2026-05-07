use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::cli::Cli;
use crate::core::http_probe::probe_url;
use crate::error::ProbeStatus;
use crate::i18n::I18n;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSiteResult {
    pub url: String,
    pub domain: String,
    pub success: bool,
    pub latency_ms: Option<f64>,
    pub status_code: Option<u16>,
    pub score: u32,
    pub error: Option<String>,
}

pub async fn run(file_path: Option<&str>, urls: Vec<String>, cli: &Cli, _i18n: &I18n) -> Result<()> {
    let mut target_urls = Vec::new();

    if let Some(path) = file_path {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                let line = line.trim();
                if !line.is_empty() && !line.starts_with('#') {
                    target_urls.push(line.to_string());
                }
            }
        } else {
            println!("{}", format!("❌ Failed to read file: {}", path).red().bold());
            return Ok(());
        }
    }

    target_urls.extend(urls);

    if target_urls.is_empty() {
        println!("{}", "❌ No URLs to test. Provide a file path or URLs as arguments.".red().bold());
        println!();
        println!("{}", "Usage:".bold());
        println!("  netscope batch --file sites.txt");
        println!("  netscope batch https://google.com https://github.com");
        println!();
        println!("{}", "sites.txt format:".bold());
        println!("  # One URL per line");
        println!("  https://google.com");
        println!("  https://github.com");
        return Ok(());
    }

    if !cli.quiet && !cli.json && !cli.md {
        println!("{}", "📊 Batch Site Tester".cyan().bold());
        println!("  Testing {} URLs...", target_urls.len());
        if !cli.verbose {
            println!("  (Use --verbose for detailed output)");
        }
        println!();
    }

    let semaphore = Arc::new(Semaphore::new(10));
    let mut handles = Vec::new();

    for url in target_urls {
        let sem = semaphore.clone();
        
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            test_single_url(&url).await
        }));
    }

    let mut results: Vec<BatchSiteResult> = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }

    results.sort_by(|a, b| b.score.cmp(&a.score));

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap_or_default());
    } else if cli.md {
        print_markdown(&results);
    } else {
        print_terminal(&results, cli.verbose);
    }

    Ok(())
}

async fn test_single_url(url: &str) -> BatchSiteResult {
    let domain = extract_domain(url);
    
    let start = std::time::Instant::now();
    let probe_result = probe_url(url, Some(5000)).await;
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    if probe_result.status == ProbeStatus::Ok {
        if let Some(http_result) = probe_result.data {
            let score = calculate_score(http_result.status, latency_ms);
            
            return BatchSiteResult {
                url: url.to_string(),
                domain,
                success: true,
                latency_ms: Some(latency_ms),
                status_code: Some(http_result.status),
                score,
                error: None,
            };
        }
    }
    
    BatchSiteResult {
        url: url.to_string(),
        domain,
        success: false,
        latency_ms: Some(latency_ms),
        status_code: None,
        score: 0,
        error: probe_result.error,
    }
}

fn extract_domain(url: &str) -> String {
    url.replace("https://", "")
       .replace("http://", "")
       .split('/')
       .next()
       .unwrap_or(url)
       .to_string()
}

fn calculate_score(status: u16, latency_ms: f64) -> u32 {
    let status_score = if status == 200 {
        50
    } else if status >= 200 && status < 300 {
        50
    } else if status >= 300 && status < 400 {
        40
    } else if status >= 400 && status < 500 {
        20
    } else {
        0
    };

    let latency_score = if latency_ms < 100.0 {
        50
    } else if latency_ms < 300.0 {
        40
    } else if latency_ms < 500.0 {
        30
    } else if latency_ms < 1000.0 {
        20
    } else {
        10
    };

    status_score + latency_score
}

fn print_terminal(results: &[BatchSiteResult], verbose: bool) {
    let success_count = results.iter().filter(|r| r.success).count();
    let total_count = results.len();
    let avg_latency: f64 = results.iter().filter_map(|r| r.latency_ms).sum::<f64>() / results.len() as f64;

    println!("{}", "╔══════════════════════════════════════════════════════════════╗".cyan());
    println!("{}  {}", "║".cyan(), "📊 Batch Test Summary".bold());
    println!("{}    Sites tested: {}", "║".cyan(), total_count);
    println!("{}    Successful: {} ({:.0}%)", "║".cyan(), success_count, (success_count as f64 / total_count as f64) * 100.0);
    println!("{}    Average latency: {:.0}ms", "║".cyan(), avg_latency);
    println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());
    
    println!("{}  {:<30} {:>8} {:>10} {:>8}", 
        "║".cyan(), 
        "Domain".bold(), 
        "Status".bold(), 
        "Latency".bold(), 
        "Score".bold()
    );
    println!("{}  {}", "║".cyan(), "-".repeat(60).cyan());

    for result in results {
        let status_str = if let Some(code) = result.status_code {
            format!("{}", code)
        } else {
            "Error".to_string()
        };

        let latency_str = if let Some(latency) = result.latency_ms {
            format!("{:.0}ms", latency)
        } else {
            "-".to_string()
        };

        let score_str = format!("{}", result.score);

        let marker = if result.score >= 80 { "✅" } else if result.score >= 50 { "⚠️" } else { "❌" };

        let line = format!(
            "  {} {:<28} {:>8} {:>10} {:>8}",
            marker,
            truncate(&result.domain, 28),
            status_str,
            latency_str,
            score_str
        );

        let final_line = if result.success {
            if result.score >= 80 {
                line.green().to_string()
            } else if result.score >= 50 {
                line.yellow().to_string()
            } else {
                line.red().to_string()
            }
        } else {
            line.red().dimmed().to_string()
        };

        println!("{}  {}", "║".cyan(), final_line);

        if verbose && !result.success {
            if let Some(ref error) = result.error {
                println!("{}    {} {}", "║".cyan(), "   Error:".red(), truncate(error, 50).dimmed());
            }
        }
    }

    println!("{}", "╚══════════════════════════════════════════════════════════════╝".cyan());
}

fn print_markdown(results: &[BatchSiteResult]) {
    println!("# Batch Site Test Report");
    println!();
    println!("| Domain | Status | Latency | Score |");
    println!("|--------|--------|---------|-------|");
    
    for result in results {
        let status_str = if let Some(code) = result.status_code {
            format!("{}", code)
        } else {
            "Error".to_string()
        };

        let latency_str = if let Some(latency) = result.latency_ms {
            format!("{:.0}ms", latency)
        } else {
            "-".to_string()
        };

        let recommended = if result.score >= 80 { "✅" } else { "" };

        println!(
            "| {} | {} | {} | {} | {} |",
            result.domain,
            status_str,
            latency_str,
            result.score,
            recommended
        );
    }
    println!();
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}