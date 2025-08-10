use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use url::Url;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterList {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub rules: Vec<FilterRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRule {
    pub pattern: String,
    pub rule_type: FilterRuleType,
    pub domains: Option<Vec<String>>,
    pub exceptions: Option<Vec<String>>,
    pub options: FilterOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterRuleType {
    Block,
    Allow,
    Hide,
    Redirect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterOptions {
    pub script: bool,
    pub image: bool,
    pub stylesheet: bool,
    pub xmlhttprequest: bool,
    pub subdocument: bool,
    pub third_party: bool,
    pub popup: bool,
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self {
            script: true,
            image: true,
            stylesheet: true,
            xmlhttprequest: true,
            subdocument: true,
            third_party: true,
            popup: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteShields {
    pub domain: String,
    pub ad_blocking: bool,
    pub tracker_blocking: bool,
    pub third_party_cookies: bool,
    pub fingerprinting_protection: bool,
    pub https_only: bool,
    pub scripts_blocked: u32,
    pub trackers_blocked: u32,
    pub ads_blocked: u32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for SiteShields {
    fn default() -> Self {
        Self {
            domain: String::new(),
            ad_blocking: true,
            tracker_blocking: true,
            third_party_cookies: false,
            fingerprinting_protection: true,
            https_only: true,
            scripts_blocked: 0,
            trackers_blocked: 0,
            ads_blocked: 0,
            last_updated: chrono::Utc::now(),
        }
    }
}

pub struct FilterEngine {
    pub filter_lists: HashMap<String, FilterList>,
    pub site_shields: HashMap<String, SiteShields>,
    pub compiled_rules: HashMap<String, Regex>,
    pub global_stats: GlobalStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalStats {
    pub total_ads_blocked: u64,
    pub total_trackers_blocked: u64,
    pub total_scripts_blocked: u64,
    pub bandwidth_saved: u64,
    pub last_reset: chrono::DateTime<chrono::Utc>,
}

static FILTER_ENGINE: Lazy<RwLock<FilterEngine>> = Lazy::new(|| {
    RwLock::new(FilterEngine::new())
});

impl FilterEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            filter_lists: HashMap::new(),
            site_shields: HashMap::new(),
            compiled_rules: HashMap::new(),
            global_stats: GlobalStats::default(),
        };
        
        engine.load_default_filter_lists();
        engine
    }

    fn load_default_filter_lists(&mut self) {
        let easylist = FilterList {
            name: "EasyList".to_string(),
            url: "https://easylist.to/easylist/easylist.txt".to_string(),
            enabled: true,
            last_updated: chrono::Utc::now(),
            rules: Vec::new(),
        };
        
        let easyprivacy = FilterList {
            name: "EasyPrivacy".to_string(),
            url: "https://easylist.to/easylist/easyprivacy.txt".to_string(),
            enabled: true,
            last_updated: chrono::Utc::now(),
            rules: Vec::new(),
        };
        
        self.filter_lists.insert("easylist".to_string(), easylist);
        self.filter_lists.insert("easyprivacy".to_string(), easyprivacy);
    }

    pub fn should_block_request(&self, url: &str, request_type: &str, origin_domain: &str) -> bool {
        if let Ok(parsed_url) = Url::parse(url) {
            let domain = parsed_url.domain().unwrap_or("");
            
            if let Some(shields) = self.site_shields.get(origin_domain) {
                if !shields.ad_blocking && !shields.tracker_blocking {
                    return false;
                }
                
                if shields.third_party_cookies && domain != origin_domain {
                    return true;
                }
            }
            
            for filter_list in self.filter_lists.values() {
                if !filter_list.enabled {
                    continue;
                }
                
                for rule in &filter_list.rules {
                    if self.matches_rule(url, rule, request_type, origin_domain) {
                        match rule.rule_type {
                            FilterRuleType::Block => return true,
                            FilterRuleType::Allow => return false,
                            _ => continue,
                        }
                    }
                }
            }
        }
        
        false
    }

    fn matches_rule(&self, url: &str, rule: &FilterRule, request_type: &str, origin_domain: &str) -> bool {
        if let Some(regex) = self.compiled_rules.get(&rule.pattern) {
            if !regex.is_match(url) {
                return false;
            }
        } else {
            if !url.contains(&rule.pattern) {
                return false;
            }
        }
        
        if let Some(domains) = &rule.domains {
            if !domains.contains(&origin_domain.to_string()) {
                return false;
            }
        }
        
        if let Some(exceptions) = &rule.exceptions {
            if exceptions.contains(&origin_domain.to_string()) {
                return false;
            }
        }
        
        match request_type {
            "script" => rule.options.script,
            "image" => rule.options.image,
            "stylesheet" => rule.options.stylesheet,
            "xmlhttprequest" => rule.options.xmlhttprequest,
            "subdocument" => rule.options.subdocument,
            _ => true,
        }
    }

    pub fn update_site_shields(&mut self, domain: &str, shields: SiteShields) {
        self.site_shields.insert(domain.to_string(), shields);
    }

    pub fn get_site_shields(&self, domain: &str) -> SiteShields {
        self.site_shields.get(domain)
            .cloned()
            .unwrap_or_else(|| {
                let mut shields = SiteShields::default();
                shields.domain = domain.to_string();
                shields
            })
    }

    pub fn increment_blocked_count(&mut self, domain: &str, block_type: &str) {
        if let Some(shields) = self.site_shields.get_mut(domain) {
            match block_type {
                "ad" => {
                    shields.ads_blocked += 1;
                    self.global_stats.total_ads_blocked += 1;
                }
                "tracker" => {
                    shields.trackers_blocked += 1;
                    self.global_stats.total_trackers_blocked += 1;
                }
                "script" => {
                    shields.scripts_blocked += 1;
                    self.global_stats.total_scripts_blocked += 1;
                }
                _ => {}
            }
            shields.last_updated = chrono::Utc::now();
        }
    }

    pub async fn update_filter_lists(&mut self) -> Result<(), String> {
        for (_, filter_list) in self.filter_lists.iter_mut() {
            if let Ok(response) = reqwest::get(&filter_list.url).await {
                if let Ok(content) = response.text().await {
                    let parsed_rules = Self::parse_filter_rules(&content);
                    filter_list.rules = parsed_rules;
                    filter_list.last_updated = chrono::Utc::now();
                }
            }
        }
        Ok(())
    }

    fn parse_filter_rules(content: &str) -> Vec<FilterRule> {
        let mut rules = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('!') || line.starts_with('[') {
                continue;
            }
            
            let rule_type = if line.starts_with("@@") {
                FilterRuleType::Allow
            } else if line.contains("##") {
                FilterRuleType::Hide
            } else {
                FilterRuleType::Block
            };
            
            let pattern = line.replace("@@", "").split("$").next().unwrap_or(line).to_string();
            
            rules.push(FilterRule {
                pattern,
                rule_type,
                domains: None,
                exceptions: None,
                options: FilterOptions::default(),
            });
        }
        
        rules
    }
}

#[tauri::command]
pub async fn get_site_shields(domain: String) -> Result<SiteShields, String> {
    let engine = FILTER_ENGINE.read().await;
    Ok(engine.get_site_shields(&domain))
}

#[tauri::command]
pub async fn update_site_shields(domain: String, shields: SiteShields) -> Result<(), String> {
    let mut engine = FILTER_ENGINE.write().await;
    engine.update_site_shields(&domain, shields);
    Ok(())
}

#[tauri::command]
pub async fn get_global_stats() -> Result<GlobalStats, String> {
    let engine = FILTER_ENGINE.read().await;
    Ok(engine.global_stats.clone())
}

#[tauri::command]
pub async fn should_block_request(url: String, request_type: String, origin_domain: String) -> Result<bool, String> {
    let engine = FILTER_ENGINE.read().await;
    Ok(engine.should_block_request(&url, &request_type, &origin_domain))
}

#[tauri::command]
pub async fn update_filter_lists() -> Result<(), String> {
    let mut engine = FILTER_ENGINE.write().await;
    engine.update_filter_lists().await
}