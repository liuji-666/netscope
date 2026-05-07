use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetScopeConfig {
    pub preferred_dns: Option<String>,
    pub dns_backup: Option<String>,
    pub preferred_mirrors: HashMap<String, String>,
    pub cached_mirror_results: HashMap<String, CachedMirrorResult>,
    pub cached_dns_results: Option<CachedDnsResult>,
    pub favorite_sites: Vec<String>,
    pub language: Option<String>,
    pub first_run: bool,
    pub last_update: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedMirrorResult {
    pub mirror_name: String,
    pub mirror_url: String,
    pub latency_ms: f64,
    pub score: u32,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedDnsResult {
    pub dns_server: String,
    pub latency_ms: f64,
    pub success_rate: f64,
    pub timestamp: String,
}

impl Default for NetScopeConfig {
    fn default() -> Self {
        NetScopeConfig {
            preferred_dns: None,
            dns_backup: None,
            preferred_mirrors: HashMap::new(),
            cached_mirror_results: HashMap::new(),
            cached_dns_results: None,
            favorite_sites: vec![],
            language: None,
            first_run: true,
            last_update: None,
        }
    }
}

impl NetScopeConfig {
    pub fn load() -> Self {
        if let Some(config_path) = Self::get_config_path() {
            if config_path.exists() {
                match fs::read_to_string(&config_path) {
                    Ok(content) => match serde_json::from_str(&content) {
                        Ok(config) => return config,
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        if let Some(config_path) = Self::get_config_path() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let content = serde_json::to_string_pretty(self)?;
            fs::write(&config_path, content)?;
        }
        Ok(())
    }

    pub fn get_config_path() -> Option<PathBuf> {
        match dirs::config_dir() {
            Some(mut path) => {
                path.push("netscope");
                path.push("config.json");
                Some(path)
            }
            None => None,
        }
    }

    pub fn set_preferred_dns(&mut self, primary: &str, backup: Option<&str>) {
        self.preferred_dns = Some(primary.to_string());
        self.dns_backup = backup.map(|s| s.to_string());
        self.last_update = Some(Self::get_timestamp());
    }

    pub fn set_preferred_mirror(&mut self, category: &str, mirror_name: &str, mirror_url: &str) {
        self.preferred_mirrors.insert(category.to_string(), mirror_name.to_string());
        let cached = CachedMirrorResult {
            mirror_name: mirror_name.to_string(),
            mirror_url: mirror_url.to_string(),
            latency_ms: 0.0,
            score: 0,
            timestamp: Self::get_timestamp(),
        };
        self.cached_mirror_results.insert(category.to_string(), cached);
        self.last_update = Some(Self::get_timestamp());
    }

    pub fn cache_mirror_result(&mut self, category: &str, result: CachedMirrorResult) {
        self.cached_mirror_results.insert(category.to_string(), result);
        self.last_update = Some(Self::get_timestamp());
    }

    pub fn cache_dns_result(&mut self, result: CachedDnsResult) {
        self.cached_dns_results = Some(result);
        self.last_update = Some(Self::get_timestamp());
    }

    pub fn add_favorite_site(&mut self, site: &str) {
        if !self.favorite_sites.contains(&site.to_string()) {
            self.favorite_sites.push(site.to_string());
            self.last_update = Some(Self::get_timestamp());
        }
    }

    pub fn remove_favorite_site(&mut self, site: &str) {
        self.favorite_sites.retain(|s| s != site);
        self.last_update = Some(Self::get_timestamp());
    }

    pub fn get_preferred_mirror(&self, category: &str) -> Option<&str> {
        self.preferred_mirrors.get(category).map(|s| s.as_str())
    }

    pub fn get_cached_mirror_result(&self, category: &str) -> Option<&CachedMirrorResult> {
        self.cached_mirror_results.get(category)
    }

    pub fn is_cache_valid(&self, category: &str) -> bool {
        if let Some(cached) = self.get_cached_mirror_result(category) {
            if let Ok(timestamp) = chrono::DateTime::parse_from_rfc3339(&cached.timestamp) {
                let now = chrono::Utc::now();
                let diff = now.signed_duration_since(timestamp);
                return diff.num_hours() < 24;
            }
        }
        false
    }

    pub fn set_language(&mut self, lang: &str) {
        self.language = Some(lang.to_string());
        self.first_run = false;
        self.last_update = Some(Self::get_timestamp());
    }

    pub fn is_first_run(&self) -> bool {
        self.first_run
    }

    pub fn get_language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    pub fn mark_setup_complete(&mut self) {
        self.first_run = false;
        if let Err(_) = self.save() {}
    }

    fn get_timestamp() -> String {
        chrono::Utc::now().to_rfc3339()
    }
}