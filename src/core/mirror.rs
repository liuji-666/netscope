use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorResult {
    pub name: String,
    pub url: String,
    pub category: String,
    pub avg_ms: f64,
    pub speed_mbps: Option<f64>,
    pub status: String,
    pub score: u32,
    pub recommended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorTestResult {
    pub category: String,
    pub mirrors: Vec<MirrorResult>,
    pub best_mirror: Option<String>,
    pub total_tested: u32,
    pub successful: u32,
}

pub struct MirrorTester {
    categories: Vec<MirrorCategory>,
}

struct MirrorCategory {
    name: String,
    mirrors: Vec<MirrorConfig>,
}

struct MirrorConfig {
    name: String,
    url: String,
    test_url: Option<String>,
}

impl MirrorTester {
    pub fn new() -> Self {
        MirrorTester {
            categories: vec![
                MirrorCategory {
                    name: "GitHub".to_string(),
                    mirrors: vec![
                        MirrorConfig {
                            name: "GitHub Original".to_string(),
                            url: "https://github.com".to_string(),
                            test_url: Some("https://raw.githubusercontent.com".to_string()),
                        },
                        MirrorConfig {
                            name: "GHProxy".to_string(),
                            url: "https://ghproxy.com".to_string(),
                            test_url: Some("https://ghproxy.com/https://raw.githubusercontent.com".to_string()),
                        },
                        MirrorConfig {
                            name: "FastGit".to_string(),
                            url: "https://hub.fastgit.xyz".to_string(),
                            test_url: Some("https://download.fastgit.org".to_string()),
                        },
                        MirrorConfig {
                            name: "GitHub Proxy".to_string(),
                            url: "https://github.moeyy.xyz".to_string(),
                            test_url: Some("https://github.moeyy.xyz/https://raw.githubusercontent.com".to_string()),
                        },
                    ],
                },
                MirrorCategory {
                    name: "Docker Hub".to_string(),
                    mirrors: vec![
                        MirrorConfig {
                            name: "Docker Hub Original".to_string(),
                            url: "https://hub.docker.com".to_string(),
                            test_url: Some("https://registry.hub.docker.com".to_string()),
                        },
                        MirrorConfig {
                            name: "DaoCloud".to_string(),
                            url: "https://hub.daocloud.io".to_string(),
                            test_url: None,
                        },
                        MirrorConfig {
                            name: "Tencent Container".to_string(),
                            url: "https://mirrors.cloud.tencent.com/docker".to_string(),
                            test_url: None,
                        },
                        MirrorConfig {
                            name: "AliYun Container".to_string(),
                            url: "https://mirrors.aliyun.com/docker".to_string(),
                            test_url: None,
                        },
                    ],
                },
                MirrorCategory {
                    name: "PyPI".to_string(),
                    mirrors: vec![
                        MirrorConfig {
                            name: "PyPI Original".to_string(),
                            url: "https://pypi.org".to_string(),
                            test_url: Some("https://files.pythonhosted.org".to_string()),
                        },
                        MirrorConfig {
                            name: "Tsinghua".to_string(),
                            url: "https://pypi.tuna.tsinghua.edu.cn".to_string(),
                            test_url: Some("https://pypi.tuna.tsinghua.edu.cn/simple".to_string()),
                        },
                        MirrorConfig {
                            name: "AliYun".to_string(),
                            url: "https://mirrors.aliyun.com/pypi".to_string(),
                            test_url: Some("https://mirrors.aliyun.com/pypi/simple".to_string()),
                        },
                        MirrorConfig {
                            name: "Tencent".to_string(),
                            url: "https://mirrors.cloud.tencent.com/pypi".to_string(),
                            test_url: Some("https://mirrors.cloud.tencent.com/pypi/simple".to_string()),
                        },
                        MirrorConfig {
                            name: "Huawei".to_string(),
                            url: "https://repo.huaweicloud.com/repository/pypi".to_string(),
                            test_url: Some("https://repo.huaweicloud.com/repository/pypi/simple".to_string()),
                        },
                    ],
                },
                MirrorCategory {
                    name: "npm".to_string(),
                    mirrors: vec![
                        MirrorConfig {
                            name: "npm Official".to_string(),
                            url: "https://www.npmjs.com".to_string(),
                            test_url: Some("https://registry.npmjs.org".to_string()),
                        },
                        MirrorConfig {
                            name: "Taobao".to_string(),
                            url: "https://npmmirror.com".to_string(),
                            test_url: Some("https://registry.npmmirror.com".to_string()),
                        },
                        MirrorConfig {
                            name: "Tencent".to_string(),
                            url: "https://mirrors.cloud.tencent.com/npm".to_string(),
                            test_url: Some("https://mirrors.cloud.tencent.com/npm".to_string()),
                        },
                        MirrorConfig {
                            name: "AliYun".to_string(),
                            url: "https://mirrors.aliyun.com/npm".to_string(),
                            test_url: Some("https://mirrors.aliyun.com/npm".to_string()),
                        },
                    ],
                },
                MirrorCategory {
                    name: "Rust Crates".to_string(),
                    mirrors: vec![
                        MirrorConfig {
                            name: "crates.io".to_string(),
                            url: "https://crates.io".to_string(),
                            test_url: Some("https://static.crates.io".to_string()),
                        },
                        MirrorConfig {
                            name: "USTC".to_string(),
                            url: "https://mirrors.ustc.edu.cn/crates.io-index".to_string(),
                            test_url: Some("https://mirrors.ustc.edu.cn/crates.io-index".to_string()),
                        },
                        MirrorConfig {
                            name: "TUNA".to_string(),
                            url: "https://mirrors.tuna.tsinghua.edu.cn/crates.io-index".to_string(),
                            test_url: Some("https://mirrors.tuna.tsinghua.edu.cn/crates.io".to_string()),
                        },
                        MirrorConfig {
                            name: "SJTUG".to_string(),
                            url: "https://mirrors.sjtug.sjtu.edu.cn/crates.io-index".to_string(),
                            test_url: Some("https://mirrors.sjtug.sjtu.edu.cn".to_string()),
                        },
                    ],
                },
                MirrorCategory {
                    name: "Go Module".to_string(),
                    mirrors: vec![
                        MirrorConfig {
                            name: "Go Proxy".to_string(),
                            url: "https://proxy.golang.org".to_string(),
                            test_url: Some("https://proxy.golang.org/github.com".to_string()),
                        },
                        MirrorConfig {
                            name: "GOPROXY.CN".to_string(),
                            url: "https://goproxy.cn".to_string(),
                            test_url: Some("https://goproxy.cn/github.com".to_string()),
                        },
                        MirrorConfig {
                            name: "GOPROXY.IO".to_string(),
                            url: "https://goproxy.io".to_string(),
                            test_url: Some("https://goproxy.io/github.com".to_string()),
                        },
                        MirrorConfig {
                            name: "Aliyun".to_string(),
                            url: "https://mirrors.aliyun.com/goproxy".to_string(),
                            test_url: Some("https://mirrors.aliyun.com/goproxy/github.com".to_string()),
                        },
                    ],
                },
            ],
        }
    }

    pub async fn test_all(&self) -> Vec<MirrorTestResult> {
        let mut results: Vec<MirrorTestResult> = Vec::new();

        for category in &self.categories {
            let test_result = self.test_category(category).await;
            results.push(test_result);
        }

        results
    }

    pub async fn test_category_by_name(&self, category_name: &str) -> Option<MirrorTestResult> {
        let category = self.categories.iter().find(|c| c.name.to_lowercase() == category_name.to_lowercase())?;
        Some(self.test_category(category).await)
    }

    async fn test_category(&self, category: &MirrorCategory) -> MirrorTestResult {
        let mut mirrors: Vec<MirrorResult> = Vec::new();

        for mirror_config in &category.mirrors {
            let result = self.test_mirror(
                &mirror_config.name,
                &mirror_config.url,
                &category.name,
                mirror_config.test_url.as_deref(),
            ).await;
            mirrors.push(result);
        }

        mirrors.sort_by(|a, b| b.score.cmp(&a.score));

        let mut final_mirrors: Vec<MirrorResult> = Vec::new();
        for (i, mut mirror) in mirrors.into_iter().enumerate() {
            mirror.recommended = i == 0;
            final_mirrors.push(mirror);
        }

        let total_tested = final_mirrors.len() as u32;
        let successful = final_mirrors.iter().filter(|m| m.status == "OK").count() as u32;
        let best_mirror = final_mirrors.first().map(|m| m.name.clone());

        MirrorTestResult {
            category: category.name.clone(),
            mirrors: final_mirrors,
            best_mirror,
            total_tested,
            successful,
        }
    }

    async fn test_mirror(&self, name: &str, url: &str, category: &str, test_url: Option<&str>) -> MirrorResult {
        let (avg_ms, status) = if let Some(test) = test_url {
            self.measure_latency(test).await
        } else {
            self.measure_latency(url).await
        };

        let speed_mbps = if status == "OK" {
            self.measure_speed(url).await
        } else {
            None
        };

        let score = self.calculate_score(avg_ms, speed_mbps, &status);

        MirrorResult {
            name: name.to_string(),
            url: url.to_string(),
            category: category.to_string(),
            avg_ms,
            speed_mbps,
            status,
            score,
            recommended: false,
        }
    }

    async fn measure_latency(&self, url: &str) -> (f64, String) {
        let start = Instant::now();

        match tokio::time::timeout(
            Duration::from_secs(5),
            reqwest::get(url),
        )
        .await
        {
            Ok(Ok(response)) if response.status().is_success() => {
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                (elapsed, "OK".to_string())
            }
            Ok(Ok(response)) => {
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                (elapsed, format!("HTTP {}", response.status().as_u16()))
            }
            Ok(Err(e)) => {
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                (elapsed, format!("Error: {}", e))
            }
            Err(_) => (9999.0, "Timeout".to_string()),
        }
    }

    async fn measure_speed(&self, url: &str) -> Option<f64> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .ok()?;

        let test_url = format!("{}/favicon.ico", url);

        match client.get(&test_url).send().await {
            Ok(response) if response.status().is_success() => {
                let size_bytes = response.content_length().unwrap_or(0) as f64;
                if size_bytes > 0.0 {
                    Some(size_bytes * 8.0 / 10.0 / 1_000_000.0)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn calculate_score(&self, avg_ms: f64, speed_mbps: Option<f64>, status: &str) -> u32 {
        if status != "OK" {
            return 0;
        }

        let latency_score = if avg_ms < 50.0 {
            60
        } else if avg_ms < 100.0 {
            50
        } else if avg_ms < 200.0 {
            40
        } else if avg_ms < 500.0 {
            30
        } else if avg_ms < 1000.0 {
            20
        } else {
            10
        };

        let speed_score = if let Some(speed) = speed_mbps {
            if speed > 10.0 {
                40
            } else if speed > 5.0 {
                30
            } else if speed > 1.0 {
                20
            } else {
                10
            }
        } else {
            20
        };

        latency_score + speed_score
    }
}

impl Default for MirrorTester {
    fn default() -> Self {
        Self::new()
    }
}