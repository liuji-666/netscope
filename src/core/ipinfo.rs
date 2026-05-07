use crate::core::IpInfoResult;
use crate::error::ProbeResult;
use std::time::Instant;

/// 获取本机公网 IP 信息
pub async fn get_my_ip() -> ProbeResult<IpInfoResult> {
    let start = Instant::now();
    
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => return ProbeResult::error(&format!("Failed to create HTTP client: {}", e), start.elapsed()),
    };
    
    // 尝试多个 IP 查询服务
    let services = vec![
        "https://api.ipify.org?format=json",
        "https://ifconfig.me/ip",
    ];
    
    for service in services {
        if let Ok(response) = client.get(service).send().await {
            if let Ok(text) = response.text().await {
                let ip = text.trim().to_string();
                if !ip.is_empty() {
                    return ProbeResult::ok(
                        IpInfoResult {
                            ip,
                            hostname: None,
                            city: None,
                            region: None,
                            country: None,
                            org: None,
                            timezone: None,
                        },
                        start.elapsed(),
                    );
                }
            }
        }
    }
    
    ProbeResult::error("Failed to get IP info", start.elapsed())
}