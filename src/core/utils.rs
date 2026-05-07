/// 猜测端口对应的服务名称
pub fn guess_service(port: u16) -> Option<String> {
    let services = vec![
        (21, "FTP"),
        (22, "SSH"),
        (23, "Telnet"),
        (25, "SMTP"),
        (53, "DNS"),
        (80, "HTTP"),
        (110, "POP3"),
        (143, "IMAP"),
        (443, "HTTPS"),
        (993, "IMAPS"),
        (995, "POP3S"),
        (1433, "SQL Server"),
        (1521, "Oracle"),
        (3306, "MySQL"),
        (3389, "RDP"),
        (5432, "PostgreSQL"),
        (6379, "Redis"),
        (7001, "WebLogic"),
        (8000, "HTTP-8000"),
        (8080, "HTTP-Proxy"),
        (8088, "Hadoop"),
        (8443, "HTTPS-Alt"),
        (8500, "Consul"),
        (8888, "HTTP-8888"),
        (9000, "HTTP-9000"),
        (9042, "Cassandra"),
        (9090, "Prometheus"),
        (9200, "Elasticsearch"),
        (9300, "ES-Transport"),
        (11211, "Memcached"),
        (15672, "RabbitMQ-UI"),
        (2181, "ZooKeeper"),
        (2379, "etcd"),
        (26257, "CockroachDB"),
        (27017, "MongoDB"),
        (50070, "Hadoop-NameNode"),
    ];
    
    services
        .iter()
        .find(|(p, _)| *p == port)
        .map(|(_, s)| s.to_string())
}

/// 解析主机名获取IP地址
pub async fn resolve_ip(target: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let addrs = tokio::net::lookup_host((target, 0)).await?;
    for addr in addrs {
        return Ok(addr.ip().to_string());
    }
    Ok(target.to_string())
}

/// 判断是否为IPv6地址
pub fn is_ipv6(ip: &str) -> bool {
    ip.contains(':')
}

/// 格式化socket地址
pub fn format_socket_addr(ip: &str, port: u16) -> String {
    if is_ipv6(ip) {
        format!("[{}]:{}", ip, port)
    } else {
        format!("{}:{}", ip, port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_service() {
        assert_eq!(guess_service(80), Some("HTTP".to_string()));
        assert_eq!(guess_service(443), Some("HTTPS".to_string()));
        assert_eq!(guess_service(22), Some("SSH".to_string()));
        assert_eq!(guess_service(12345), None);
    }

    #[test]
    fn test_is_ipv6() {
        assert!(is_ipv6("::1"));
        assert!(is_ipv6("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
        assert!(!is_ipv6("192.168.1.1"));
        assert!(!is_ipv6("127.0.0.1"));
    }

    #[test]
    fn test_format_socket_addr() {
        assert_eq!(format_socket_addr("192.168.1.1", 80), "192.168.1.1:80");
        assert_eq!(format_socket_addr("::1", 80), "[::1]:80");
        assert_eq!(format_socket_addr("2001:db8::1", 443), "[2001:db8::1]:443");
    }
}