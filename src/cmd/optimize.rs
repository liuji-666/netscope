use anyhow::Result;
use colored::Colorize;
use serde_json;

use crate::cli::Cli;
use crate::core::config::NetScopeConfig;
use crate::core::dns_optimizer::DnsOptimizer;
use crate::i18n::I18n;

pub async fn run(cli: &Cli, _i18n: &I18n) -> Result<()> {
    if !cli.quiet && !cli.json && !cli.md {
        println!("{}", "🔧 DNS One-Click Optimizer".cyan().bold());
        println!("  Automatically detecting and configuring the best DNS server...\n");
    }

    let optimizer = DnsOptimizer::new();
    let test_domain = "www.google.com";
    let result = optimizer.optimize(test_domain).await;

    if let Some(data) = &result.data {
        if !cli.quiet && !cli.json && !cli.md {
            print_results(data);
        }

        let top_servers: Vec<_> = data.servers.iter()
            .filter(|s| s.avg_ms < 9999.0)
            .take(2)
            .collect();

        if !top_servers.is_empty() {
            let best = top_servers[0];
            let second = top_servers.get(1);

            let second_ip = second.cloned().map(|s| s.ip.clone());
            save_dns_config(best.ip.as_str(), second_ip);

            if !cli.quiet && !cli.json && !cli.md {
                print_recommendation(best, second.cloned().cloned());
            } else if cli.json {
                print_json_output(best, second.cloned().cloned());
            }
        } else {
            println!("{}", "❌ All DNS servers are unreachable. Please check your network connection.".red().bold());
        }
    } else {
        println!("{}", "❌ DNS optimization failed. Please try again later.".red().bold());
    }

    Ok(())
}

fn save_dns_config(primary: &str, secondary: Option<String>) {
    let mut config = NetScopeConfig::load();
    config.set_preferred_dns(primary, secondary.as_deref());
    if config.save().is_ok() {
        println!("{} DNS configuration saved to preferences", "ℹ️ ".blue());
    }
}

fn print_results(data: &crate::core::dns_optimizer::DnsOptimizerResult) {
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".cyan());
    println!("{}  {}", "║".cyan(), "📊 DNS Server Test Results".bold());
    println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());

    println!("{}  {:<22} {:>10} {:>10} {:>10}",
        "║".cyan(),
        "DNS Server".bold(),
        "Latency".bold(),
        "Success".bold(),
        "Score".bold()
    );
    println!("{}  {}", "║".cyan(), "-".repeat(55).cyan());

    for (i, server) in data.servers.iter().enumerate() {
        let is_best = i == 0;
        let marker = if is_best { "⭐ " } else { "   " };

        let latency_str = if server.avg_ms < 9999.0 {
            format!("{:.1}ms", server.avg_ms)
        } else {
            "Timeout".to_string()
        };

        let success_str = format!("{:.0}%", server.success_rate);

        let score_color = if server.score >= 80 {
            "green"
        } else if server.score >= 50 {
            "yellow"
        } else {
            "red"
        };

        let line = format!(
            "  {}{:<19} {:>10} {:>10} {:>10}",
            marker,
            server.server,
            latency_str,
            success_str,
            server.score
        );

        let final_line = match score_color {
            "green" => line.green().bold().to_string(),
            "yellow" => line.yellow().to_string(),
            _ => line.red().to_string(),
        };

        println!("{}  {}", "║".cyan(), final_line);
    }

    println!("{}", "╚══════════════════════════════════════════════════════════════╝".cyan());
    println!();
}

fn print_recommendation(best: &crate::core::dns_optimizer::DnsServerResult, second: Option<crate::core::dns_optimizer::DnsServerResult>) {
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".cyan());
    println!("{}  {}", "║".cyan(), "✅ Recommended DNS Server".bold().green());
    println!("{}    {} ({})", "║".cyan(), best.server.bold(), best.ip);
    println!("{}    Description: {}", "║".cyan(), best.description);
    println!("{}    Latency: {:.1}ms | Success: {:.0}% | Score: {}", "║".cyan(), best.avg_ms, best.success_rate, best.score);
    
    if let Some(ref s) = second {
        println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());
        println!("{}  {}", "║".cyan(), "📌 Secondary DNS (Backup)".bold().yellow());
        println!("{}    {} ({})", "║".cyan(), s.server, s.ip);
        println!("{}    Latency: {:.1}ms", "║".cyan(), s.avg_ms);
    }
    
    println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());
    println!("{}  {}", "║".cyan(), "🔧 Configuration Commands".bold());
    println!();

    print_linux_config(best.ip.as_str(), second.as_ref().map(|s| s.ip.as_str()));
    print_windows_config(best.ip.as_str(), second.as_ref().map(|s| s.ip.as_str()));
    print_macos_config(best.ip.as_str(), second.as_ref().map(|s| s.ip.as_str()));
    
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".cyan());
    println!();
    println!("💡 Tip: After configuration, test with: {}", "netscope dns www.google.com".green().bold());
}

