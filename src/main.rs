mod cli;
mod cmd;
mod config;
mod core;
mod error;
mod i18n;
mod output;
mod scoring;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use i18n::I18n;
use core::config::NetScopeConfig;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    }

    let mut cfg = NetScopeConfig::load();

    if cfg.is_first_run() && !matches!(cli.command, Commands::Config { .. }) {
        show_welcome_wizard(&mut cfg);
    }

    let i18n = if let Some(lang) = cfg.get_language() {
        match lang.to_lowercase().as_str() {
            "zh" | "chinese" => I18n::new(i18n::Language::Chinese),
            _ => I18n::new(cli.lang.clone()),
        }
    } else {
        I18n::new(cli.lang.clone())
    };

    let result = match &cli.command {
        Commands::Check {
            target,
            ping_count,
            top_ports,
        } => cmd::check::run(target, *ping_count, *top_ports, &cli, &i18n).await,
        Commands::Ping {
            target,
            count,
            timeout,
        } => cmd::ping::run(target, *count, *timeout, &cli, &i18n).await,
        Commands::Trace { target, max_hops } => {
            cmd::trace::run(target, *max_hops, &cli, &i18n).await
        }
        Commands::Dns { domain } => cmd::dns::run(domain, &cli, &i18n).await,
        Commands::Port {
            target,
            ports,
            top,
            concurrency,
        } => cmd::port::run(target, ports.clone(), *top, *concurrency, &cli, &i18n).await,
        Commands::Http { url } => cmd::http::run(url, &cli, &i18n).await,
        Commands::Speed => cmd::speed::run(&cli, &i18n).await,
        Commands::Myip => cmd::myip::run(&cli, &i18n).await,
        Commands::Report { target, output } => {
            cmd::report::run(target, output.as_deref(), &cli, &i18n).await
        }
        Commands::DnsOptimizer { domain } => {
            cmd::dns_optimizer::run(domain.as_deref(), &cli, &i18n).await
        }
        Commands::Mirror { category } => {
            cmd::mirror::run(category.as_deref(), &cli, &i18n).await
        }
        Commands::Site { target } => {
            cmd::site::run(target.as_deref(), &cli, &i18n).await
        }
        Commands::Optimize => {
            cmd::optimize::run(&cli, &i18n).await
        }
        Commands::Batch { file, urls } => {
            cmd::batch::run(file.as_deref(), urls.clone(), &cli, &i18n).await
        }
        Commands::Tcp { target } => {
            cmd::tcp::run(target, &cli, &i18n).await
        }
        Commands::Mobile { url } => {
            cmd::mobile::run(&url, &cli, &i18n).await
        }
        Commands::Interactive => {
            cmd::interactive::run(&cli, &i18n).await
        }
        Commands::Config { action, key, value } => {
            cmd::config::run(action, key.as_deref(), value.as_deref(), &cli, &i18n).await
        }
    };

    if let Err(e) = result {
        if cli.json {
            eprintln!("{}", serde_json::json!({"error": e.to_string()}));
        } else {
            eprintln!("{} {}", i18n.t("error").red().bold(), e);
        }
        std::process::exit(1);
    }
}

fn show_welcome_wizard(cfg: &mut NetScopeConfig) {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("  🔭 Welcome to NetScope!");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("  First time setup - just a few seconds!");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    println!("🌐 Select your language / 选择您的语言:");
    println!();
    println!("  1. English");
    println!("  2. 中文");
    println!();

    print!("  Enter choice (1/2): ");
    io::stdout().flush().ok();

    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_ok() {
        let choice = choice.trim();
        match choice {
            "2" | "chinese" | "zh" => {
                cfg.set_language("zh");
                println!();
                println!("✅ 语言已设置为中文");
            }
            _ => {
                cfg.set_language("en");
                println!();
                println!("✅ Language set to English");
            }
        }
        println!();
    }

    println!("💡 Quick tips:");
    println!("  • Run netscope check <target> for full diagnosis");
    println!("  • Run netscope optimize to find fastest DNS");
    println!("  • Run netscope mirror to test mirrors");
    println!("  • Use --quiet flag for quiet mode");
    println!("  • Use --json flag for JSON output");
    println!();
    println!("Setup complete! Press Enter to continue...");

    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).ok();

    println!();
}