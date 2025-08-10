use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettings {
    pub general: GeneralSettings,
    pub privacy: PrivacySettings,
    pub appearance: AppearanceSettings,
    pub search: SearchSettings,
    pub downloads: DownloadSettings,
    pub advanced: AdvancedSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub homepage: String,
    pub new_tab_page: String,
    pub default_search_engine: String,
    pub restore_tabs_on_startup: bool,
    pub show_bookmarks_bar: bool,
    pub enable_notifications: bool,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub block_ads: bool,
    pub block_trackers: bool,
    pub block_third_party_cookies: bool,
    pub enable_fingerprinting_protection: bool,
    pub https_only_mode: bool,
    pub clear_data_on_exit: bool,
    pub send_do_not_track: bool,
    pub enable_private_browsing_by_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme: String,
    pub font_family: String,
    pub font_size: u32,
    pub zoom_level: f64,
    pub show_tab_previews: bool,
    pub compact_mode: bool,
    pub custom_css: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSettings {
    pub search_engines: HashMap<String, SearchEngine>,
    pub default_engine: String,
    pub enable_search_suggestions: bool,
    pub show_search_in_address_bar: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngine {
    pub name: String,
    pub url: String,
    pub suggest_url: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSettings {
    pub download_directory: String,
    pub ask_where_to_save: bool,
    pub auto_open_downloads: bool,
    pub clear_downloads_on_exit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub enable_javascript: bool,
    pub enable_images: bool,
    pub enable_plugins: bool,
    pub enable_webgl: bool,
    pub enable_webrtc: bool,
    pub user_agent: Option<String>,
    pub proxy_settings: ProxySettings,
    pub developer_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxySettings {
    pub proxy_type: ProxyType,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxyType {
    None,
    Http,
    Https,
    Socks4,
    Socks5,
}

static SETTINGS_MANAGER: Lazy<RwLock<SettingsManager>> = Lazy::new(|| {
    RwLock::new(SettingsManager::new())
});

pub struct SettingsManager {
    pub settings: BrowserSettings,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        let mut search_engines = HashMap::new();
        
        search_engines.insert("google".to_string(), SearchEngine {
            name: "Google".to_string(),
            url: "https://www.google.com/search?q={searchTerms}".to_string(),
            suggest_url: Some("https://suggestqueries.google.com/complete/search?client=chrome&q={searchTerms}".to_string()),
            icon: None,
        });
        
        search_engines.insert("duckduckgo".to_string(), SearchEngine {
            name: "DuckDuckGo".to_string(),
            url: "https://duckduckgo.com/?q={searchTerms}".to_string(),
            suggest_url: Some("https://ac.duckduckgo.com/ac/?q={searchTerms}&type=list".to_string()),
            icon: None,
        });
        
        search_engines.insert("bing".to_string(), SearchEngine {
            name: "Bing".to_string(),
            url: "https://www.bing.com/search?q={searchTerms}".to_string(),
            suggest_url: Some("https://www.bing.com/osjson.aspx?query={searchTerms}".to_string()),
            icon: None,
        });
        
        Self {
            general: GeneralSettings {
                homepage: "about:blank".to_string(),
                new_tab_page: "about:newtab".to_string(),
                default_search_engine: "duckduckgo".to_string(),
                restore_tabs_on_startup: true,
                show_bookmarks_bar: true,
                enable_notifications: true,
                language: "en-US".to_string(),
            },
            privacy: PrivacySettings {
                block_ads: true,
                block_trackers: true,
                block_third_party_cookies: true,
                enable_fingerprinting_protection: true,
                https_only_mode: true,
                clear_data_on_exit: false,
                send_do_not_track: true,
                enable_private_browsing_by_default: false,
            },
            appearance: AppearanceSettings {
                theme: "system".to_string(),
                font_family: "system-ui".to_string(),
                font_size: 16,
                zoom_level: 1.0,
                show_tab_previews: true,
                compact_mode: false,
                custom_css: None,
            },
            search: SearchSettings {
                search_engines,
                default_engine: "duckduckgo".to_string(),
                enable_search_suggestions: true,
                show_search_in_address_bar: true,
            },
            downloads: DownloadSettings {
                download_directory: dirs::download_dir()
                    .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
                    .to_string_lossy()
                    .to_string(),
                ask_where_to_save: false,
                auto_open_downloads: false,
                clear_downloads_on_exit: false,
            },
            advanced: AdvancedSettings {
                enable_javascript: true,
                enable_images: true,
                enable_plugins: false,
                enable_webgl: true,
                enable_webrtc: false,
                user_agent: None,
                proxy_settings: ProxySettings {
                    proxy_type: ProxyType::None,
                    host: None,
                    port: None,
                    username: None,
                    password: None,
                },
                developer_mode: false,
            },
        }
    }
}

impl SettingsManager {
    pub fn new() -> Self {
        Self {
            settings: BrowserSettings::default(),
        }
    }

    pub fn get_settings(&self) -> &BrowserSettings {
        &self.settings
    }

    pub fn update_general_settings(&mut self, settings: GeneralSettings) {
        self.settings.general = settings;
    }

    pub fn update_privacy_settings(&mut self, settings: PrivacySettings) {
        self.settings.privacy = settings;
    }

    pub fn update_appearance_settings(&mut self, settings: AppearanceSettings) {
        self.settings.appearance = settings;
    }

    pub fn update_search_settings(&mut self, settings: SearchSettings) {
        self.settings.search = settings;
    }

    pub fn update_download_settings(&mut self, settings: DownloadSettings) {
        self.settings.downloads = settings;
    }

    pub fn update_advanced_settings(&mut self, settings: AdvancedSettings) {
        self.settings.advanced = settings;
    }

    pub fn add_search_engine(&mut self, id: &str, engine: SearchEngine) {
        self.settings.search.search_engines.insert(id.to_string(), engine);
    }

    pub fn remove_search_engine(&mut self, id: &str) -> Result<(), String> {
        if id == self.settings.search.default_engine {
            return Err("Cannot remove default search engine".to_string());
        }
        
        self.settings.search.search_engines.remove(id)
            .ok_or("Search engine not found")?;
        
        Ok(())
    }

    pub fn set_default_search_engine(&mut self, id: &str) -> Result<(), String> {
        if !self.settings.search.search_engines.contains_key(id) {
            return Err("Search engine not found".to_string());
        }
        
        self.settings.search.default_engine = id.to_string();
        Ok(())
    }

    pub fn reset_to_defaults(&mut self) {
        self.settings = BrowserSettings::default();
    }

    pub fn export_settings(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.settings)
            .map_err(|e| format!("Failed to export settings: {}", e))
    }

    pub fn import_settings(&mut self, data: &str) -> Result<(), String> {
        let imported_settings: BrowserSettings = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse settings data: {}", e))?;
        
        self.settings = imported_settings;
        Ok(())
    }

    pub fn get_search_url(&self, query: &str) -> Option<String> {
        let engine = self.settings.search.search_engines
            .get(&self.settings.search.default_engine)?;
        
        Some(engine.url.replace("{searchTerms}", &urlencoding::encode(query)))
    }

    pub fn get_suggestion_url(&self, query: &str) -> Option<String> {
        let engine = self.settings.search.search_engines
            .get(&self.settings.search.default_engine)?;
        
        engine.suggest_url.as_ref().map(|url| {
            url.replace("{searchTerms}", &urlencoding::encode(query))
        })
    }
}

#[tauri::command]
pub async fn get_settings() -> Result<BrowserSettings, String> {
    let manager = SETTINGS_MANAGER.read().await;
    Ok(manager.get_settings().clone())
}

#[tauri::command]
pub async fn update_general_settings(settings: GeneralSettings) -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.update_general_settings(settings);
    Ok(())
}

#[tauri::command]
pub async fn update_privacy_settings(settings: PrivacySettings) -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.update_privacy_settings(settings);
    Ok(())
}

#[tauri::command]
pub async fn update_appearance_settings(settings: AppearanceSettings) -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.update_appearance_settings(settings);
    Ok(())
}

