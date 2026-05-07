use anyhow::Result;
use colored::Colorize;

use crate::cli::Cli;
use crate::core::config::NetScopeConfig;
use crate::i18n::I18n;

pub async fn run(action: &str, key: Option<&str>, value: Option<&str>, _cli: &Cli, _i18n: &I18n) -> Result<()> {
    match action.to_lowercase().as_str() {
        "show" => show_config(),
        "set" => set_config(key, value),
        "add" => add_favorite(key),
        "remove" => remove_favorite(key),
        "clear" => clear_config(),
        _ => {
            println!("{} Unknown action: {}", "⚠️ ".yellow(), action);
            print_help();
            Ok(())
        }
    }
}

fn show_config() -> Result<()> {
    let config = NetScopeConfig::load();

    println!("{}", "⚙️ NetScope Configuration".bold());
    println!();

    println!("{}", "📍 Preferred DNS:".bold());
    if let Some(dns) = &config.preferred_dns {
        println!("  Primary: {}", dns.green());
        if let Some(backup) = &config.dns_backup {
            println!("  Backup:  {}", backup.blue());
        }
    } else {
        println!("  {}", "Not set".dimmed());
    }

    println!();
    println!("{}", "🪞 Preferred Mirrors:".bold());
    if config.preferred_mirrors.is_empty() {
        println!("  {}", "None".dimmed());
    } else {
        for (category, mirror) in &config.preferred_mirrors {
            println!("  {}: {}", category, mirror.green());
        }
    }

    println!();
    println!("{}", "⭐ Favorite Sites:".bold());
    if config.favorite_sites.is_empty() {
        println!("  {}", "None".dimmed());
        println!("  Hint: Use `netscope config add <url>` to add a site");
    } else {
        for (i, site) in config.favorite_sites.iter().enumerate() {
            println!("  {}. {}", i + 1, site);
        }
    }

    println!();
    println!("{}", "📦 Cache Status:".bold());
    let cache_count = config.cached_mirror_results.len();
    println!("  Mirror results cached: {}", cache_count);
    if let Some(last_update) = &config.last_update {
        println!("  Last updated: {}", last_update);
    }

    if let Some(path) = NetScopeConfig::get_config_path() {
        println!();
        println!("{}", "📁 Config Path:".bold());
        println!("  {}", path.to_string_lossy());
    }

    Ok(())
}

fn set_config(key: Option<&str>, value: Option<&str>) -> Result<()> {
    let mut config = NetScopeConfig::load();

    match (key, value) {
        (Some(k), Some(v)) => {
            match k.to_lowercase().as_str() {
                "dns" => {
                    let parts: Vec<&str> = v.split(',').collect();
                    let primary = parts[0];
                    let backup = parts.get(1).copied();
                    config.set_preferred_dns(primary, backup);
                    config.save()?;
                    println!("{} DNS set: {} (backup: {})", 
                        "✅".green(), 
                        primary, 
                        backup.unwrap_or("none")
                    );
                }
                category @ ("github" | "docker" | "pypi" | "npm" | "crates" | "go") => {
                    config.set_preferred_mirror(category, v, v);
                    config.save()?;
                    println!("{} {} mirror set to: {}", 
                        "✅".green(), 
                        category, 
                        v.green()
                    );
                }
                _ => {
                    println!("{} Unknown key: {}", "⚠️ ".yellow(), k);
                    print_help();
                }
            }
        }
        _ => {
            println!("{} Missing key or value", "⚠️ ".yellow());
            print_help();
        }
    }

    Ok(())
}

fn add_favorite(key: Option<&str>) -> Result<()> {
    if let Some(site) = key {
        let mut config = NetScopeConfig::load();
        config.add_favorite_site(site);
        config.save()?;
        println!("{} Added to favorites: {}", "✅".green(), site);
    } else {
        println!("{} Missing site URL", "⚠️ ".yellow());
        print_help();
    }
    Ok(())
}

fn remove_favorite(key: Option<&str>) -> Result<()> {
    if let Some(site) = key {
        let mut config = NetScopeConfig::load();
        config.remove_favorite_site(site);
        config.save()?;
        println!("{} Removed from favorites: {}", "✅".green(), site);
    } else {
        println!("{} Missing site URL", "⚠️ ".yellow());
        print_help();
    }
    Ok(())
}

fn clear_config() -> Result<()> {
    let path = match NetScopeConfig::get_config_path() {
        Some(p) => p,
        None => {
            println!("{} Cannot find config path", "⚠️ ".yellow());
            return Ok(());
        }
    };

    if path.exists() {
        std::fs::remove_file(&path)?;
        println!("{} Configuration cleared", "✅".green());
    } else {
        println!("{} No config file to clear", "ℹ️ ".blue());
    }
    Ok(())
}

fn print_help() {
    println!();
    println!("{}", "Usage:".bold());
    println!("  netscope config show              Show current configuration");
    println!("  netscope config set dns <ip>      Set preferred DNS server");
    println!("  netscope config set dns <ip>,<backup>  Set primary and backup DNS");
    println!("  netscope config set <category> <mirror>  Set preferred mirror");
    println!("  netscope config add <url>         Add site to favorites");
    println!("  netscope config remove <url>      Remove site from favorites");
    println!("  netscope config clear             Clear all configuration");
    println!();
    println!("{}", "Mirror categories: github, docker, pypi, npm, crates, go".dimmed());
}
