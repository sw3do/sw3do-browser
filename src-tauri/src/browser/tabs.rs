use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: String,
    pub window_id: String,
    pub url: String,
    pub title: String,
    pub favicon: Option<String>,
    pub is_loading: bool,
    pub is_pinned: bool,
    pub is_muted: bool,
    pub is_private: bool,
    pub zoom_level: f64,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabStats {
    pub total_tabs: usize,
    pub active_tabs: usize,
    pub pinned_tabs: usize,
    pub private_tabs: usize,
    pub loading_tabs: usize,
}

static TAB_MANAGER: Lazy<RwLock<TabManager>> = Lazy::new(|| {
    RwLock::new(TabManager::new())
});

pub struct TabManager {
    pub tabs: HashMap<String, Tab>,
    pub window_tabs: HashMap<String, Vec<String>>,
    pub active_tabs: HashMap<String, String>,
}

impl Tab {
    pub fn new(window_id: String, url: String, is_private: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            window_id,
            url: url.clone(),
            title: url,
            favicon: None,
            is_loading: false,
            is_pinned: false,
            is_muted: false,
            is_private,
            zoom_level: 1.0,
            can_go_back: false,
            can_go_forward: false,
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
        }
    }

    pub fn update_url(&mut self, url: String, title: Option<String>) {
        self.url = url;
        if let Some(title) = title {
            self.title = title;
        }
        self.last_accessed = chrono::Utc::now();
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
        self.last_accessed = chrono::Utc::now();
    }

    pub fn set_favicon(&mut self, favicon: Option<String>) {
        self.favicon = favicon;
    }

    pub fn set_navigation_state(&mut self, can_go_back: bool, can_go_forward: bool) {
        self.can_go_back = can_go_back;
        self.can_go_forward = can_go_forward;
    }

    pub fn set_zoom_level(&mut self, zoom_level: f64) {
        self.zoom_level = zoom_level.max(0.25).min(5.0);
    }
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            window_tabs: HashMap::new(),
            active_tabs: HashMap::new(),
        }
    }

    pub fn create_tab(&mut self, window_id: String, url: String, is_private: bool) -> String {
        let tab = Tab::new(window_id.clone(), url, is_private);
        let tab_id = tab.id.clone();
        
        if !self.window_tabs.contains_key(&window_id) {
            self.window_tabs.insert(window_id.clone(), Vec::new());
        }
        
        self.tabs.insert(tab_id.clone(), tab);
        
        if let Some(window_tabs) = self.window_tabs.get_mut(&window_id) {
            window_tabs.push(tab_id.clone());
        }
        
        if !self.active_tabs.contains_key(&window_id) {
            self.active_tabs.insert(window_id, tab_id.clone());
        }
        
        tab_id
    }

    pub fn close_tab(&mut self, tab_id: &str) -> Result<(), String> {
        let tab = self.tabs.get(tab_id).ok_or("Tab not found")?;
        let window_id = tab.window_id.clone();
        
        self.tabs.remove(tab_id);
        
        if let Some(window_tabs) = self.window_tabs.get_mut(&window_id) {
            window_tabs.retain(|id| id != tab_id);
            
            if let Some(active_tab_id) = self.active_tabs.get(&window_id) {
                if active_tab_id == tab_id {
                    if let Some(new_active) = window_tabs.first() {
                        self.active_tabs.insert(window_id.clone(), new_active.clone());
                    } else {
                        self.active_tabs.remove(&window_id);
                    }
                }
            }
            
            if window_tabs.is_empty() {
                self.window_tabs.remove(&window_id);
            }
        }
        
        Ok(())
    }

    pub fn update_tab_url(&mut self, tab_id: &str, url: String, title: Option<String>) -> Result<(), String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        tab.update_url(url, title);
        Ok(())
    }

    pub fn get_tab(&self, tab_id: &str) -> Option<&Tab> {
        self.tabs.get(tab_id)
    }

    pub fn get_all_tabs(&self) -> Vec<&Tab> {
        self.tabs.values().collect()
    }

    pub fn get_window_tabs(&self, window_id: &str) -> Vec<&Tab> {
        if let Some(tab_ids) = self.window_tabs.get(window_id) {
            tab_ids.iter()
                .filter_map(|id| self.tabs.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn set_active_tab(&mut self, window_id: &str, tab_id: &str) -> Result<(), String> {
        if !self.tabs.contains_key(tab_id) {
            return Err("Tab not found".to_string());
        }
        
        let tab = self.tabs.get(tab_id).unwrap();
        if tab.window_id != window_id {
            return Err("Tab does not belong to this window".to_string());
        }
        
        self.active_tabs.insert(window_id.to_string(), tab_id.to_string());
        
        if let Some(tab) = self.tabs.get_mut(tab_id) {
            tab.last_accessed = chrono::Utc::now();
        }
        
        Ok(())
    }

    pub fn get_active_tab(&self, window_id: &str) -> Option<&Tab> {
        let active_tab_id = self.active_tabs.get(window_id)?;
        self.tabs.get(active_tab_id)
    }

    pub fn duplicate_tab(&mut self, tab_id: &str) -> Result<String, String> {
        let original_tab = self.tabs.get(tab_id)
            .ok_or("Tab not found")?
            .clone();
        
        let new_tab_id = self.create_tab(
            original_tab.window_id.clone(),
            original_tab.url.clone(),
            original_tab.is_private
        );
        
        if let Some(new_tab) = self.tabs.get_mut(&new_tab_id) {
            new_tab.title = original_tab.title;
            new_tab.favicon = original_tab.favicon;
            new_tab.zoom_level = original_tab.zoom_level;
        }
        
        Ok(new_tab_id)
    }

    pub fn move_tab(&mut self, tab_id: &str, from_window: &str, to_window: &str, index: Option<usize>) -> Result<(), String> {
        let mut tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?
            .clone();
        
        if let Some(from_tabs) = self.window_tabs.get_mut(from_window) {
            from_tabs.retain(|id| id != tab_id);
        }
        
        tab.window_id = to_window.to_string();
        self.tabs.insert(tab_id.to_string(), tab);
        
        let to_tabs = self.window_tabs.entry(to_window.to_string())
            .or_insert_with(Vec::new);
        
        if let Some(index) = index {
            let insert_index = index.min(to_tabs.len());
            to_tabs.insert(insert_index, tab_id.to_string());
        } else {
            to_tabs.push(tab_id.to_string());
        }
        
        Ok(())
    }

    pub fn pin_tab(&mut self, tab_id: &str) -> Result<(), String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        tab.is_pinned = true;
        Ok(())
    }

    pub fn unpin_tab(&mut self, tab_id: &str) -> Result<(), String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        tab.is_pinned = false;
        Ok(())
    }

    pub fn mute_tab(&mut self, tab_id: &str) -> Result<(), String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        tab.is_muted = true;
        Ok(())
    }

    pub fn unmute_tab(&mut self, tab_id: &str) -> Result<(), String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        tab.is_muted = false;
        Ok(())
    }

    pub fn set_tab_loading(&mut self, tab_id: &str, loading: bool) -> Result<(), String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        tab.set_loading(loading);
        Ok(())
    }

    pub fn set_tab_favicon(&mut self, tab_id: &str, favicon: Option<String>) -> Result<(), String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        tab.set_favicon(favicon);
        Ok(())
    }

    pub fn set_tab_navigation_state(&mut self, tab_id: &str, can_go_back: bool, can_go_forward: bool) -> Result<(), String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        tab.set_navigation_state(can_go_back, can_go_forward);
        Ok(())
    }

    pub fn zoom_in(&mut self, tab_id: &str) -> Result<f64, String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        let new_zoom = (tab.zoom_level * 1.2).min(5.0);
        tab.set_zoom_level(new_zoom);
        Ok(new_zoom)
    }

    pub fn zoom_out(&mut self, tab_id: &str) -> Result<f64, String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        let new_zoom = (tab.zoom_level / 1.2).max(0.25);
        tab.set_zoom_level(new_zoom);
        Ok(new_zoom)
    }

    pub fn reset_zoom(&mut self, tab_id: &str) -> Result<f64, String> {
        let tab = self.tabs.get_mut(tab_id)
            .ok_or("Tab not found")?;
        
        tab.set_zoom_level(1.0);
        Ok(1.0)
    }

    pub fn get_tab_stats(&self) -> TabStats {
        let total_tabs = self.tabs.len();
        let pinned_tabs = self.tabs.values().filter(|t| t.is_pinned).count();
        let private_tabs = self.tabs.values().filter(|t| t.is_private).count();
        let loading_tabs = self.tabs.values().filter(|t| t.is_loading).count();
        let active_tabs = self.active_tabs.len();
        
        TabStats {
            total_tabs,
            active_tabs,
            pinned_tabs,
            private_tabs,
            loading_tabs,
        }
    }

    pub fn close_window_tabs(&mut self, window_id: &str) {
        if let Some(tab_ids) = self.window_tabs.remove(window_id) {
            for tab_id in tab_ids {
                self.tabs.remove(&tab_id);
            }
        }
        self.active_tabs.remove(window_id);
    }
}

