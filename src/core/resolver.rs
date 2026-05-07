use crate::core::{DnsResult, DnsRecord, ResolverResult, resolve_ip as core_resolve_ip};
use crate::error::ProbeResult;
use std::time::Instant;

pub async fn full_diagnosis(domain: &str, resolvers: &[String]) -> ProbeResult<DnsResult> {
    let start = Instant::now();

    let records_result = get_dns_records(domain).await;

    let mut resolver_comparison = Vec::new();
    let mut has_aaaa = false;
    let mut all_ips = Vec::new();

    // 如果没有指定解析器，使用默认的
    let default_resolvers = vec![
        "8.8.8.8".to_string(),
        "1.1.1.1".to_string(),
        "223.5.5.5".to_string(),
    ];
    
    let resolvers_to_test = if resolvers.is_empty() {
        &default_resolvers
    } else {
        resolvers
    };

    for resolver in resolvers_to_test {
        let resolver_start = Instant::now();
        match resolve_with_resolver(domain, resolver).await {
            Ok(ip) => {
                let latency = resolver_start.elapsed().as_secs_f64() * 1000.0;
                resolver_comparison.push(ResolverResult {
                    resolver: resolver.clone(),
                    ip: ip.clone(),
                    latency_ms: latency,
                });
                all_ips.push(ip);
            }
            Err(_) => continue,
        }
    }

    let consistent = !all_ips.is_empty()
        && all_ips
            .iter()
            .all(|ip| ip == &all_ips[0]);

    // 不检查 DNSSEC，简化实现
    let dnssec = false;

    match records_result {
        Ok(records) => {
            has_aaaa = records.iter().any(|r| r.record_type == "AAAA");
            
            ProbeResult::ok(
                DnsResult {
                    domain: domain.to_string(),
                    records,
                    resolver_comparison,
                    consistent,
                    has_aaaa,
                    dnssec,
                },
                start.elapsed(),
            )
        }
        Err(e) => ProbeResult::error(&e.to_string(), start.elapsed()),
    }
}

async fn get_dns_records(domain: &str) -> Result<Vec<DnsRecord>, Box<dyn std::error::Error>> {
    let mut records = Vec::new();

    if let Ok(addrs) = tokio::net::lookup_host((domain, 0)).await {
        for addr in addrs {
            let ip = addr.ip();
            if ip.is_ipv4() {
                records.push(DnsRecord {
                    record_type: "A".into(),
                    value: ip.to_string(),
                    ttl: 300,
                });
            } else {
                records.push(DnsRecord {
                    record_type: "AAAA".into(),
                    value: ip.to_string(),
                    ttl: 300,
                });
            }
        }
    }

    if records.is_empty() {
        if let Ok(ip) = resolve_simple(domain).await {
            if ip.contains(':') {
                records.push(DnsRecord {
                    record_type: "AAAA".into(),
                    value: ip,
                    ttl: 300,
                });
            } else {
                records.push(DnsRecord {
                    record_type: "A".into(),
                    value: ip,
                    ttl: 300,
                });
            }
        }
    }

    Ok(records)
}

async fn resolve_simple(domain: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    core_resolve_ip(domain).await
}

async fn resolve_with_resolver(domain: &str, _resolver: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // 简化版本：始终使用系统解析器
    // 未来可以使用 trust-dns-resolver 或其他库来支持自定义DNS服务器
    resolve_simple(domain).await
}