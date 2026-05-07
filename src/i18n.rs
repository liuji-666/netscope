use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Language {
    English,
    Chinese,
}

impl std::str::FromStr for Language {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en" | "english" => Ok(Language::English),
            "zh" | "chinese" | "cn" => Ok(Language::Chinese),
            _ => Err(format!("Unknown language: {}", s)),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::English => write!(f, "en"),
            Language::Chinese => write!(f, "zh"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct I18n {
    lang: Language,
    translations: HashMap<String, String>,
}

impl I18n {
    pub fn new(lang: Language) -> Self {
        let translations = match lang {
            Language::English => load_english(),
            Language::Chinese => load_chinese(),
        };
        I18n { lang, translations }
    }

    pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
        self.translations.get(key).map(|s| s.as_str()).unwrap_or(key)
    }

    pub fn lang(&self) -> Language {
        self.lang
    }
}

fn load_english() -> HashMap<String, String> {
    let mut map = HashMap::new();
    
    map.insert("starting_diagnosis".into(), "Starting full-chain diagnosis...".to_string());
    map.insert("target".into(), "Target".to_string());
    map.insert("time".into(), "Time".to_string());
    map.insert("score".into(), "Score".to_string());
    map.insert("grade_excellent".into(), "Excellent".to_string());
    map.insert("grade_good".into(), "Good".to_string());
    map.insert("grade_fair".into(), "Fair".to_string());
    map.insert("grade_poor".into(), "Poor".to_string());
    
    map.insert("dns_resolution".into(), "DNS Resolution".to_string());
    map.insert("dns_no_records".into(), "No records found".to_string());
    map.insert("dns_all_agree".into(), "All resolvers agree".to_string());
    map.insert("dns_disagree".into(), "Resolvers disagree!".to_string());
    map.insert("dns_resolvers_checked".into(), "({} resolvers checked)".to_string());
    
    map.insert("connectivity".into(), "Connectivity".to_string());
    map.insert("ping_received".into(), "{}/{} received".to_string());
    map.insert("ping_loss".into(), "{:.1}% loss".to_string());
    map.insert("ping_rtt".into(), "RTT:  min={:.1}  avg={:.1}  max={:.1}  p95={:.1}  stddev={:.1}  ms".to_string());
    
    map.insert("route".into(), "Route".to_string());
    map.insert("route_no_hops".into(), "No hops detected".to_string());
    map.insert("route_hop".into(), "Hop".to_string());
    map.insert("route_ip".into(), "IP".to_string());
    map.insert("route_avg_ms".into(), "Avg(ms)".to_string());
    map.insert("route_loss".into(), "Loss".to_string());
    map.insert("route_bar".into(), "Bar".to_string());
    
    map.insert("port_scan".into(), "Port Scan".to_string());
    map.insert("port_open".into(), "open".to_string());
    map.insert("port_closed".into(), "closed".to_string());
    map.insert("port_filtered".into(), "filtered".to_string());
    map.insert("port_timeout".into(), "timeout".to_string());
    map.insert("port_summary".into(), "{} open / {} closed / {} filtered / {} timeout ({}ms)".to_string());
    
    map.insert("http_analysis".into(), "HTTP Analysis".to_string());
    map.insert("http_status".into(), "Status".to_string());
    map.insert("http_timing".into(), "Timing".to_string());
    map.insert("http_total".into(), "Total".to_string());
    map.insert("http_type".into(), "Type".to_string());
    map.insert("http_server".into(), "Server".to_string());
    map.insert("http_tls".into(), "TLS".to_string());
    map.insert("http_redirects".into(), "redirects".to_string());
    map.insert("http_error".into(), "error sending request".to_string());
    
    map.insert("warnings".into(), "Warnings".to_string());
    map.insert("suggestions".into(), "Suggestions".to_string());
    
    map.insert("error".into(), "error".to_string());
    map.insert("unknown_error".into(), "Unknown error".to_string());
    map.insert("timeout".into(), "timeout".to_string());
    map.insert("skipped".into(), "skipped".to_string());
    
    map.insert("warn_dns_hijack".into(), "DNS resolvers disagree — possible hijacking/pollution".to_string());
    map.insert("warn_no_aaaa".into(), "No AAAA record — IPv6 not available".to_string());
    map.insert("warn_high_loss".into(), "High packet loss: {:.1}%".to_string());
    map.insert("warn_severe_loss".into(), "Severe packet loss: {:.1}%".to_string());
    map.insert("warn_packet_loss".into(), "Packet loss: {:.1}%".to_string());
    map.insert("warn_high_latency".into(), "High latency: {:.1}ms".to_string());
    map.insert("warn_very_high_latency".into(), "Very high latency: {:.1}ms".to_string());
    map.insert("warn_server_error".into(), "Server error: HTTP {}".to_string());
    map.insert("warn_client_error".into(), "Client error: HTTP {}".to_string());
    map.insert("warn_slow_ttfb".into(), "Slow TTFB: {:.0}ms".to_string());
    map.insert("warn_very_slow_ttfb".into(), "Very slow TTFB: {:.0}ms".to_string());
    map.insert("warn_no_https".into(), "HTTPS not available".to_string());
    map.insert("warn_database_port".into(), "Database/cache port {} is open — consider restricting access".to_string());
    map.insert("warn_telnet".into(), "Telnet port (23) is open — insecure protocol".to_string());
    map.insert("warn_dns_failed".into(), "DNS resolution failed".to_string());
    map.insert("warn_host_unreachable".into(), "Host unreachable".to_string());
    map.insert("warn_http_failed".into(), "HTTP analysis failed".to_string());
    
    map
}

fn load_chinese() -> HashMap<String, String> {
    let mut map = HashMap::new();
    
    map.insert("starting_diagnosis".into(), "正在执行全链路诊断...".to_string());
    map.insert("target".into(), "目标".to_string());
    map.insert("time".into(), "时间".to_string());
    map.insert("score".into(), "评分".to_string());
    map.insert("grade_excellent".into(), "优秀".to_string());
    map.insert("grade_good".into(), "良好".to_string());
    map.insert("grade_fair".into(), "一般".to_string());
    map.insert("grade_poor".into(), "较差".to_string());
    
    map.insert("dns_resolution".into(), "DNS 解析".to_string());
    map.insert("dns_no_records".into(), "未找到记录".to_string());
    map.insert("dns_all_agree".into(), "所有解析器一致".to_string());
    map.insert("dns_disagree".into(), "解析器不一致!".to_string());
    map.insert("dns_resolvers_checked".into(), "({} 个解析器检查)".to_string());
    
    map.insert("connectivity".into(), "网络连通性".to_string());
    map.insert("ping_received".into(), "{}/{} 已接收".to_string());
    map.insert("ping_loss".into(), "丢包率 {:.1}%".to_string());
    map.insert("ping_rtt".into(), "RTT:  最小={:.1}  平均={:.1}  最大={:.1}  P95={:.1}  标准差={:.1}  ms".to_string());
    
    map.insert("route".into(), "路由追踪".to_string());
    map.insert("route_no_hops".into(), "未检测到路由".to_string());
    map.insert("route_hop".into(), "跳数".to_string());
    map.insert("route_ip".into(), "IP".to_string());
    map.insert("route_avg_ms".into(), "平均(ms)".to_string());
    map.insert("route_loss".into(), "丢包".to_string());
    map.insert("route_bar".into(), "进度条".to_string());
    
    map.insert("port_scan".into(), "端口扫描".to_string());
    map.insert("port_open".into(), "开放".to_string());
    map.insert("port_closed".into(), "关闭".to_string());
    map.insert("port_filtered".into(), "过滤".to_string());
    map.insert("port_timeout".into(), "超时".to_string());
    map.insert("port_summary".into(), "{} 开放 / {} 关闭 / {} 过滤 / {} 超时 ({}ms)".to_string());
    
    map.insert("http_analysis".into(), "HTTP 分析".to_string());
    map.insert("http_status".into(), "状态".to_string());
    map.insert("http_timing".into(), "耗时".to_string());
    map.insert("http_total".into(), "总计".to_string());
    map.insert("http_type".into(), "类型".to_string());
    map.insert("http_server".into(), "服务器".to_string());
    map.insert("http_tls".into(), "TLS".to_string());
    map.insert("http_redirects".into(), "重定向".to_string());
    map.insert("http_error".into(), "请求发送错误".to_string());
    
    map.insert("warnings".into(), "警告".to_string());
    map.insert("suggestions".into(), "建议".to_string());
    
    map.insert("error".into(), "错误".to_string());
    map.insert("unknown_error".into(), "未知错误".to_string());
    map.insert("timeout".into(), "超时".to_string());
    map.insert("skipped".into(), "已跳过".to_string());
    
    map.insert("warn_dns_hijack".into(), "DNS解析器不一致 — 可能存在劫持/污染".to_string());
    map.insert("warn_no_aaaa".into(), "没有 AAAA 记录 — IPv6 不可用".to_string());
    map.insert("warn_high_loss".into(), "高丢包率: {:.1}%".to_string());
    map.insert("warn_severe_loss".into(), "严重丢包: {:.1}%".to_string());
    map.insert("warn_packet_loss".into(), "丢包: {:.1}%".to_string());
    map.insert("warn_high_latency".into(), "高延迟: {:.1}ms".to_string());
    map.insert("warn_very_high_latency".into(), "极高延迟: {:.1}ms".to_string());
    map.insert("warn_server_error".into(), "服务器错误: HTTP {}".to_string());
    map.insert("warn_client_error".into(), "客户端错误: HTTP {}".to_string());
    map.insert("warn_slow_ttfb".into(), "慢 TTFB: {:.0}ms".to_string());
    map.insert("warn_very_slow_ttfb".into(), "极慢 TTFB: {:.0}ms".to_string());
    map.insert("warn_no_https".into(), "HTTPS 不可用".to_string());
    map.insert("warn_database_port".into(), "数据库/缓存端口 {} 开放 — 建议限制访问".to_string());
    map.insert("warn_telnet".into(), "Telnet 端口 (23) 开放 — 不安全协议".to_string());
    map.insert("warn_dns_failed".into(), "DNS 解析失败".to_string());
    map.insert("warn_host_unreachable".into(), "主机不可达".to_string());
    map.insert("warn_http_failed".into(), "HTTP 分析失败".to_string());
    
    map
}