#[tauri::command]
pub async fn create_tab(window_id: String, url: String, is_private: bool) -> Result<String, String> {
    let mut manager = TAB_MANAGER.write().await;
    Ok(manager.create_tab(window_id, url, is_private))
}

#[tauri::command]
pub async fn close_tab(tab_id: String) -> Result<(), String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.close_tab(&tab_id)
}

#[tauri::command]
pub async fn update_tab_url(tab_id: String, url: String, title: Option<String>) -> Result<(), String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.update_tab_url(&tab_id, url, title)
}

#[tauri::command]
pub async fn get_tab(tab_id: String) -> Result<Option<Tab>, String> {
    let manager = TAB_MANAGER.read().await;
    Ok(manager.get_tab(&tab_id).cloned())
}

#[tauri::command]
pub async fn get_all_tabs() -> Result<Vec<Tab>, String> {
    let manager = TAB_MANAGER.read().await;
    Ok(manager.get_all_tabs().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn set_active_tab(window_id: String, tab_id: String) -> Result<(), String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.set_active_tab(&window_id, &tab_id)
}

#[tauri::command]
pub async fn duplicate_tab(tab_id: String) -> Result<String, String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.duplicate_tab(&tab_id)
}

#[tauri::command]
pub async fn move_tab(tab_id: String, from_window: String, to_window: String, index: Option<usize>) -> Result<(), String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.move_tab(&tab_id, &from_window, &to_window, index)
}

