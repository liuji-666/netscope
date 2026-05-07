use crate::core::DnsResult;
use crate::core::PingResult;
use crate::core::TraceResult;
use crate::core::{PortScanResult, PortState};
use crate::core::HttpResult;
use crate::error::{ProbeResult, ProbeStatus};
use crate::i18n::I18n;

#[derive(Debug, Clone)]
pub struct ScoreResult {
    pub score: u32,
    pub grade: String,
    pub diagnosis: Vec<DiagnosisItem>,
}

#[derive(Debug, Clone)]
pub struct DiagnosisItem {
    pub severity: Severity,
    pub category: String,
    pub issue: String,
    pub cause: String,
    pub solution: String,
    pub impact: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

pub fn evaluate(
    dns: &ProbeResult<DnsResult>,
    ping: &ProbeResult<PingResult>,
    _trace: &ProbeResult<TraceResult>,
    ports: &ProbeResult<PortScanResult>,
    http: &ProbeResult<HttpResult>,
    i18n: &I18n,
) -> ScoreResult {
    let score: f64;
    let mut diagnosis = Vec::new();

    let mut dns_score = 100.0;
    let mut ping_score = 100.0;
    let mut port_score = 100.0;
    let mut http_score = 100.0;

    match &dns.data {
        Some(dns_data) => {
            if !dns_data.consistent {
                dns_score -= 30.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Critical,
                    category: "DNS".to_string(),
                    issue: "DNS解析器不一致".to_string(),
                    cause: "多个DNS解析器返回不同的IP地址，可能存在DNS劫持或污染".to_string(),
                    solution: "建议切换到可靠的公共DNS服务器，如Cloudflare (1.1.1.1) 或阿里DNS (223.5.5.5)".to_string(),
                    impact: "可能导致访问错误的网站或被重定向到恶意站点".to_string(),
                });
            }
            if !dns_data.has_aaaa {
                dns_score -= 10.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Info,
                    category: "DNS".to_string(),
                    issue: "缺少AAAA记录".to_string(),
                    cause: "目标网站不支持IPv6协议".to_string(),
                    solution: "如果需要IPv6支持，请联系网站管理员".to_string(),
                    impact: "无法使用IPv6网络访问该网站".to_string(),
                });
            }
        }
        None => {
            if dns.status == ProbeStatus::Error {
                dns_score = 0.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Critical,
                    category: "DNS".to_string(),
                    issue: "DNS解析失败".to_string(),
                    cause: "无法解析目标域名，可能是网络问题或DNS服务器故障".to_string(),
                    solution: "1. 检查网络连接\n2. 尝试更换DNS服务器\n3. 检查防火墙设置".to_string(),
                    impact: "无法访问目标网站".to_string(),
                });
            }
        }
    }

    match &ping.data {
        Some(ping_data) => {
            if ping_data.loss_pct > 50.0 {
                ping_score -= 40.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Critical,
                    category: "网络连通性".to_string(),
                    issue: "严重丢包".to_string(),
                    cause: "网络不稳定，可能是网络拥堵、路由器问题或运营商限制".to_string(),
                    solution: "1. 重启路由器\n2. 切换网络（WiFi/有线）\n3. 联系ISP排查问题".to_string(),
                    impact: "网络体验极差，网页加载缓慢或无法加载".to_string(),
                });
            } else if ping_data.loss_pct > 10.0 {
                ping_score -= 20.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Warning,
                    category: "网络连通性".to_string(),
                    issue: "高丢包率".to_string(),
                    cause: "网络质量不佳，可能是无线信号弱或网络拥堵".to_string(),
                    solution: "1. 靠近路由器\n2. 切换到有线连接\n3. 避开网络高峰时段".to_string(),
                    impact: "部分数据需要重传，影响上网体验".to_string(),
                });
            } else if ping_data.loss_pct > 0.0 {
                ping_score -= 5.0;
            }

            if ping_data.rtt_avg_ms > 500.0 {
                ping_score -= 30.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Critical,
                    category: "网络延迟".to_string(),
                    issue: "极高延迟".to_string(),
                    cause: "目标服务器距离遥远或网络路由不佳".to_string(),
                    solution: "1. 尝试使用CDN加速\n2. 切换到更近的服务器\n3. 使用VPN优化路由".to_string(),
                    impact: "网页加载极慢，视频卡顿".to_string(),
                });
            } else if ping_data.rtt_avg_ms > 200.0 {
                ping_score -= 15.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Warning,
                    category: "网络延迟".to_string(),
                    issue: "高延迟".to_string(),
                    cause: "网络延迟较高，可能是跨运营商或国际连接".to_string(),
                    solution: "1. 使用本地镜像或CDN\n2. 尝试不同的网络连接".to_string(),
                    impact: "网页响应较慢".to_string(),
                });
            }
        }
        None => {
            if ping.status == ProbeStatus::Error {
                ping_score = 0.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Critical,
                    category: "网络连通性".to_string(),
                    issue: "主机不可达".to_string(),
                    cause: "无法连接到目标主机，可能是防火墙阻止或网络隔离".to_string(),
                    solution: "1. 检查防火墙设置\n2. 确认目标主机可访问\n3. 检查网络路由".to_string(),
                    impact: "完全无法访问目标网站".to_string(),
                });
            }
        }
    }

    if let Some(port_data) = &ports.data {
        let has_http = port_data.ports.iter().any(|p| p.port == 80 && p.state == PortState::Open);
        let has_https = port_data.ports.iter().any(|p| p.port == 443 && p.state == PortState::Open);

        if !has_http && !has_https {
            port_score -= 20.0;
            diagnosis.push(DiagnosisItem {
                severity: Severity::Warning,
                category: "端口安全".to_string(),
                issue: "HTTP/HTTPS端口未开放".to_string(),
                cause: "目标服务器可能未运行Web服务或端口被防火墙阻止".to_string(),
                solution: "1. 确认目标服务器正在运行\n2. 检查防火墙规则\n3. 使用正确的端口号".to_string(),
                impact: "无法通过浏览器访问该网站".to_string(),
            });
        }

        for port_info in &port_data.ports {
            if port_info.state == PortState::Open {
                match port_info.port {
                    3306 | 5432 | 6379 | 27017 | 11211 => {
                        port_score -= 15.0;
                        diagnosis.push(DiagnosisItem {
                            severity: Severity::Warning,
                            category: "端口安全".to_string(),
                            issue: format!("数据库/缓存端口 {} 开放", port_info.port),
                            cause: "敏感服务端口暴露在公网上".to_string(),
                            solution: "1. 使用防火墙限制访问来源\n2. 考虑使用VPN访问\n3. 迁移到内网环境".to_string(),
                            impact: "可能遭受暴力破解或数据泄露风险".to_string(),
                        });
                    }
                    23 => {
                        port_score -= 15.0;
                        diagnosis.push(DiagnosisItem {
                            severity: Severity::Critical,
                            category: "端口安全".to_string(),
                            issue: "Telnet端口开放".to_string(),
                            cause: "使用不安全的Telnet协议".to_string(),
                            solution: "1. 立即关闭Telnet服务\n2. 改用SSH替代\n3. 加强安全配置".to_string(),
                            impact: "Telnet明文传输，密码可能被窃取".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    match &http.data {
        Some(http_data) => {
            if http_data.status >= 500 {
                http_score -= 30.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Critical,
                    category: "HTTP".to_string(),
                    issue: format!("服务器错误 HTTP {}", http_data.status),
                    cause: "目标服务器内部错误，无法正常处理请求".to_string(),
                    solution: "1. 稍后重试\n2. 联系网站管理员\n3. 尝试其他入口".to_string(),
                    impact: "无法正常访问网站功能".to_string(),
                });
            } else if http_data.status >= 400 {
                http_score -= 15.0;
            }

            if http_data.timing.ttfb_ms > 3000.0 {
                http_score -= 30.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Critical,
                    category: "HTTP性能".to_string(),
                    issue: "极慢的TTFB".to_string(),
                    cause: "服务器响应缓慢或网络传输延迟".to_string(),
                    solution: "1. 使用CDN加速\n2. 优化服务器性能\n3. 使用就近节点".to_string(),
                    impact: "网页加载体验极差".to_string(),
                });
            } else if http_data.timing.ttfb_ms > 1500.0 {
                http_score -= 15.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Warning,
                    category: "HTTP性能".to_string(),
                    issue: "较慢的TTFB".to_string(),
                    cause: "服务器响应时间较长".to_string(),
                    solution: "1. 优化后端响应速度\n2. 添加缓存层\n3. 使用CDN".to_string(),
                    impact: "网页首屏加载较慢".to_string(),
                });
            }

            if http_data.tls.is_none() && http_data.url.starts_with("https://") {
                http_score -= 20.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Warning,
                    category: "安全".to_string(),
                    issue: "HTTPS不可用".to_string(),
                    cause: "服务器未配置SSL证书或证书无效".to_string(),
                    solution: "1. 配置SSL证书\n2. 启用HTTPS\n3. 使用有效的证书".to_string(),
                    impact: "数据传输可能被窃听或篡改".to_string(),
                });
            }
        }
        None => {
            if http.status == ProbeStatus::Error {
                http_score -= 25.0;
                diagnosis.push(DiagnosisItem {
                    severity: Severity::Warning,
                    category: "HTTP".to_string(),
                    issue: "HTTP分析失败".to_string(),
                    cause: "无法建立HTTP连接，可能是端口未开放或服务器问题".to_string(),
                    solution: "1. 确认Web服务器运行正常\n2. 检查端口是否开放\n3. 尝试HTTP替代HTTPS".to_string(),
                    impact: "无法获取网站的详细信息".to_string(),
                });
            }
        }
    }

    score = f64::max(0.0, f64::min(100.0, dns_score * 0.25 + ping_score * 0.30 + port_score * 0.20 + http_score * 0.25));

    let grade = match score as u32 {
        90..=100 => i18n.t("grade_excellent").to_string(),
        75..=89 => i18n.t("grade_good").to_string(),
        60..=74 => i18n.t("grade_fair").to_string(),
        _ => i18n.t("grade_poor").to_string(),
    };

    ScoreResult {
        score: score as u32,
        grade,
        diagnosis,
    }
}