use crate::core::{PortScanResult, PortInfo, PortState, guess_service, resolve_ip as core_resolve_ip, format_socket_addr};
use crate::config::common_ports;
use crate::error::ProbeResult;
use std::sync::Arc;
use std::time::{Duration, Instant};

const MAX_CONCURRENCY: usize = 200;
const VALID_PORT_RANGE: std::ops::RangeInclusive<u16> = 1..=65535;

pub async fn scan_top_ports(target: &str, top: usize, concurrency: usize) -> ProbeResult<PortScanResult> {
    let ports = common_ports(top);
    scan_ports_internal(target, &ports, concurrency).await
}

pub async fn scan_port(target: &str, port: u16) -> PortInfo {
    let addr = format_socket_addr(target, port);

    let state = match tokio::time::timeout(
        Duration::from_secs(2),
        tokio::net::TcpStream::connect(&addr),
    )
    .await
    {
        Ok(Ok(_)) => PortState::Open,
        Ok(Err(_)) => PortState::Closed,
        Err(_) => PortState::Timeout,
    };

    PortInfo {
        port,
        state,
        service: guess_service(port),
        banner: None,
    }
}

pub async fn scan_ports(target: &str, ports: &[u16], concurrency: usize) -> ProbeResult<PortScanResult> {
    scan_ports_internal(target, ports, concurrency).await
}

async fn scan_ports_internal(target: &str, ports: &[u16], concurrency: usize) -> ProbeResult<PortScanResult> {
    let start = Instant::now();

    let safe_concurrency = concurrency.min(MAX_CONCURRENCY);

    let valid_ports: Vec<u16> = ports.iter()
        .filter(|&&port| VALID_PORT_RANGE.contains(&port))
        .cloned()
        .collect();

    if valid_ports.is_empty() {
        return ProbeResult::error("No valid ports specified", start.elapsed());
    }

    let ip = match core_resolve_ip(target).await {
        Ok(ip) => ip,
        Err(e) => return ProbeResult::error(&format!("Failed to resolve {}: {}", target, e), start.elapsed()),
    };

    if is_private_ip(&ip) {
        return ProbeResult::error("Private IP scanning is restricted", start.elapsed());
    }

    let semaphore = Arc::new(tokio::sync::Semaphore::new(safe_concurrency));
    let mut handles = Vec::new();

    for &port in &valid_ports {
        let sem = semaphore.clone();
        let ip_clone = ip.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            scan_port(&ip_clone, port).await
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(port_info) = handle.await {
            results.push(port_info);
        }
    }

    results.sort_by_key(|p| p.port);

    let open_count = results.iter().filter(|p| p.state == PortState::Open).count() as u32;
    let closed_count = results.iter().filter(|p| p.state == PortState::Closed).count() as u32;
    let filtered_count = results.iter().filter(|p| p.state == PortState::Filtered).count() as u32;
    let scan_duration_ms = start.elapsed().as_millis() as u64;

    ProbeResult::ok(
        PortScanResult {
            target: target.to_string(),
            ip,
            ports: results,
            open_count,
            closed_count,
            filtered_count,
            scan_duration_ms,
        },
        start.elapsed(),
    )
}

fn is_private_ip(ip: &str) -> bool {
    if ip.starts_with("10.") {
        return true;
    }
    if ip.starts_with("192.168.") {
        return true;
    }
    if ip.starts_with("172.") {
        if let Some(second) = ip.split('.').nth(1) {
            if let Ok(num) = second.parse::<u8>() {
                return num >= 16 && num <= 31;
            }
        }
    }
    if ip.starts_with("127.") {
        return true;
    }
    false
}