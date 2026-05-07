use crate::error::ProbeResult;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsServerResult {
    pub server: String,
    pub ip: String,
    pub description: String,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub success_rate: f64,
    pub score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsOptimizerResult {
    pub target_domain: String,
    pub servers: Vec<DnsServerResult>,
    pub best_server: Option<String>,
    pub recommendation: String,
}

pub struct DnsOptimizer {
    servers: Vec<DnsServerConfig>,
}

struct DnsServerConfig {
    name: String,
    ip: String,
    description: String,
}

impl DnsOptimizer {
    pub fn new() -> Self {
        DnsOptimizer {
            servers: vec![
                DnsServerConfig {
                    name: "Cloudflare".into(),
                    ip: "1.1.1.1".into(),
                    description: "Cloudflare DNS - 全球最快".into(),
                },
                DnsServerConfig {
                    name: "Cloudflare Secondary".into(),
                    ip: "1.0.0.1".into(),
                    description: "Cloudflare 备用 DNS".into(),
                },
                DnsServerConfig {
                    name: "Google".into(),
                    ip: "8.8.8.8".into(),
                    description: "Google DNS - 全球化".into(),
                },
                DnsServerConfig {
                    name: "Google Secondary".into(),
                    ip: "8.8.4.4".into(),
                    description: "Google 备用 DNS".into(),
                },
                DnsServerConfig {
                    name: "Quad9".into(),
                    ip: "9.9.9.9".into(),
                    description: "Quad9 DNS - 安全优先".into(),
                },
                DnsServerConfig {
                    name: "AliDNS".into(),
                    ip: "223.5.5.5".into(),
                    description: "阿里 DNS - 国内优化".into(),
                },
                DnsServerConfig {
                    name: "AliDNS Secondary".into(),
                    ip: "223.6.6.6".into(),
                    description: "阿里 DNS 备用".into(),
                },
                DnsServerConfig {
                    name: "Tencent DNS".into(),
                    ip: "119.29.29.29".into(),
                    description: "腾讯 DNS".into(),
                },
                DnsServerConfig {
                    name: "DNSPod".into(),
                    ip: "119.28.28.28".into(),
                    description: "DNSPod DNS".into(),
                },
                DnsServerConfig {
                    name: "114 DNS".into(),
                    ip: "114.114.114.114".into(),
                    description: "114 DNS".into(),
                },
            ],
        }
    }

    pub async fn optimize(&self, target_domain: &str) -> ProbeResult<DnsOptimizerResult> {
        let start = Instant::now();
        let mut results: Vec<DnsServerResult> = Vec::new();

        for server_config in &self.servers {
            let result = self.test_dns_server(&server_config.ip, &server_config.name, &server_config.description, target_domain).await;
            results.push(result);
        }

        results.sort_by(|a, b| b.score.cmp(&a.score));

        let best_server = results.first().map(|r| r.server.clone());
        let recommendation = self.generate_recommendation(&results);

        ProbeResult::ok(
            DnsOptimizerResult {
                target_domain: target_domain.to_string(),
                servers: results,
                best_server,
                recommendation,
            },
            start.elapsed(),
        )
    }

    async fn test_dns_server(&self, dns_ip: &str, name: &str, description: &str, target: &str) -> DnsServerResult {
        let mut latencies: Vec<f64> = Vec::new();
        let attempts = 3;
        let mut success_count = 0;

        for _ in 0..attempts {
            match self.measure_dns_latency(target).await {
                Some(ms) => {
                    latencies.push(ms);
                    success_count += 1;
                }
                None => {}
            }
        }

        let avg_ms = if latencies.is_empty() {
            9999.0
        } else {
            latencies.iter().sum::<f64>() / latencies.len() as f64
        };

        let min_ms = latencies.iter().cloned().fold(f64::MAX, f64::min);
        let max_ms = latencies.iter().cloned().fold(0.0, f64::max);
        let success_rate = (success_count as f64 / attempts as f64) * 100.0;

        let score = self.calculate_score(avg_ms, success_rate);

        DnsServerResult {
            server: name.to_string(),
            ip: dns_ip.to_string(),
            description: description.to_string(),
            avg_ms,
            min_ms,
            max_ms,
            success_rate,
            score,
        }
    }

    async fn measure_dns_latency(&self, target: &str) -> Option<f64> {
        let start = Instant::now();
        
        match tokio::time::timeout(
            Duration::from_secs(3),
            tokio::net::lookup_host((target, 443)),
        )
        .await
        {
            Ok(Ok(_)) => {}
            _ => return None,
        }

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        Some(elapsed)
    }

    fn calculate_score(&self, avg_ms: f64, success_rate: f64) -> u32 {
        let latency_score = if avg_ms < 10.0 {
            50
        } else if avg_ms < 30.0 {
            40
        } else if avg_ms < 50.0 {
            30
        } else if avg_ms < 100.0 {
            20
        } else if avg_ms < 200.0 {
            10
        } else {
            0
        };

        let success_score = if success_rate >= 100.0 {
            50
        } else if success_rate >= 90.0 {
            45
        } else if success_rate >= 70.0 {
            35
        } else if success_rate >= 50.0 {
            20
        } else {
            0
        };

        latency_score + success_score
    }

    fn generate_recommendation(&self, results: &[DnsServerResult]) -> String {
        if let Some(best) = results.first() {
            if best.avg_ms > 500.0 {
                "所有 DNS 服务器延迟都较高，建议检查网络连接".to_string()
            } else if best.success_rate < 50.0 {
                "DNS 解析不稳定，建议使用多个 DNS 服务器".to_string()
            } else if best.server.contains("AliDNS") || best.server.contains("Tencent") || best.server.contains("DNSPod") {
                format!(
                    "推荐使用 {} ({})，国内优化，延迟 {:.1}ms",
                    best.server, best.ip, best.avg_ms
                )
            } else {
                format!(
                    "推荐使用 {} ({})，延迟 {:.1}ms",
                    best.server, best.ip, best.avg_ms
                )
            }
        } else {
            "无法完成 DNS 优化，建议检查网络连接".to_string()
        }
    }
}

impl Default for DnsOptimizer {
    fn default() -> Self {
        Self::new()
    }
}