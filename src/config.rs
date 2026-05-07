use std::time::Duration;

pub struct Config {
    pub ping_count: u32,
    pub ping_timeout: Duration,
    pub trace_max_hops: u32,
    pub trace_timeout: Duration,
    pub port_concurrency: usize,
    pub port_timeout: Duration,
    pub http_timeout: Duration,
    pub top_ports: usize,
    pub dns_resolvers: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ping_count: 5,
            ping_timeout: Duration::from_secs(3),
            trace_max_hops: 30,
            trace_timeout: Duration::from_secs(5),
            port_concurrency: 100,
            port_timeout: Duration::from_secs(2),
            http_timeout: Duration::from_secs(10),
            top_ports: 20,
            dns_resolvers: vec![
                "8.8.8.8".to_string(),
                "1.1.1.1".to_string(),
                "223.5.5.5".to_string(),
                "119.29.29.29".to_string(),
            ],
        }
    }
}

pub fn common_ports(top: usize) -> Vec<u16> {
    let all = vec![
        80, 443, 22, 21, 25, 53, 110, 143, 993, 995, 3306, 5432, 6379, 8080,
        8443, 8888, 9090, 27017, 11211, 15672, 5672, 9200, 9300, 2379, 7001,
        8500, 2181, 50070, 8088, 4040, 19888, 8042, 135, 445, 3389, 5985,
        5986, 47001, 3000, 5000, 8000, 9000, 4200, 3001, 5173, 23, 25, 105,
        106, 109, 110, 143, 158, 220, 465, 587, 993, 995, 2049, 3000, 3306,
        4443, 5432, 5900, 6379, 6667, 8000, 8080, 8443, 8888, 9090, 9200,
        9300, 11211, 27017, 50000, 50070, 1433, 1434, 1521, 2483, 2484,
        3050, 3306, 5432, 5984, 6379, 8529, 9042, 15272, 26257, 26258,
        161, 162, 389, 636, 88, 464, 749, 3268, 3269, 5722, 9389,
    ];
    all.into_iter().take(top).collect()
}