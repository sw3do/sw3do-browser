use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use uuid::Uuid;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserWindow {
    pub id: String,
    pub is_private: bool,
    pub tabs: Vec<String>,
    pub active_tab: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTab {
    pub id: String,
    pub window_id: String,
    pub url: String,
    pub title: String,
    pub favicon: Option<String>,
    pub is_loading: bool,
    pub is_private: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationEntry {
    pub url: String,
    pub title: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

static BROWSER_ENGINE: Lazy<RwLock<BrowserEngine>> = Lazy::new(|| {
    RwLock::new(BrowserEngine::new())
});

pub struct BrowserEngine {
    pub windows: HashMap<String, BrowserWindow>,
    pub tabs: HashMap<String, BrowserTab>,
}

impl BrowserEngine {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            tabs: HashMap::new(),
        }
    }

    pub fn create_window(&mut self, is_private: bool) -> String {
        let window_id = Uuid::new_v4().to_string();
        let window = BrowserWindow {
            id: window_id.clone(),
            is_private,
            tabs: Vec::new(),
            active_tab: None,
            created_at: chrono::Utc::now(),
        };
        self.windows.insert(window_id.clone(), window);
        window_id
    }

    pub fn create_tab(&mut self, window_id: &str, url: &str, is_private: bool) -> Result<String, String> {
        if !self.windows.contains_key(window_id) {
            return Err("Window not found".to_string());
        }

        let tab_id = Uuid::new_v4().to_string();
        let tab = BrowserTab {
            id: tab_id.clone(),
            window_id: window_id.to_string(),
            url: url.to_string(),
            title: "Loading...".to_string(),
            favicon: None,
            is_loading: true,
            is_private,
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
        };

        self.tabs.insert(tab_id.clone(), tab);
        
        if let Some(window) = self.windows.get_mut(window_id) {
            window.tabs.push(tab_id.clone());
            if window.active_tab.is_none() {
                window.active_tab = Some(tab_id.clone());
            }
        }

        Ok(tab_id)
    }

    pub fn close_tab(&mut self, tab_id: &str) -> Result<(), String> {
        let tab = self.tabs.get(tab_id).ok_or("Tab not found")?;
        let window_id = tab.window_id.clone();
        
        self.tabs.remove(tab_id);
        
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.tabs.retain(|id| id != tab_id);
            
            if window.active_tab.as_ref() == Some(&tab_id.to_string()) {
                window.active_tab = window.tabs.first().cloned();
            }
        }

        Ok(())
    }

    pub fn set_active_tab(&mut self, window_id: &str, tab_id: &str) -> Result<(), String> {
        let window = self.windows.get_mut(window_id).ok_or("Window not found")?;
        
        if !window.tabs.contains(&tab_id.to_string()) {
            return Err("Tab not in window".to_string());
        }

        window.active_tab = Some(tab_id.to_string());
        
        if let Some(tab) = self.tabs.get_mut(tab_id) {
            tab.last_accessed = chrono::Utc::now();
        }

        Ok(())
    }

    pub fn update_tab_url(&mut self, tab_id: &str, url: &str, title: Option<&str>) -> Result<(), String> {
        let tab = self.tabs.get_mut(tab_id).ok_or("Tab not found")?;
        tab.url = url.to_string();
        tab.is_loading = false;
        tab.last_accessed = chrono::Utc::now();
        
        if let Some(title) = title {
            tab.title = title.to_string();
        }

        Ok(())
    }

    pub fn get_window_tabs(&self, window_id: &str) -> Vec<&BrowserTab> {
        if let Some(window) = self.windows.get(window_id) {
            window.tabs.iter()
                .filter_map(|tab_id| self.tabs.get(tab_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_active_tab(&self, window_id: &str) -> Option<&BrowserTab> {
        let window = self.windows.get(window_id)?;
        let active_tab_id = window.active_tab.as_ref()?;
        self.tabs.get(active_tab_id)
    }
}

#[tauri::command]
pub async fn create_browser_window(app: AppHandle, is_private: bool) -> Result<String, String> {
    let window_id = Uuid::new_v4().to_string();
    let window_label = format!("browser-{}", window_id);
    
    let builder = WebviewWindowBuilder::new(
        &app,
        &window_label,
        WebviewUrl::App("index.html".into())
    )
    .title(if is_private { "Sw3do Browser (Private)" } else { "Sw3do Browser" })
    .inner_size(1200.0, 800.0)
    .min_inner_size(800.0, 600.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .closable(true)
    .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Sw3doBrowser/1.0")
    .accept_first_mouse(true);

    match builder.build() {
        Ok(_) => {
            let mut engine = BROWSER_ENGINE.write().await;
            let window_id_clone = window_id.clone();
            engine.create_window(is_private);
            Ok(window_id_clone)
        },
        Err(e) => Err(format!("Failed to create window: {}", e)),
    }
}

#[tauri::command]
pub async fn close_browser_window(app: AppHandle, window_id: String) -> Result<(), String> {
    let window_label = format!("browser-{}", window_id);
    
    {
        let mut engine = BROWSER_ENGINE.write().await;
        let tab_ids = engine.windows.get(&window_id)
            .map(|window| window.tabs.clone())
            .unwrap_or_default();
        
        for tab_id in &tab_ids {
            engine.tabs.remove(tab_id);
        }
        engine.windows.remove(&window_id);
    }
    
    if let Some(window) = app.get_webview_window(&window_label) {
        window.close().map_err(|e| format!("Failed to close window: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_browser_window(window_id: String) -> Result<Option<BrowserWindow>, String> {
    let engine = BROWSER_ENGINE.read().await;
    Ok(engine.windows.get(&window_id).cloned())
}

#[tauri::command]
pub async fn get_all_windows() -> Result<Vec<BrowserWindow>, String> {
    let engine = BROWSER_ENGINE.read().await;
    Ok(engine.windows.values().cloned().collect())
}

#[tauri::command]
pub async fn create_engine_tab(window_id: String, url: String, is_private: bool) -> Result<String, String> {
    let mut engine = BROWSER_ENGINE.write().await;
    engine.create_tab(&window_id, &url, is_private)
}

#[tauri::command]
pub async fn close_engine_tab(tab_id: String) -> Result<(), String> {
    let mut engine = BROWSER_ENGINE.write().await;
    engine.close_tab(&tab_id)
}

#[tauri::command]
pub async fn update_engine_tab_url(tab_id: String, url: String, title: Option<String>) -> Result<(), String> {
    let mut engine = BROWSER_ENGINE.write().await;
    engine.update_tab_url(&tab_id, &url, title.as_deref())
}

#[tauri::command]
pub async fn set_engine_active_tab(window_id: String, tab_id: String) -> Result<(), String> {
    let mut engine = BROWSER_ENGINE.write().await;
    engine.set_active_tab(&window_id, &tab_id)
}

#[tauri::command]
pub async fn get_engine_window_tabs(window_id: String) -> Result<Vec<BrowserTab>, String> {
    let engine = BROWSER_ENGINE.read().await;
    Ok(engine.get_window_tabs(&window_id).into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_engine_active_tab(window_id: String) -> Result<Option<BrowserTab>, String> {
    let engine = BROWSER_ENGINE.read().await;
    Ok(engine.get_active_tab(&window_id).cloned())
}

#[tauri::command]
pub async fn create_webview_tab(app: AppHandle, tab_id: String, url: String) -> Result<(), String> {
    let webview_label = format!("webview-{}", tab_id);
    
    let builder = WebviewWindowBuilder::new(
        &app,
        &webview_label,
        WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?)
    )
    .title("Loading...")
    .inner_size(1200.0, 800.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .closable(true)
    .visible(false)
    .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Sw3doBrowser/1.0")
    .accept_first_mouse(true);

    match builder.build() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create webview tab: {}", e)),
    }
}

#[tauri::command]
pub async fn show_webview_tab(app: AppHandle, tab_id: String) -> Result<(), String> {
    let webview_label = format!("webview-{}", tab_id);
    
    if let Some(webview) = app.get_webview_window(&webview_label) {
        webview.show().map_err(|e| format!("Failed to show webview: {}", e))?;
        webview.set_focus().map_err(|e| format!("Failed to focus webview: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn hide_webview_tab(app: AppHandle, tab_id: String) -> Result<(), String> {
    let webview_label = format!("webview-{}", tab_id);
    
    if let Some(webview) = app.get_webview_window(&webview_label) {
        webview.hide().map_err(|e| format!("Failed to hide webview: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn close_webview_tab(app: AppHandle, tab_id: String) -> Result<(), String> {
    let webview_label = format!("webview-{}", tab_id);
    
    if let Some(webview) = app.get_webview_window(&webview_label) {
        webview.close().map_err(|e| format!("Failed to close webview: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn navigate_webview_tab(app: AppHandle, tab_id: String, url: String) -> Result<(), String> {
    let webview_label = format!("webview-{}", tab_id);
    
    if let Some(webview) = app.get_webview_window(&webview_label) {
        let js_code = format!("window.location.href = '{}';", url.replace("'", "\\'")); 
        webview.eval(&js_code).map_err(|e| format!("Failed to navigate webview: {}", e))?;
    }
    
    Ok(())
}