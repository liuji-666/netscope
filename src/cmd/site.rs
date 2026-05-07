use anyhow::Result;
use colored::Colorize;

use crate::cli::Cli;
use crate::core::site_tester::{SiteTester, SiteTestResult};
use crate::i18n::I18n;
use crate::output;

pub async fn run(target: Option<&str>, cli: &Cli, _i18n: &I18n) -> Result<()> {
    let tester = SiteTester::new();

    if !cli.quiet && !cli.json && !cli.md {
        println!("{}", "🔍 Site Entry Tester".cyan().bold());
        if let Some(t) = target {
            println!("  Testing: {}", t);
        } else {
            println!("  Testing all supported sites...");
        }
        println!();
    }

    let results = if let Some(t) = target {
        if t.starts_with("http://") || t.starts_with("https://") || t.contains('.') {
            let result = tester.test_any_url(t).await;
            vec![result]
        } else {
            match tester.test_site_by_name(t).await {
                Some(result) => vec![result],
                None => {
                    println!("Unknown site: {}. Try a URL (http://example.com) or one of: Google, GitHub, YouTube, ChatGPT, etc.", t);
                    return Ok(());
                }
            }
        }
    } else {
        tester.test_all().await
    };

    if cli.json {
        output::print_json(&results);
    } else if cli.md {
        print_markdown(&results);
    } else {
        print_terminal(&results);
    }

    Ok(())
}

fn print_terminal(results: &[SiteTestResult]) {
    for result in results {
        println!("{}", "╔══════════════════════════════════════════════════════════════╗".cyan());
        println!("{}  {}", "║".cyan(), format!("🌐 {}", result.site).bold());
        println!("{}  {}", "║".cyan(), result.summary);
        println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());
        
        println!("{}  {:<15} {:<20} {:>10} {:>10}", 
            "║".cyan(), 
            "Region".bold(), 
            "URL".bold(), 
            "Latency".bold(), 
            "Score".bold()
        );
        println!("{}  {}", "║".cyan(), "-".repeat(60).cyan());

        for entry in &result.entries {
            let _status_color = if entry.success { "green" } else { "red" };
            let recommended_marker = if entry.recommended { "✅ " } else { "   " };

            let latency_str = if entry.success {
                format!("{:.0}ms", entry.latency_ms)
            } else {
                "Failed".to_string()
            };

            let score_str = if entry.success {
                format!("{}", entry.score)
            } else {
                "-".to_string()
            };

            let line = format!(
                "  {}{:<12} {:<20} {:>10} {:>10}",
                recommended_marker,
                entry.region,
                truncate(&entry.url, 20),
                latency_str,
                score_str
            );

            let final_line = if !entry.success {
                line.dimmed().red().to_string()
            } else if entry.recommended {
                line.green().bold().to_string()
            } else if entry.score >= 70 {
                line.green().to_string()
            } else if entry.score >= 50 {
                line.yellow().to_string()
            } else {
                line.red().to_string()
            };

            println!("{}  {}", "║".cyan(), final_line);
        }

        if let Some(url) = &result.recommended_url {
            println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());
            println!("{}  {}", "║".cyan(), "💡 Recommended Entry".bold().green());
            println!("{}    {}", "║".cyan(), url);
        }

        println!("{}", "╚══════════════════════════════════════════════════════════════╝".cyan());
        println!();
    }
}

fn print_markdown(results: &[SiteTestResult]) {
    for result in results {
        println!("## {}", result.site);
        println!();
        println!("**Summary:** {}", result.summary);
        println!();
        println!("| Region | URL | Latency | Score | Recommended |");
        println!("|--------|-----|----------|-------|-------------|");
        
        for entry in &result.entries {
            let latency_str = if entry.success {
                format!("{:.0}ms", entry.latency_ms)
            } else {
                "Failed".to_string()
            };

            let score_str = if entry.success {
                format!("{}", entry.score)
            } else {
                "-".to_string()
            };

            let recommended = if entry.recommended { "✅" } else { "" };

            println!(
                "| {} | {} | {} | {} | {} |",
                entry.region,
                entry.url,
                latency_str,
                score_str,
                recommended
            );
        }

        if let Some(url) = &result.recommended_url {
            println!();
            println!("**Recommended:** {}", url);
        }
        println!();
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len-3])
    }
}