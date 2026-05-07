use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteEntry {
    pub name: String,
    pub url: String,
    pub region: String,
    pub latency_ms: f64,
    pub success: bool,
    pub status_code: Option<u16>,
    pub score: u32,
    pub recommended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteTestResult {
    pub site: String,
    pub entries: Vec<SiteEntry>,
    pub best_entry: Option<String>,
    pub recommended_url: Option<String>,
    pub summary: String,
}

pub struct SiteTester {
    sites: Vec<SiteConfig>,
}

struct SiteConfig {
    name: String,
    entries: Vec<SiteEntryConfig>,
}

struct SiteEntryConfig {
    name: String,
    url: String,
    region: String,
}

impl SiteTester {
    pub fn new() -> Self {
        SiteTester {
            sites: vec![
                SiteConfig {
                    name: "Google".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Global".to_string(),
                            url: "https://www.google.com".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Hong Kong".to_string(),
                            url: "https://www.google.com.hk".to_string(),
                            region: "香港".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Taiwan".to_string(),
                            url: "https://www.google.com.tw".to_string(),
                            region: "台湾".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Japan".to_string(),
                            url: "https://www.google.co.jp".to_string(),
                            region: "日本".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Singapore".to_string(),
                            url: "https://www.google.com.sg".to_string(),
                            region: "新加坡".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "GitHub".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Original".to_string(),
                            url: "https://github.com".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "GHProxy".to_string(),
                            url: "https://ghproxy.com".to_string(),
                            region: "中国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "FastGit".to_string(),
                            url: "https://hub.fastgit.xyz".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "YouTube".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Global".to_string(),
                            url: "https://www.youtube.com".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Hong Kong".to_string(),
                            url: "https://www.youtube.com.hk".to_string(),
                            region: "香港".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Japan".to_string(),
                            url: "https://www.youtube.co.jp".to_string(),
                            region: "日本".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Singapore".to_string(),
                            url: "https://www.youtube.com.sg".to_string(),
                            region: "新加坡".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "ChatGPT".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Original".to_string(),
                            url: "https://chat.openai.com".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Singapore".to_string(),
                            url: "https://chat.openai.com".to_string(),
                            region: "新加坡".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Stack Overflow".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Original".to_string(),
                            url: "https://stackoverflow.com".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Cloudflare".to_string(),
                            url: "https://stackoverflow.com".to_string(),
                            region: "CDN".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Rust Docs".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Original".to_string(),
                            url: "https://doc.rust-lang.org".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "USTC Mirror".to_string(),
                            url: "https://mirrors.ustc.edu.cn/rust-static/doc".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Crates.io".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Original".to_string(),
                            url: "https://crates.io".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "USTC Mirror".to_string(),
                            url: "https://mirrors.ustc.edu.cn/crates.io-index".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Wikipedia".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Original".to_string(),
                            url: "https://www.wikipedia.org".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "China".to_string(),
                            url: "https://zh.wikipedia.org".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "NPM".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Official".to_string(),
                            url: "https://www.npmjs.com".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Taobao".to_string(),
                            url: "https://registry.npmmirror.com".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "PyPI".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Official".to_string(),
                            url: "https://pypi.org".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Tsinghua".to_string(),
                            url: "https://pypi.tuna.tsinghua.edu.cn".to_string(),
                            region: "中国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Aliyun".to_string(),
                            url: "https://mirrors.aliyun.com/pypi".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Docker Hub".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Official".to_string(),
                            url: "https://hub.docker.com".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "DaoCloud".to_string(),
                            url: "https://hub.daocloud.io".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Reddit".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Original".to_string(),
                            url: "https://www.reddit.com".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Nitter".to_string(),
                            url: "https://nitter.net".to_string(),
                            region: "CDN".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Twitter/X".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Original".to_string(),
                            url: "https://x.com".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Nitter".to_string(),
                            url: "https://nitter.privacydev.net".to_string(),
                            region: "CDN".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Cloudflare".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Global".to_string(),
                            url: "https://www.cloudflare.com".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "China".to_string(),
                            url: "https://www.cloudflare-cn.com".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "AWS".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Global".to_string(),
                            url: "https://aws.amazon.com".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "China".to_string(),
                            url: "https://amazonaws.cn".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "OpenAI".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Global".to_string(),
                            url: "https://openai.com".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "API".to_string(),
                            url: "https://api.openai.com".to_string(),
                            region: "美国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Anthropic".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Global".to_string(),
                            url: "https://www.anthropic.com".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "API".to_string(),
                            url: "https://api.anthropic.com".to_string(),
                            region: "美国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Hugging Face".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Global".to_string(),
                            url: "https://huggingface.co".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "China Mirror".to_string(),
                            url: "https://hf-mirror.com".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "GitLab".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Global".to_string(),
                            url: "https://gitlab.com".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "China".to_string(),
                            url: "https://gitlab.cn".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "npm Registry".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Official".to_string(),
                            url: "https://registry.npmjs.org".to_string(),
                            region: "美国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "China Mirror".to_string(),
                            url: "https://registry.npmmirror.com".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Rust Crates".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Official".to_string(),
                            url: "https://crates.io".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "USTC".to_string(),
                            url: "https://mirrors.ustc.edu.cn/crates.io-index".to_string(),
                            region: "中国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "TUNA".to_string(),
                            url: "https://mirrors.tuna.tsinghua.edu.cn/crates.io-index".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Go Modules".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Official".to_string(),
                            url: "https://proxy.golang.org".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "GOPROXY.CN".to_string(),
                            url: "https://goproxy.cn".to_string(),
                            region: "中国".to_string(),
                        },
                        SiteEntryConfig {
                            name: "GOPROXY.IO".to_string(),
                            url: "https://goproxy.io".to_string(),
                            region: "全球".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Maven".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Official".to_string(),
                            url: "https://repo.maven.apache.org".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "Aliyun".to_string(),
                            url: "https://maven.aliyun.com/repository/public".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "Gradle".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Official".to_string(),
                            url: "https://plugins.gradle.org".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "China Mirror".to_string(),
                            url: "https://mirrors.aliyun.com/gradle".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "JetBrains".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Global".to_string(),
                            url: "https://www.jetbrains.com".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "China".to_string(),
                            url: "https://www.jetbrains.com.cn".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
                SiteConfig {
                    name: "VS Code".to_string(),
                    entries: vec![
                        SiteEntryConfig {
                            name: "Marketplace".to_string(),
                            url: "https://marketplace.visualstudio.com".to_string(),
                            region: "全球".to_string(),
                        },
                        SiteEntryConfig {
                            name: "China".to_string(),
                            url: "https://marketplace.visualstudio.com.vsixmirror.com".to_string(),
                            region: "中国".to_string(),
                        },
                    ],
                },
            ],
        }
    }

    pub async fn test_all(&self) -> Vec<SiteTestResult> {
        let mut results: Vec<SiteTestResult> = Vec::new();

        for site in &self.sites {
            let result = self.test_site(site).await;
            results.push(result);
        }

        results
    }

    pub async fn test_site_by_name(&self, site_name: &str) -> Option<SiteTestResult> {
        let site = self.sites.iter().find(|s| s.name.to_lowercase() == site_name.to_lowercase())?;
        Some(self.test_site(site).await)
    }

    pub async fn test_any_url(&self, url: &str) -> SiteTestResult {
        let domain = extract_domain(url);
        
        let mut test_entries = Vec::new();
        
        if url.starts_with("http://") || url.starts_with("https://") {
            test_entries.push(SiteEntryConfig {
                name: "Original".to_string(),
                url: url.to_string(),
                region: "Original".to_string(),
            });
            
            let base_url = if url.contains("://") {
                let parts: Vec<&str> = url.split("://").collect();
                if parts.len() >= 2 {
                    let domain_part = parts[1].split('/').next().unwrap_or("");
                    format!("{}://{}", parts[0], domain_part)
                } else {
                    url.to_string()
                }
            } else {
                url.to_string()
            };
            
            let https_url = if base_url.starts_with("http://") {
                base_url.replacen("http://", "https://", 1)
            } else {
                base_url.clone()
            };
            
            let http_url = if base_url.starts_with("https://") {
                base_url.replacen("https://", "http://", 1)
            } else {
                base_url.clone()
            };
            
            if https_url != url {
                test_entries.push(SiteEntryConfig {
                    name: "HTTPS".to_string(),
                    url: https_url,
                    region: "HTTPS".to_string(),
                });
            }
            
            if http_url != url && http_url != test_entries.last().map(|e| e.url.as_str()).unwrap_or("") {
                test_entries.push(SiteEntryConfig {
                    name: "HTTP".to_string(),
                    url: http_url,
                    region: "HTTP".to_string(),
                });
            }
            
            if !url.ends_with('/') && url.matches('/').count() > 2 {
                test_entries.push(SiteEntryConfig {
                    name: "Root".to_string(),
                    url: base_url,
                    region: "Root".to_string(),
                });
            }
        } else {
            test_entries.push(SiteEntryConfig {
                name: "HTTPS".to_string(),
                url: format!("https://{}", url),
                region: "HTTPS".to_string(),
            });
            test_entries.push(SiteEntryConfig {
                name: "HTTP".to_string(),
                url: format!("http://{}", url),
                region: "HTTP".to_string(),
            });
        }
        
        let mut entries: Vec<SiteEntry> = Vec::new();
        for entry_config in test_entries {
            let entry = self.test_single_url(&entry_config.url, entry_config.region.clone()).await;
            entries.push(entry);
        }
        
        entries.sort_by(|a, b| b.score.cmp(&a.score));
        
        let mut final_entries: Vec<SiteEntry> = Vec::new();
        for (i, mut entry) in entries.into_iter().enumerate() {
            entry.recommended = i == 0;
            final_entries.push(entry);
        }
        
        let best_entry = final_entries.first().map(|e| e.name.clone());
        let recommended_url = final_entries.first().and_then(|e| if e.success { Some(e.url.clone()) } else { None });
        
        let successful_count = final_entries.iter().filter(|e| e.success).count();
        let total_count = final_entries.len();
        
        let summary = if successful_count == 0 {
            format!("所有入口都无法访问")
        } else if successful_count == total_count {
            format!("所有 {} 个入口都可访问", total_count)
        } else {
            format!("{} / {} 个入口可访问", successful_count, total_count)
        };
        
        SiteTestResult {
            site: domain,
            entries: final_entries,
            best_entry,
            recommended_url,
            summary,
        }
    }
    
    pub async fn test_custom_url(&self, url: &str) -> SiteTestResult {
        let entry = self.test_single_url(url, "Custom".to_string()).await;
        let latency_ms = entry.latency_ms;
        let success = entry.success;
        
        SiteTestResult {
            site: extract_domain(url),
            entries: vec![entry],
            best_entry: Some("Custom".to_string()),
            recommended_url: Some(url.to_string()),
            summary: if success {
                format!("URL 可访问，延迟 {:.0}ms", latency_ms)
            } else {
                "URL 不可访问".to_string()
            },
        }
    }

    async fn test_site(&self, site: &SiteConfig) -> SiteTestResult {
        let mut entries: Vec<SiteEntry> = Vec::new();

        for entry_config in &site.entries {
            let entry = self.test_single_url(&entry_config.url, entry_config.region.clone()).await;
            entries.push(entry);
        }

        entries.sort_by(|a, b| b.score.cmp(&a.score));

        let mut final_entries: Vec<SiteEntry> = Vec::new();
        for (i, mut entry) in entries.into_iter().enumerate() {
            entry.recommended = i == 0;
            final_entries.push(entry);
        }

        let best_entry = final_entries.first().map(|e| e.name.clone());
        let recommended_url = final_entries.first().and_then(|e| if e.success { Some(e.url.clone()) } else { None });

        let successful_count = final_entries.iter().filter(|e| e.success).count();
        let total_count = final_entries.len();

        let summary = if successful_count == 0 {
            format!("所有入口都无法访问")
        } else if successful_count == total_count {
            format!("所有 {} 个入口都可访问", total_count)
        } else {
            format!("{} / {} 个入口可访问", successful_count, total_count)
        };

        SiteTestResult {
            site: site.name.clone(),
            entries: final_entries,
            best_entry,
            recommended_url,
            summary,
        }
    }

    async fn test_single_url(&self, url: &str, region: String) -> SiteEntry {
        let start = Instant::now();
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        let response = client.get(url).send().await;

        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

        match response {
            Ok(resp) => {
                let status_code = resp.status().as_u16();
                let success = resp.status().is_success();
                let score = self.calculate_score(elapsed_ms, success);

                SiteEntry {
                    name: extract_domain(url),
                    url: url.to_string(),
                    region,
                    latency_ms: elapsed_ms,
                    success,
                    status_code: Some(status_code),
                    score,
                    recommended: false,
                }
            }
            Err(_) => {
                SiteEntry {
                    name: extract_domain(url),
                    url: url.to_string(),
                    region,
                    latency_ms: 9999.0,
                    success: false,
                    status_code: None,
                    score: 0,
                    recommended: false,
                }
            }
        }
    }

    fn calculate_score(&self, latency_ms: f64, success: bool) -> u32 {
        if !success {
            return 0;
        }

        let latency_score = if latency_ms < 50.0 {
            70
        } else if latency_ms < 100.0 {
            60
        } else if latency_ms < 200.0 {
            50
        } else if latency_ms < 500.0 {
            35
        } else if latency_ms < 1000.0 {
            20
        } else {
            10
        };

        let availability_score = 30;

        latency_score + availability_score
    }
}

impl Default for SiteTester {
    fn default() -> Self {
        Self::new()
    }
}

fn extract_domain(url: &str) -> String {
    url.replace("https://", "")
       .replace("http://", "")
       .split('/')
       .next()
       .unwrap_or(url)
       .to_string()
}