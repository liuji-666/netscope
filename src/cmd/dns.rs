use anyhow::Result;
use colored::Colorize;

use crate::cli::Cli;
use crate::core::resolver;
use crate::i18n::I18n;
use crate::output;

pub async fn run(domain: &str, cli: &Cli, i18n: &I18n) -> Result<()> {
    let result = resolver::full_diagnosis(domain, &["8.8.8.8".to_string(), "1.1.1.1".to_string()]).await;

    if cli.json {
        output::print_json(&result);
    } else if cli.md {
        println!("# {}: {}", i18n.t("dns_resolution"), domain);
        println!();
        if let Some(data) = &result.data {
            println!("| Type | Value |");
            println!("|------|-------|");
            for r in &data.records {
                println!("| {} | {} |", r.record_type, r.value);
            }
        }
    } else {
        println!("{}", i18n.t("dns_resolution"));
        println!();
        match &result.data {
            Some(data) => {
                for record in &data.records {
                    println!("  {}  {}", record.record_type.cyan(), record.value);
                }
                if data.consistent {
                    println!("  {}", i18n.t("dns_all_agree").green());
                } else {
                    println!("  {}", i18n.t("dns_disagree").red());
                }
            }
            None => {
                println!("  ❌ {}", result.error.as_deref().unwrap_or(i18n.t("unknown_error")));
            }
        }
    }

    Ok(())
}