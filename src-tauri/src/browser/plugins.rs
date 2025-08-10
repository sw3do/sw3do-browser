use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub enabled: bool,
    pub permissions: Vec<PluginPermission>,
    pub hooks: Vec<PluginHook>,
    pub settings: HashMap<String, PluginSetting>,
    pub manifest_path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginPermission {
    NetworkAccess,
    FileSystemRead,
    FileSystemWrite,
    BookmarkAccess,
    HistoryAccess,
    TabManagement,
    WindowManagement,
    SettingsAccess,
    NotificationAccess,
    ClipboardAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PluginHook {
    BeforeNavigate,
    AfterNavigate,
    BeforeRequest,
    AfterRequest,
    TabCreated,
    TabClosed,
    WindowCreated,
    WindowClosed,
    BookmarkAdded,
    BookmarkRemoved,
    HistoryAdded,
    DownloadStarted,
    DownloadCompleted,
    SettingsChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSetting {
    pub key: String,
    pub value: serde_json::Value,
    pub setting_type: PluginSettingType,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginSettingType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub main: String,
    pub permissions: Vec<PluginPermission>,
    pub hooks: Vec<PluginHook>,
    pub settings: Vec<PluginSettingDefinition>,
    pub min_browser_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSettingDefinition {
    pub key: String,
    pub setting_type: PluginSettingType,
    pub description: String,
    pub default_value: serde_json::Value,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEvent {
    pub plugin_id: String,
    pub hook: PluginHook,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStats {
    pub total_plugins: usize,
    pub enabled_plugins: usize,
    pub disabled_plugins: usize,
    pub events_processed: u64,
    pub last_event_time: Option<chrono::DateTime<chrono::Utc>>,
}

static PLUGIN_MANAGER: Lazy<RwLock<PluginManager>> = Lazy::new(|| {
    RwLock::new(PluginManager::new())
});

pub struct PluginManager {
    pub plugins: HashMap<String, Plugin>,
    pub event_handlers: HashMap<PluginHook, Vec<String>>,
    pub stats: PluginStats,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            event_handlers: HashMap::new(),
            stats: PluginStats {
                total_plugins: 0,
                enabled_plugins: 0,
                disabled_plugins: 0,
                events_processed: 0,
                last_event_time: None,
            },
        }
    }

    pub fn install_plugin(&mut self, manifest_path: &str) -> Result<String, String> {
        let manifest_content = std::fs::read_to_string(manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;
        
        let manifest: PluginManifest = serde_json::from_str(&manifest_content)
            .map_err(|e| format!("Invalid manifest format: {}", e))?;
        
        let plugin_id = Uuid::new_v4().to_string();
        let mut settings = HashMap::new();
        
        for setting_def in &manifest.settings {
            settings.insert(setting_def.key.clone(), PluginSetting {
                key: setting_def.key.clone(),
                value: setting_def.default_value.clone(),
                setting_type: setting_def.setting_type.clone(),
                description: setting_def.description.clone(),
                required: setting_def.required,
            });
        }
        
        let plugin = Plugin {
            id: plugin_id.clone(),
            name: manifest.name,
            version: manifest.version,
            description: manifest.description,
            author: manifest.author,
            enabled: true,
            permissions: manifest.permissions,
            hooks: manifest.hooks.clone(),
            settings,
            manifest_path: manifest_path.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        for hook in &manifest.hooks {
            self.event_handlers.entry(hook.clone())
                .or_insert_with(Vec::new)
                .push(plugin_id.clone());
        }
        
        self.plugins.insert(plugin_id.clone(), plugin);
        self.update_stats();
        
        Ok(plugin_id)
    }

    pub fn uninstall_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let plugin = self.plugins.remove(plugin_id)
            .ok_or("Plugin not found")?;
        
        for hook in &plugin.hooks {
            if let Some(handlers) = self.event_handlers.get_mut(hook) {
                handlers.retain(|id| id != plugin_id);
                if handlers.is_empty() {
                    self.event_handlers.remove(hook);
                }
            }
        }
        
        self.update_stats();
        Ok(())
    }

    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let plugin = self.plugins.get_mut(plugin_id)
            .ok_or("Plugin not found")?;
        
        plugin.enabled = true;
        plugin.updated_at = chrono::Utc::now();
        
        for hook in &plugin.hooks {
            self.event_handlers.entry(hook.clone())
                .or_insert_with(Vec::new)
                .push(plugin_id.to_string());
        }
        
        self.update_stats();
        Ok(())
    }

    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let plugin = self.plugins.get_mut(plugin_id)
            .ok_or("Plugin not found")?;
        
        plugin.enabled = false;
        plugin.updated_at = chrono::Utc::now();
        
        for hook in &plugin.hooks {
            if let Some(handlers) = self.event_handlers.get_mut(hook) {
                handlers.retain(|id| id != plugin_id);
                if handlers.is_empty() {
                    self.event_handlers.remove(hook);
                }
            }
        }
        
        self.update_stats();
        Ok(())
    }

    pub fn get_plugin(&self, plugin_id: &str) -> Option<&Plugin> {
        self.plugins.get(plugin_id)
    }

    pub fn get_all_plugins(&self) -> Vec<&Plugin> {
        self.plugins.values().collect()
    }

    pub fn get_enabled_plugins(&self) -> Vec<&Plugin> {
        self.plugins.values().filter(|p| p.enabled).collect()
    }

    pub fn update_plugin_setting(&mut self, plugin_id: &str, key: &str, value: serde_json::Value) -> Result<(), String> {
        let plugin = self.plugins.get_mut(plugin_id)
            .ok_or("Plugin not found")?;
        
        if let Some(setting) = plugin.settings.get_mut(key) {
            setting.value = value;
            plugin.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err("Setting not found".to_string())
        }
    }

    pub fn get_plugin_setting(&self, plugin_id: &str, key: &str) -> Option<&PluginSetting> {
        self.plugins.get(plugin_id)?
            .settings.get(key)
    }

    pub fn trigger_event(&mut self, hook: PluginHook, data: serde_json::Value) -> Vec<String> {
        let mut triggered_plugins = Vec::new();
        
        if let Some(handlers) = self.event_handlers.get(&hook) {
            for plugin_id in handlers {
                if let Some(plugin) = self.plugins.get(plugin_id) {
                    if plugin.enabled {
                        triggered_plugins.push(plugin_id.clone());
                    }
                }
            }
        }
        
        self.stats.events_processed += 1;
        self.stats.last_event_time = Some(chrono::Utc::now());
        
        triggered_plugins
    }

    pub fn has_permission(&self, plugin_id: &str, permission: &PluginPermission) -> bool {
        if let Some(plugin) = self.plugins.get(plugin_id) {
            plugin.enabled && plugin.permissions.contains(permission)
        } else {
            false
        }
    }

    pub fn get_plugins_by_hook(&self, hook: &PluginHook) -> Vec<&Plugin> {
        self.plugins.values()
            .filter(|p| p.enabled && p.hooks.contains(hook))
            .collect()
    }

    pub fn search_plugins(&self, query: &str) -> Vec<&Plugin> {
        let query_lower = query.to_lowercase();
        self.plugins.values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower) ||
                p.description.to_lowercase().contains(&query_lower) ||
                p.author.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn export_plugin_settings(&self, plugin_id: &str) -> Result<String, String> {
        let plugin = self.plugins.get(plugin_id)
            .ok_or("Plugin not found")?;
        
        serde_json::to_string_pretty(&plugin.settings)
            .map_err(|e| format!("Failed to export settings: {}", e))
    }

    pub fn import_plugin_settings(&mut self, plugin_id: &str, settings_data: &str) -> Result<(), String> {
        let settings: HashMap<String, PluginSetting> = serde_json::from_str(settings_data)
            .map_err(|e| format!("Invalid settings format: {}", e))?;
        
        let plugin = self.plugins.get_mut(plugin_id)
            .ok_or("Plugin not found")?;
        
        for (key, setting) in settings {
            if plugin.settings.contains_key(&key) {
                plugin.settings.insert(key, setting);
            }
        }
        
        plugin.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn get_stats(&self) -> &PluginStats {
        &self.stats
    }

    fn update_stats(&mut self) {
        self.stats.total_plugins = self.plugins.len();
        self.stats.enabled_plugins = self.plugins.values().filter(|p| p.enabled).count();
        self.stats.disabled_plugins = self.stats.total_plugins - self.stats.enabled_plugins;
    }

    pub fn validate_plugin_manifest(&self, manifest_path: &str) -> Result<PluginManifest, String> {
        let manifest_content = std::fs::read_to_string(manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;
        
        let manifest: PluginManifest = serde_json::from_str(&manifest_content)
            .map_err(|e| format!("Invalid manifest format: {}", e))?;
        
        if manifest.name.is_empty() {
            return Err("Plugin name cannot be empty".to_string());
        }
        
        if manifest.version.is_empty() {
            return Err("Plugin version cannot be empty".to_string());
        }
        
        Ok(manifest)
    }
}

#[tauri::command]
pub async fn install_plugin(manifest_path: String) -> Result<String, String> {
    let mut manager = PLUGIN_MANAGER.write().await;
    manager.install_plugin(&manifest_path)
}

#[tauri::command]
pub async fn uninstall_plugin(plugin_id: String) -> Result<(), String> {
    let mut manager = PLUGIN_MANAGER.write().await;
    manager.uninstall_plugin(&plugin_id)
}

#[tauri::command]
pub async fn enable_plugin(plugin_id: String) -> Result<(), String> {
    let mut manager = PLUGIN_MANAGER.write().await;
    manager.enable_plugin(&plugin_id)
}

#[tauri::command]
pub async fn disable_plugin(plugin_id: String) -> Result<(), String> {
    let mut manager = PLUGIN_MANAGER.write().await;
    manager.disable_plugin(&plugin_id)
}

#[tauri::command]
pub async fn get_plugin(plugin_id: String) -> Result<Option<Plugin>, String> {
    let manager = PLUGIN_MANAGER.read().await;
    Ok(manager.get_plugin(&plugin_id).cloned())
}

#[tauri::command]
pub async fn get_all_plugins() -> Result<Vec<Plugin>, String> {
    let manager = PLUGIN_MANAGER.read().await;
    Ok(manager.get_all_plugins().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_enabled_plugins() -> Result<Vec<Plugin>, String> {
    let manager = PLUGIN_MANAGER.read().await;
    Ok(manager.get_enabled_plugins().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn update_plugin_setting(plugin_id: String, key: String, value: serde_json::Value) -> Result<(), String> {
    let mut manager = PLUGIN_MANAGER.write().await;
    manager.update_plugin_setting(&plugin_id, &key, value)
}

#[tauri::command]
pub async fn get_plugin_setting(plugin_id: String, key: String) -> Result<Option<PluginSetting>, String> {
    let manager = PLUGIN_MANAGER.read().await;
    Ok(manager.get_plugin_setting(&plugin_id, &key).cloned())
}

#[tauri::command]
pub async fn trigger_plugin_event(hook: PluginHook, data: serde_json::Value) -> Result<Vec<String>, String> {
    let mut manager = PLUGIN_MANAGER.write().await;
    Ok(manager.trigger_event(hook, data))
}

#[tauri::command]
pub async fn has_plugin_permission(plugin_id: String, permission: PluginPermission) -> Result<bool, String> {
    let manager = PLUGIN_MANAGER.read().await;
    Ok(manager.has_permission(&plugin_id, &permission))
}

#[tauri::command]
pub async fn get_plugins_by_hook(hook: PluginHook) -> Result<Vec<Plugin>, String> {
    let manager = PLUGIN_MANAGER.read().await;
    Ok(manager.get_plugins_by_hook(&hook).into_iter().cloned().collect())
}

#[tauri::command]
pub async fn search_plugins(query: String) -> Result<Vec<Plugin>, String> {
    let manager = PLUGIN_MANAGER.read().await;
    Ok(manager.search_plugins(&query).into_iter().cloned().collect())
}

#[tauri::command]
pub async fn export_plugin_settings(plugin_id: String) -> Result<String, String> {
    let manager = PLUGIN_MANAGER.read().await;
    manager.export_plugin_settings(&plugin_id)
}

#[tauri::command]
pub async fn import_plugin_settings(plugin_id: String, settings_data: String) -> Result<(), String> {
    let mut manager = PLUGIN_MANAGER.write().await;
    manager.import_plugin_settings(&plugin_id, &settings_data)
}

#[tauri::command]
pub async fn get_plugin_stats() -> Result<PluginStats, String> {
    let manager = PLUGIN_MANAGER.read().await;
    Ok(manager.get_stats().clone())
}

#[tauri::command]
pub async fn validate_plugin_manifest(manifest_path: String) -> Result<PluginManifest, String> {
    let manager = PLUGIN_MANAGER.read().await;
    manager.validate_plugin_manifest(&manifest_path)
}