use anyhow::Result;
use colored::Colorize;

use crate::cli::Cli;
use crate::core::ipinfo;
use crate::i18n::I18n;
use crate::output;

pub async fn run(cli: &Cli, i18n: &I18n) -> Result<()> {
    let result = ipinfo::get_my_ip().await;

    if cli.json {
        output::print_json(&result);
    } else if cli.md {
        println!("# My IP");
        println!();
        if let Some(data) = &result.data {
            println!("**IP:** {}", data.ip);
            let location = vec![data.city.clone(), data.region.clone(), data.country.clone()]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join(", ");
            if !location.is_empty() {
                println!("**Location:** {}", location);
            }
            if let Some(org) = &data.org {
                println!("**ISP:** {}", org);
            }
        }
    } else {
        println!("{}", "🌐 My IP".bold());
        println!();
        match &result.data {
            Some(data) => {
                println!("  IP: {}", data.ip);
                let location = vec![data.city.clone(), data.region.clone(), data.country.clone()]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .join(", ");
                if !location.is_empty() {
                    println!("  Location: {}", location);
                }
                if let Some(org) = &data.org {
                    println!("  ISP: {}", org);
                }
            }
            None => {
                println!("  ❌ {}", result.error.as_deref().unwrap_or(i18n.t("unknown_error")));
            }
        }
    }

    Ok(())
}