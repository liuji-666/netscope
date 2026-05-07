use crate::core::{PingResult, PingHop, resolve_ip as core_resolve_ip, format_socket_addr};
use crate::error::ProbeResult;
use std::time::{Duration, Instant};

/// TCP Ping - 尝试连接常见端口来模拟 ping
pub async fn tcp_ping(target: &str, count: u32, timeout_secs: u64) -> ProbeResult<PingResult> {
    let start = Instant::now();
    let mut rtts = Vec::new();
    let mut hops = Vec::new();
    
    // 尝试的端口顺序
    let ports = vec![443, 80, 22, 8080];
    let mut resolved_ip = String::new();
    
    // 先解析一次IP
    if let Ok(ip) = core_resolve_ip(target).await {
        resolved_ip = ip;
    }
    
    for seq in 0..count {
        let _packet_start = Instant::now();
        let mut success = false;
        let mut rtt_ms = None;
        
        // 尝试多个端口，直到找到一个可连接的
        for &port in &ports {
            let port_start = Instant::now();
            let addr = format_socket_addr(&resolved_ip, port);
            
            match tokio::time::timeout(
                Duration::from_secs(timeout_secs),
                tokio::net::TcpStream::connect(&addr),
            )
            .await
            {
                Ok(Ok(_)) => {
                    let port_rtt = port_start.elapsed().as_secs_f64() * 1000.0;
                    rtt_ms = Some(port_rtt);
                    rtts.push(port_rtt);
                    success = true;
                    break;
                }
                _ => continue,
            }
        }
        
        hops.push(PingHop {
            seq,
            success,
            rtt_ms,
        });
        
        // 间隔一小段时间
        if seq < count - 1 {
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }
    
    // 计算统计数据
    let packets_recv = rtts.len() as u32;
    let loss_pct = if count == 0 {
        0.0
    } else {
        ((count - packets_recv) as f64 / count as f64) * 100.0
    };
    
    let (rtt_min_ms, rtt_avg_ms, rtt_max_ms, rtt_p95_ms, rtt_stddev_ms) =
        if !rtts.is_empty() {
            let mut sorted = rtts.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            
            let min = sorted.first().unwrap_or(&0.0);
            let max = sorted.last().unwrap_or(&0.0);
            let avg = rtts.iter().sum::<f64>() / rtts.len() as f64;
            let p95_index = ((rtts.len() as f64 * 0.95).ceil() as usize).min(rtts.len() - 1);
            let p95 = sorted[p95_index];
            
            let variance = rtts
                .iter()
                .map(|&x| (x - avg) * (x - avg))
                .sum::<f64>()
                / rtts.len() as f64;
            let stddev = variance.sqrt();
            
            (*min, avg, *max, p95, stddev)
        } else {
            (0.0, 0.0, 0.0, 0.0, 0.0)
        };
    
    ProbeResult::ok(
        PingResult {
            target: target.to_string(),
            resolved_ip,
            packets_sent: count,
            packets_recv,
            loss_pct,
            rtt_min_ms,
            rtt_avg_ms,
            rtt_max_ms,
            rtt_p95_ms,
            rtt_stddev_ms,
            hops,
        },
        start.elapsed(),
    )
}