use anyhow::Result;
use colored::Colorize;

use crate::cli::Cli;
use crate::core::scanner;
use crate::i18n::I18n;
use crate::output;

pub async fn run(target: &str, ports: Option<Vec<u16>>, top: usize, concurrency: usize, cli: &Cli, i18n: &I18n) -> Result<()> {
    let result = match ports {
        Some(ref port_list) => scanner::scan_ports(target, port_list, concurrency).await,
        None => scanner::scan_top_ports(target, top, concurrency).await,
    };

    if cli.json {
        output::print_json(&result);
    } else if cli.md {
        println!("# {}: {}", i18n.t("port_scan"), target);
        println!();
        if let Some(data) = &result.data {
            println!("| Port | State | Service |");
            println!("|------|-------|---------|");
            for p in &data.ports {
                let state = match p.state {
                    crate::core::PortState::Open => i18n.t("port_open"),
                    crate::core::PortState::Closed => i18n.t("port_closed"),
                    crate::core::PortState::Filtered => i18n.t("port_filtered"),
                    crate::core::PortState::Timeout => i18n.t("port_timeout"),
                };
                println!("| {} | {} | {} |", p.port, state, p.service.as_deref().unwrap_or("-"));
            }
        }
    } else {
        println!("{}: {}", i18n.t("port_scan"), target);
        println!();
        match &result.data {
            Some(data) => {
                for p in &data.ports {
                    let state_str = match p.state {
                        crate::core::PortState::Open => format!("({})", i18n.t("port_open")).green().bold(),
                        crate::core::PortState::Closed => format!("({})", i18n.t("port_closed")).dimmed(),
                        crate::core::PortState::Filtered => format!("({})", i18n.t("port_filtered")).yellow(),
                        crate::core::PortState::Timeout => format!("({})", i18n.t("port_timeout")).red(),
                    };
                    println!("  {:<6} {} {}", p.port, state_str, p.service.as_deref().unwrap_or("-"));
                }
            }
            None => {
                println!("  ❌ {}", result.error.as_deref().unwrap_or(i18n.t("unknown_error")));
            }
        }
    }

    Ok(())
}