fn print_linux_config(primary: &str, secondary: Option<&str>) {
    println!("{}  {}", "║".cyan(), "🐧 Linux:".bold().yellow());
    println!("{}    {}", "║".cyan(), "# Option 1: Direct resolv.conf (temporary)".dimmed());
    println!("{}    sudo bash -c 'echo -e \"nameserver {}\" > /etc/resolv.conf'", "║".cyan(), primary);
    if let Some(s) = secondary {
        println!("{}    sudo bash -c 'echo -e \"nameserver {}\" >> /etc/resolv.conf'", "║".cyan(), s);
    }
    println!();
    println!("{}    {}", "║".cyan(), "# Option 2: systemd-resolved".dimmed());
    println!("{}    sudo systemd-resolve --set-dns={} --set-domain=~.", "║".cyan(), primary);
    if let Some(s) = secondary {
        println!("{}    sudo systemd-resolve --set-dns={} --set-domain=~.", "║".cyan(), s);
    }
    println!();
    println!("{}    {}", "║".cyan(), "# Option 3: NetworkManager (nmcli)".dimmed());
    let nmcli_cmd1 = format!("sudo nmcli con mod \"$(nmcli con show --active | grep -v NAME | head -1 | awk '{{print $1}}')\" ipv4.dns \"{}\"", primary);
    println!("{}    {}", "║".cyan(), nmcli_cmd1);
    if let Some(s) = secondary {
        let nmcli_cmd2 = format!("sudo nmcli con mod \"$(nmcli con show --active | grep -v NAME | head -1 | awk '{{print $1}}')\" +ipv4.dns \"{}\"", s);
        println!("{}    {}", "║".cyan(), nmcli_cmd2);
    }
    let nmcli_cmd3 = "sudo nmcli con up \"$(nmcli con show --active | grep -v NAME | head -1 | awk '{print $1}')\"";
    println!("{}    {}", "║".cyan(), nmcli_cmd3);
    println!();
}

fn print_windows_config(primary: &str, secondary: Option<&str>) {
    println!("{}  {}", "║".cyan(), "🪟 Windows (PowerShell as Administrator):".bold().yellow());
    let secondary_str = secondary.map(|s| format!(", '{}'", s)).unwrap_or_default();
    println!("{}    Get-NetAdapter | ForEach-Object {{ Set-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex -ServerAddresses ('{}'{} ) }}", "║".cyan(), primary, secondary_str);
    println!();
    println!("{}    {}", "║".cyan(), "# Alternative: Using netsh".dimmed());
    println!("{}    netsh interface ip set dns name=\"Ethernet\" static {} primary", "║".cyan(), primary);
    if let Some(s) = secondary {
        println!("{}    netsh interface ip add dns name=\"Ethernet\" {} index=2", "║".cyan(), s);
    }
    println!();
}

fn print_macos_config(primary: &str, secondary: Option<&str>) {
    println!("{}  {}", "║".cyan(), "🍎 macOS:".bold().yellow());
    println!("{}    {}", "║".cyan(), "# Option 1: Using networksetup".dimmed());
    let adapters = "$(networksetup -listallhardwareports | grep -A1 'Wi-Fi' | grep Device | awk '{print $2}')";
    println!("{}    networksetup -setdnsservers {} {}", "║".cyan(), adapters, primary);
    if let Some(s) = secondary {
        println!("{}    networksetup -setdnsservers {} +{}", "║".cyan(), adapters, s);
    }
    println!();
    println!("{}    {}", "║".cyan(), "# Option 2: Using scutil (advanced)".dimmed());
    println!("{}    sudo scutil <<EOF", "║".cyan());
    println!("{}    d.init", "║".cyan());
    println!("{}    d.add ServerAddresses * {} {}", "║".cyan(), primary, secondary.unwrap_or(""));
    let scutil_cmd = "set State:/Network/Service/$(networksetup -listallhardwareports | grep -A1 'Wi-Fi' | grep Device | awk '{print $2}')/DNS";
    println!("{}    {}", "║".cyan(), scutil_cmd);
    println!("{}    EOF", "║".cyan());
    println!();
}

fn print_json_output(best: &crate::core::dns_optimizer::DnsServerResult, second: Option<crate::core::dns_optimizer::DnsServerResult>) {
    let json_output = serde_json::json!({
        "recommended_dns": best.server,
        "ip": best.ip,
        "latency_ms": best.avg_ms,
        "score": best.score,
        "secondary": second.as_ref().map(|s| serde_json::json!({
            "name": s.server,
            "ip": s.ip,
            "latency_ms": s.avg_ms
        })),
        "configuration": generate_config_json(best.ip.as_str(), second.as_ref().map(|s| s.ip.as_str()))
    });
    println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
}

fn generate_config_json(primary: &str, secondary: Option<&str>) -> serde_json::Value {
    serde_json::json!({
        "linux": {
            "resolv_conf": format!("nameserver {}\n{}", primary, 
                secondary.map(|s| format!("nameserver {}", s)).unwrap_or_default()),
            "systemd_resolved": format!("systemd-resolve --set-dns={} --set-domain=~.", primary),
            "networkmanager": format!("nmcli con mod <interface> ipv4.dns \"{}\"", primary)
        },
        "windows": {
            "powershell": format!("Set-DnsClientServerAddress -InterfaceIndex <index> -ServerAddresses ('{}')", primary),
            "netsh": format!("netsh interface ip set dns \"<interface>\" static {} primary", primary)
        },
        "macos": {
            "networksetup": format!("networksetup -setdnsservers <interface> {}", primary),
            "scutil": format!("ServerAddresses: [\"{}\"]", primary)
        }
    })
}
