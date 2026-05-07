use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

use super::check;
use crate::cli::Cli;
use super::dns_optimizer;
use crate::i18n::I18n;
use super::mirror;
use super::port;
use super::site;
use crate::i18n::Language;

pub async fn run(_cli: &Cli, _i18n: &I18n) -> Result<()> {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("{}  {}", "║", "🔭 NetScope Interactive Mode".bold().cyan());
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("{}  Welcome to NetScope! I'll help you diagnose your network.", "║");
    println!("{}  What would you like to do today?", "║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    loop {
        println!("┌─────────────────────────────────────────────────────────────┐");
        println!("{}  Select an option:", "║");
        println!("{}    1. 🌐  Full network diagnosis (recommended for new users)", "║");
        println!("{}    2. 🔧  DNS optimization - find the fastest DNS server", "║");
        println!("{}    3. 🪞  Mirror speed test - find fastest GitHub/Docker/PyPI mirror", "║");
        println!("{}    4. 📡  Test a specific website", "║");
        println!("{}    5. 🔍  Port scan", "║");
        println!("{}    6. ❌  Exit", "║");
        println!("└─────────────────────────────────────────────────────────────┘");
        print!("{}  Enter your choice (1-6): ", "║");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("  Enter target (domain or IP): ");
                io::stdout().flush()?;
                let mut target = String::new();
                io::stdin().read_line(&mut target)?;
                let target = target.trim();
                if !target.is_empty() {
                    println!();
                    println!("{}", "⏳ Running full diagnosis...".yellow());
                    println!();
                    let cli = Cli::default();
                    let i18n = I18n::new(Language::English);
                    check::run(&target.to_string(), 5, 20, &cli, &i18n).await?;
                }
            }
            "2" => {
                println!();
                println!("{}", "⏳ Finding the best DNS server for you...".yellow());
                println!();
                let cli = Cli::default();
                let i18n = I18n::new(Language::English);
                dns_optimizer::run(None, &cli, &i18n).await?;
            }
            "3" => {
                println!();
                println!("┌─────────────────────────────────────────────────────────────┐");
                println!("{}  Select mirror category:", "║");
                println!("{}    1. GitHub    - Find fastest GitHub mirror", "║");
                println!("{}    2. Docker    - Find fastest Docker Hub mirror", "║");
                println!("{}    3. PyPI      - Find fastest PyPI mirror", "║");
                println!("{}    4. npm       - Find fastest npm mirror", "║");
                println!("{}    5. All       - Test all categories", "║");
                println!("└─────────────────────────────────────────────────────────────┘");
                print!("{}  Enter choice (1-5): ", "║");
                io::stdout().flush()?;

                let mut cat_choice = String::new();
                io::stdin().read_line(&mut cat_choice)?;
                let cat_choice = cat_choice.trim();

                let category = match cat_choice {
                    "1" => Some("github"),
                    "2" => Some("docker"),
                    "3" => Some("pypi"),
                    "4" => Some("npm"),
                    _ => None,
                };

                let cli = Cli::default();
                let i18n = I18n::new(Language::English);
                mirror::run(category, &cli, &i18n).await?;
            }
            "4" => {
                print!("  Enter website URL: ");
                io::stdout().flush()?;
                let mut url = String::new();
                io::stdin().read_line(&mut url)?;
                let url = url.trim();
                if !url.is_empty() {
                    let cli = Cli::default();
                    let i18n = I18n::new(Language::English);
                    site::run(Some(&url.to_string()), &cli, &i18n).await?;
                }
            }
            "5" => {
                print!("  Enter target (domain or IP): ");
                io::stdout().flush()?;
                let mut target = String::new();
                io::stdin().read_line(&mut target)?;
                let target = target.trim();
                if !target.is_empty() {
                    let cli = Cli::default();
                    let i18n = I18n::new(Language::English);
                    port::run(&target.to_string(), None, 20, 100, &cli, &i18n).await?;
                }
            }
            "6" | "q" | "exit" => {
                println!();
                println!("{}", "👋 Thanks for using NetScope! Goodbye.".green());
                println!();
                break;
            }
            _ => {
                println!();
                println!("{} Invalid choice. Please try again.", "⚠️  ".yellow());
                println!();
            }
        }
    }

    Ok(())
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            command: crate::cli::Commands::Check {
                target: String::new(),
                ping_count: 5,
                top_ports: 20,
            },
            json: false,
            md: false,
            no_color: false,
            timeout: 10,
            verbose: false,
            quiet: false,
            lang: Language::English,
        }
    }
}