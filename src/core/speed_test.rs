use crate::core::{SpeedResult, SpeedServer};
use crate::error::ProbeResult;
use std::time::{Duration, Instant};

const TEST_SERVERS: &[(&str, &str)] = &[
    ("www.google.com", "Google"),
    ("www.cloudflare.com", "Cloudflare"),
    ("www.microsoft.com", "Microsoft"),
];

pub async fn run_speed_test() -> ProbeResult<SpeedResult> {
    test_speed().await
}

pub async fn test_speed() -> ProbeResult<SpeedResult> {
    let start = Instant::now();

    let (server, latency_ms) = select_best_server().await;
    
    // 简化版：只测试连接延迟
    let (download_mbps, upload_mbps) = (0.0, 0.0);
    let jitter_ms = calculate_jitter(&server.name).await;

    ProbeResult::ok(
        SpeedResult {
            server,
            download_mbps,
            upload_mbps,
            latency_ms,
            jitter_ms,
        },
        start.elapsed(),
    )
}

async fn select_best_server() -> (SpeedServer, f64) {
    let mut best_server = SpeedServer {
        name: TEST_SERVERS[0].0.to_string(),
        location: TEST_SERVERS[0].1.to_string(),
        distance_km: 0.0,
    };
    let mut best_latency = f64::MAX;

    for &(host, location) in TEST_SERVERS {
        let latency = ping_server(host).await;
        if latency < best_latency {
            best_latency = latency;
            best_server = SpeedServer {
                name: host.to_string(),
                location: location.to_string(),
                distance_km: estimate_distance(latency),
            };
        }
    }

    (best_server, best_latency)
}

async fn ping_server(host: &str) -> f64 {
    let start = Instant::now();
    
    let result = tokio::time::timeout(
        Duration::from_secs(3),
        tokio::net::TcpStream::connect(format!("{}:443", host)),
    ).await;
    
    match result {
        Ok(Ok(_)) => start.elapsed().as_secs_f64() * 1000.0,
        _ => f64::MAX,
    }
}

fn estimate_distance(latency_ms: f64) -> f64 {
    latency_ms * 150.0
}

async fn calculate_jitter(host: &str) -> f64 {
    let mut latencies = Vec::new();
    
    for _ in 0..5 {
        let latency = ping_server(host).await;
        if latency < f64::MAX {
            latencies.push(latency);
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    if latencies.len() < 2 {
        return 0.0;
    }

    let mean = latencies.iter().sum::<f64>() / latencies.len() as f64;
    let variance: f64 = latencies.iter().map(|l| (*l - mean).powi(2)).sum::<f64>() / latencies.len() as f64;
    
    variance.sqrt()
}