#[tauri::command]
pub async fn pin_tab(tab_id: String) -> Result<(), String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.pin_tab(&tab_id)
}

#[tauri::command]
pub async fn unpin_tab(tab_id: String) -> Result<(), String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.unpin_tab(&tab_id)
}

#[tauri::command]
pub async fn mute_tab(tab_id: String) -> Result<(), String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.mute_tab(&tab_id)
}

#[tauri::command]
pub async fn unmute_tab(tab_id: String) -> Result<(), String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.unmute_tab(&tab_id)
}

#[tauri::command]
pub async fn reload_tab(tab_id: String) -> Result<(), String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.set_tab_loading(&tab_id, true)
}

#[tauri::command]
pub async fn stop_tab_loading(tab_id: String) -> Result<(), String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.set_tab_loading(&tab_id, false)
}

#[tauri::command]
pub async fn go_back(tab_id: String) -> Result<(), String> {
    let manager = TAB_MANAGER.read().await;
    if let Some(tab) = manager.get_tab(&tab_id) {
        if tab.can_go_back {
            Ok(())
        } else {
            Err("Cannot go back".to_string())
        }
    } else {
        Err("Tab not found".to_string())
    }
}

#[tauri::command]
pub async fn go_forward(tab_id: String) -> Result<(), String> {
    let manager = TAB_MANAGER.read().await;
    if let Some(tab) = manager.get_tab(&tab_id) {
        if tab.can_go_forward {
            Ok(())
        } else {
            Err("Cannot go forward".to_string())
        }
    } else {
        Err("Tab not found".to_string())
    }
}

#[tauri::command]
pub async fn zoom_in(tab_id: String) -> Result<f64, String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.zoom_in(&tab_id)
}

#[tauri::command]
pub async fn zoom_out(tab_id: String) -> Result<f64, String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.zoom_out(&tab_id)
}

#[tauri::command]
pub async fn reset_zoom(tab_id: String) -> Result<f64, String> {
    let mut manager = TAB_MANAGER.write().await;
    manager.reset_zoom(&tab_id)
}