#[tauri::command]
pub async fn update_search_settings(settings: SearchSettings) -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.update_search_settings(settings);
    Ok(())
}

#[tauri::command]
pub async fn update_download_settings(settings: DownloadSettings) -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.update_download_settings(settings);
    Ok(())
}

#[tauri::command]
pub async fn update_advanced_settings(settings: AdvancedSettings) -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.update_advanced_settings(settings);
    Ok(())
}

#[tauri::command]
pub async fn add_search_engine(id: String, engine: SearchEngine) -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.add_search_engine(&id, engine);
    Ok(())
}

#[tauri::command]
pub async fn remove_search_engine(id: String) -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.remove_search_engine(&id)
}

#[tauri::command]
pub async fn set_default_search_engine(id: String) -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.set_default_search_engine(&id)
}

#[tauri::command]
pub async fn reset_settings_to_defaults() -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.reset_to_defaults();
    Ok(())
}

#[tauri::command]
pub async fn export_settings() -> Result<String, String> {
    let manager = SETTINGS_MANAGER.read().await;
    manager.export_settings()
}

#[tauri::command]
pub async fn import_settings(data: String) -> Result<(), String> {
    let mut manager = SETTINGS_MANAGER.write().await;
    manager.import_settings(&data)
}

#[tauri::command]
pub async fn get_search_url(query: String) -> Result<Option<String>, String> {
    let manager = SETTINGS_MANAGER.read().await;
    Ok(manager.get_search_url(&query))
}

#[tauri::command]
pub async fn get_suggestion_url(query: String) -> Result<Option<String>, String> {
    let manager = SETTINGS_MANAGER.read().await;
    Ok(manager.get_suggestion_url(&query))
}