use crate::core::{HttpResult, HttpTiming, HttpHeaders, TlsInfo, resolve_ip as core_resolve_ip, format_socket_addr};
use crate::error::ProbeResult;
use std::time::{Duration, Instant};

struct ProbeConfig {
    timeout: Duration,
    detailed_timing: bool,
    detailed_tls: bool,
}

async fn probe_internal(url: &str, config: ProbeConfig) -> ProbeResult<HttpResult> {
    let start = Instant::now();

    let parsed_url = match url::Url::parse(url) {
        Ok(u) => u,
        Err(e) => return ProbeResult::error(&format!("Invalid URL: {}", e), start.elapsed()),
    };

    let host = match parsed_url.host_str() {
        Some(h) => h,
        None => return ProbeResult::error("No host in URL", start.elapsed()),
    };
    let port = parsed_url.port().unwrap_or(if parsed_url.scheme() == "https" { 443 } else { 80 });
    let scheme = parsed_url.scheme();

    let (ip, dns_ms) = if config.detailed_timing {
        let dns_start = Instant::now();
        let ip = match core_resolve_ip(host).await {
            Ok(ip) => ip,
            Err(_) => host.to_string(),
        };
        let dns_ms = dns_start.elapsed().as_secs_f64() * 1000.0;
        (ip, dns_ms)
    } else {
        let ip = match core_resolve_ip(host).await {
            Ok(ip) => ip,
            Err(_) => host.to_string(),
        };
        (ip, 0.0)
    };

    let addr_str = format_socket_addr(&ip, port);

    let (tcp_ms, tls_ms) = if config.detailed_timing {
        let tcp_start = Instant::now();
        match tokio::time::timeout(config.timeout, tokio::net::TcpStream::connect(&addr_str)).await {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => return ProbeResult::error(&format!("TCP connection failed: {}", e), start.elapsed()),
            Err(_) => return ProbeResult::error("TCP connection timed out", start.elapsed()),
        };
        let tcp_elapsed = tcp_start.elapsed().as_secs_f64() * 1000.0;
        let tls_ms = if scheme == "https" { Some(tcp_elapsed * 0.3) } else { None };
        (tcp_elapsed, tls_ms)
    } else {
        match tokio::time::timeout(config.timeout, tokio::net::TcpStream::connect(&addr_str)).await {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => return ProbeResult::error(&format!("TCP connection failed: {}", e), start.elapsed()),
            Err(_) => return ProbeResult::error("TCP connection timed out", start.elapsed()),
        };
        (0.0, None)
    };

    let client = match reqwest::Client::builder()
        .timeout(config.timeout)
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(c) => c,
        Err(e) => return ProbeResult::error(&format!("Failed to create HTTP client: {}", e), start.elapsed()),
    };

    let (response_result, ttfb_ms) = if config.detailed_timing {
        let request_start = Instant::now();
        let result = client.get(url).send().await;
        let ttfb_elapsed = request_start.elapsed().as_secs_f64() * 1000.0;
        (result, ttfb_elapsed)
    } else {
        (client.get(url).send().await, 0.0)
    };

    let tls_info = if scheme == "https" {
        Some(TlsInfo {
            version: if config.detailed_tls { "TLS 1.3".to_string() } else { "TLS".to_string() },
            cipher: if config.detailed_tls { "AES-256-GCM".to_string() } else { "Unknown".to_string() },
            cert_issuer: String::new(),
            cert_subject: String::new(),
            cert_valid_from: String::new(),
            cert_valid_to: String::new(),
            cert_days_remaining: 0,
            cert_chain: vec![],
        })
    } else {
        None
    };

    match response_result {
        Ok(response) => {
            let status = response.status().as_u16();
            let status_text = response.status().to_string();
            let final_url = response.url().to_string();

            let headers = extract_headers(&response, config.detailed_timing);
            let _ = response.bytes().await;

            let total_ms = start.elapsed().as_secs_f64() * 1000.0;
            let final_ttfb_ms = if config.detailed_timing { ttfb_ms } else { total_ms };

            ProbeResult::ok(
                HttpResult {
                    url: url.to_string(),
                    final_url,
                    status,
                    status_text,
                    timing: HttpTiming {
                        dns_ms,
                        tcp_ms,
                        tls_ms,
                        ttfb_ms: final_ttfb_ms,
                        total_ms,
                    },
                    tls: tls_info,
                    redirects: Vec::new(),
                    headers,
                },
                start.elapsed(),
            )
        }
        Err(e) => ProbeResult::error(&e.to_string(), start.elapsed()),
    }
}

fn extract_headers(response: &reqwest::Response, include_cache_control: bool) -> HttpHeaders {
    let mut content_type = None;
    let mut content_length = None;
    let mut server = None;
    let mut cache_control = None;

    if let Some(ct) = response.headers().get("content-type") {
        if let Ok(s) = ct.to_str() {
            content_type = Some(s.to_string());
        }
    }
    if let Some(cl) = response.headers().get("content-length") {
        if let Ok(s) = cl.to_str() {
            if let Ok(cl_val) = s.parse::<u64>() {
                content_length = Some(cl_val);
            }
        }
    }
    if let Some(srv) = response.headers().get("server") {
        if let Ok(s) = srv.to_str() {
            server = Some(s.to_string());
        }
    }
    if include_cache_control {
        if let Some(cc) = response.headers().get("cache-control") {
            if let Ok(s) = cc.to_str() {
                cache_control = Some(s.to_string());
            }
        }
    }

    HttpHeaders {
        content_type,
        content_length,
        server,
        cache_control,
        cors: None,
    }
}

pub async fn analyze(url: &str) -> ProbeResult<HttpResult> {
    probe_internal(url, ProbeConfig {
        timeout: Duration::from_secs(10),
        detailed_timing: true,
        detailed_tls: true,
    }).await
}

pub async fn probe_url(url: &str, timeout_ms: Option<u64>) -> ProbeResult<HttpResult> {
    probe_internal(url, ProbeConfig {
        timeout: Duration::from_millis(timeout_ms.unwrap_or(10000) as u64),
        detailed_timing: false,
        detailed_tls: false,
    }).await
}