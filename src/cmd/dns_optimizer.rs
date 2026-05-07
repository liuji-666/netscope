use anyhow::Result;
use colored::Colorize;

use crate::cli::Cli;
use crate::core::dns_optimizer::{DnsOptimizer, DnsOptimizerResult};
use crate::i18n::I18n;
use crate::output;

pub async fn run(domain: Option<&str>, cli: &Cli, _i18n: &I18n) -> Result<()> {
    let target_domain = domain.unwrap_or("www.google.com");
    
    if !cli.quiet && !cli.json && !cli.md {
        println!("{}", "🔧 DNS Optimizer".cyan().bold());
        println!("  Testing DNS servers for: {}", target_domain);
        println!();
    }

    let optimizer = DnsOptimizer::new();
    let result = optimizer.optimize(target_domain).await;

    if cli.json {
        if let Some(data) = &result.data {
            output::print_json(data);
        } else {
            output::print_json(&result);
        }
    } else if cli.md {
        if let Some(data) = &result.data {
            print_markdown(data);
        } else {
            println!("DNS optimization failed");
        }
    } else {
        if let Some(data) = &result.data {
            print_terminal(data);
        } else {
            println!("DNS optimization failed");
        }
    }

    Ok(())
}

fn print_terminal(result: &DnsOptimizerResult) {
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".cyan());
    println!("{}  {}", "║".cyan(), "📊 DNS Server Comparison".bold());
    println!("{}  {} {}", "║".cyan(), "Target:".bold(), result.target_domain);
    println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());
    
    println!("{}  {:<22} {:<15} {:>8}", 
        "║".cyan(), 
        "DNS Server".bold(), 
        "IP".bold(), 
        "Avg(ms)".bold()
    );
    println!("{}  {}", "║".cyan(), "-".repeat(50).cyan());

    for server in &result.servers {
        let score_color = if server.score >= 80 {
            "green"
        } else if server.score >= 50 {
            "yellow"
        } else {
            "red"
        };

        let avg_color_fn = if server.avg_ms < 30.0 {
            |s: String| s.green()
        } else if server.avg_ms < 100.0 {
            |s: String| s.yellow()
        } else {
            |s: String| s.red()
        };

        let avg_str = format!("{:.1}", server.avg_ms);
        let score_str = format!("{}", server.score);
        let ip_str = format!("({})", server.ip);

        let line = format!(
            "  {:<22} {:<15} {:>8}",
            server.server,
            ip_str.dimmed(),
            avg_color_fn(avg_str)
        );
        let final_line = match score_color {
            "green" => line.green().to_string(),
            "yellow" => line.yellow().to_string(),
            _ => line.red().to_string(),
        };
        println!("{}  {} {:>8}", "║".cyan(), final_line, score_str.bold());
    }

    println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());
    
    if let Some(best) = result.servers.first() {
        println!("{}  {}", "║".cyan(), "✅ Best DNS Server".bold().green());
        println!("{}    {} ({})", "║".cyan(), best.server.bold(), best.ip);
        println!("{}    {}: {:.1}ms", "║".cyan(), "Average latency".dimmed(), best.avg_ms);
        println!("{}    {}: {:.0}%", "║".cyan(), "Success rate".dimmed(), best.success_rate);
    }

    println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());
    println!("{}  {}", "║".cyan(), "💡 Recommendation".bold().cyan());
    println!("{}    {}", "║".cyan(), result.recommendation);
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".cyan());
    
    println!();
    println!("{}", "📝 How to use:".bold());
    println!("  Linux/Mac: Add to /etc/resolv.conf:");
    if let Some(best) = result.servers.first() {
        println!("    nameserver {}", best.ip);
    }
    println!();
    println!("  Windows: Network Settings -> DNS -> Add:");
    if let Some(best) = result.servers.first() {
        println!("    Preferred: {}", best.ip);
    }
}

fn print_markdown(result: &DnsOptimizerResult) {
    println!("# DNS Optimizer Report");
    println!();
    println!("**Target:** {}", result.target_domain);
    println!();
    println!("## DNS Server Comparison");
    println!();
    println!("| DNS Server | IP | Avg (ms) | Score |");
    println!("|------------|----|----------|-------|");
    
    for server in &result.servers {
        println!(
            "| {} | {} | {:.1} | {} |",
            server.server,
            server.ip,
            server.avg_ms,
            server.score
        );
    }
    println!();

    if let Some(best) = result.servers.first() {
        println!("## Best DNS Server");
        println!();
        println!("**{}** ({})\n", best.server, best.ip);
        println!("- Average latency: {:.1}ms\n", best.avg_ms);
        println!("- Success rate: {:.0}%\n", best.success_rate);
    }

    println!("## Recommendation");
    println!();
    println!("{}\n", result.recommendation);

    println!("## How to Use");
    println!();
    println!("**Linux/Mac:** Add to `/etc/resolv.conf`:\n```bash\nnameserver {}\n```\n", 
        result.servers.first().map(|s| s.ip.as_str()).unwrap_or(""));
    println!("**Windows:** Network Settings -> DNS -> Add:\n");
    println!("- Preferred: {}\n", 
        result.servers.first().map(|s| s.ip.as_str()).unwrap_or(""));
}