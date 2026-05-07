use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use crate::cli::Cli;
use crate::core::mirror::{MirrorTester, MirrorTestResult};
use crate::i18n::I18n;
use crate::output;

pub async fn run(category: Option<&str>, cli: &Cli, _i18n: &I18n) -> Result<()> {
    let tester = MirrorTester::new();

    if !cli.quiet && !cli.json && !cli.md {
        println!("{}", "🔧 Mirror Speed Tester".cyan().bold());
        if let Some(cat) = category {
            println!("  Testing mirrors for: {}", cat);
        } else {
            println!("  Testing all mirror categories...");
        }
        println!();
    }

    let results = if let Some(cat) = category {
        test_category_with_progress(&tester, cat).await
    } else {
        test_all_with_progress(&tester, cli).await
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

async fn test_category_with_progress(tester: &MirrorTester, category: &str) -> Vec<MirrorTestResult> {
    vec![tester.test_category_by_name(category).await.unwrap_or_else(|| MirrorTestResult {
        category: category.to_string(),
        mirrors: vec![],
        best_mirror: None,
        total_tested: 0,
        successful: 0,
    })]
}

async fn test_all_with_progress(tester: &MirrorTester, cli: &Cli) -> Vec<MirrorTestResult> {
    let categories = vec![
        "GitHub", "Docker Hub", "PyPI", "npm", "Rust Crates", "Go Module"
    ];

    if cli.quiet {
        return tester.test_all().await;
    }

    let pb = ProgressBar::new(categories.len() as u64);
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} [{elapsed_precise}] {msg}")
            .unwrap()
    );
    pb.set_message("Initializing...");

    let mut results = Vec::new();
    for (i, cat) in categories.iter().enumerate() {
        pb.set_message(format!("Testing {}...", cat));
        let result = tester.test_category_by_name(cat).await.unwrap_or_else(|| MirrorTestResult {
            category: cat.to_string(),
            mirrors: vec![],
            best_mirror: None,
            total_tested: 0,
            successful: 0,
        });
        results.push(result);
        pb.set_position((i + 1) as u64);
    }

    pb.finish_with_message("Complete!");
    println!();

    results
}

fn print_terminal(results: &[MirrorTestResult]) {
    for result in results {
        println!("{}", "╔══════════════════════════════════════════════════════════════╗".cyan());
        println!("{}  {}", "║".cyan(), format!("📦 {}", result.category).bold());
        println!("{}  {} {}/{} {}",
            "║".cyan(),
            "Status:".dimmed(),
            result.successful,
            result.total_tested,
            "mirrors OK".dimmed()
        );
        println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());

        println!("{}  {:<25} {:>10} {:>10}",
            "║".cyan(),
            "Mirror".bold(),
            "Latency".bold(),
            "Speed".bold()
        );
        println!("{}  {}", "║".cyan(), "-".repeat(50).cyan());

        for mirror in &result.mirrors {
            let score_color = if mirror.score >= 80 {
                "green"
            } else if mirror.score >= 50 {
                "yellow"
            } else {
                "red"
            };

            let latency_str = if mirror.status == "OK" {
                format!("{:.0}ms", mirror.avg_ms)
            } else {
                mirror.status.clone()
            };

            let speed_str = if let Some(speed) = mirror.speed_mbps {
                format!("{:.1} Mbps", speed)
            } else {
                "-".to_string()
            };

            let recommended_marker = if mirror.recommended { "✅ " } else { "   " };

            let line = format!(
                "  {}{:<22} {:>10} {:>10}",
                recommended_marker,
                mirror.name,
                latency_str,
                speed_str
            );

            let final_line = if mirror.status != "OK" {
                line.dimmed().to_string()
            } else if mirror.score >= 80 {
                match score_color {
                    "green" => line.green().to_string(),
                    "yellow" => line.yellow().to_string(),
                    _ => line.red().to_string(),
                }
            } else {
                line.yellow().to_string()
            };

            println!("{}  {}", "║".cyan(), final_line);
        }

        if let Some(best) = &result.best_mirror {
            println!("{}  {}", "║".cyan(), "-".repeat(50).cyan());
            println!("{}  {} Best: {}", "║".cyan(), "✅".green(), best.bold());
        }

        println!("{}", "╚══════════════════════════════════════════════════════════════╝".cyan());
        println!();
    }
}

fn print_markdown(results: &[MirrorTestResult]) {
    for result in results {
        println!("## {}", result.category);
        println!();
        println!("| Mirror | Latency | Speed | Score | Recommended |");
        println!("|--------|----------|-------|-------|-------------|");

        for mirror in &result.mirrors {
            let latency_str = if mirror.status == "OK" {
                format!("{:.0}ms", mirror.avg_ms)
            } else {
                mirror.status.clone()
            };

            let speed_str = if let Some(speed) = mirror.speed_mbps {
                format!("{:.1} Mbps", speed)
            } else {
                "-".to_string()
            };

            let recommended = if mirror.recommended { "✅" } else { "" };

            println!(
                "| {} | {} | {} | {} | {} |",
                mirror.name,
                latency_str,
                speed_str,
                mirror.score,
                recommended
            );
        }
        println!();
    }
}