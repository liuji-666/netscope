use crate::core::{TraceResult, TraceHop, resolve_ip};
use crate::error::ProbeResult;

pub async fn trace(target: &str, _max_hops: u32) -> ProbeResult<TraceResult> {
    use std::time::Instant;

    let start = Instant::now();

    // 简化实现说明：完整traceroute需要原始socket权限，
    // 当前版本只执行基本连接测试
    let ip = resolve_ip(target).await.ok();

    let hops = vec![TraceHop {
        hop: 1,
        ip: ip.clone(),
        hostname: None,
        avg_ms: None,
        loss_pct: 0.0,
        asn: None,
        org: Some("Note: Full traceroute requires admin privileges".to_string()),
        geo: None,
    }];

    let total_hops = 1;

    ProbeResult::ok(
        TraceResult {
            target: target.to_string(),
            hops: hops.clone(),
            total_hops,
        },
        start.elapsed(),
    )
}

// 简单 IP 反查（简化版）
#[allow(dead_code)]
async fn reverse_lookup(_ip: &str) -> Option<String> {
    // 暂时不实现反查
    None
}