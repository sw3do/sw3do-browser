use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub windows: Vec<WindowSession>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_saved: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSession {
    pub id: String,
    pub is_private: bool,
    pub tabs: Vec<TabSession>,
    pub active_tab_index: Option<usize>,
    pub bounds: WindowBounds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabSession {
    pub id: String,
    pub url: String,
    pub title: String,
    pub favicon: Option<String>,
    pub history: Vec<HistoryEntry>,
    pub history_index: usize,
    pub scroll_position: ScrollPosition,
    pub form_data: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

static SESSION_MANAGER: Lazy<RwLock<SessionManager>> = Lazy::new(|| {
    RwLock::new(SessionManager::new())
});

pub struct SessionManager {
    pub current_session: Option<SessionData>,
    pub saved_sessions: HashMap<String, SessionData>,
    pub auto_save_enabled: bool,
}

impl Default for ScrollPosition {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Default for WindowBounds {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 1200,
            height: 800,
            maximized: false,
        }
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            current_session: None,
            saved_sessions: HashMap::new(),
            auto_save_enabled: true,
        }
    }

    pub fn create_session(&mut self) -> String {
        let session_id = Uuid::new_v4().to_string();
        let session = SessionData {
            windows: Vec::new(),
            created_at: chrono::Utc::now(),
            last_saved: chrono::Utc::now(),
        };
        
        self.current_session = Some(session.clone());
        self.saved_sessions.insert(session_id.clone(), session);
        
        session_id
    }

    pub fn save_current_session(&mut self) -> Result<String, String> {
        if let Some(ref mut session) = self.current_session {
            session.last_saved = chrono::Utc::now();
            let session_id = Uuid::new_v4().to_string();
            self.saved_sessions.insert(session_id.clone(), session.clone());
            Ok(session_id)
        } else {
            Err("No current session to save".to_string())
        }
    }

    pub fn restore_session(&mut self, session_id: &str) -> Result<SessionData, String> {
        let session = self.saved_sessions.get(session_id)
            .ok_or("Session not found")?;
        
        self.current_session = Some(session.clone());
        Ok(session.clone())
    }

    pub fn add_window_to_session(&mut self, window_session: WindowSession) {
        if let Some(ref mut session) = self.current_session {
            session.windows.push(window_session);
            session.last_saved = chrono::Utc::now();
        } else {
            let session = SessionData {
                windows: vec![window_session],
                created_at: chrono::Utc::now(),
                last_saved: chrono::Utc::now(),
            };
            self.current_session = Some(session);
        }
    }

    pub fn remove_window_from_session(&mut self, window_id: &str) {
        if let Some(ref mut session) = self.current_session {
            session.windows.retain(|w| w.id != window_id);
            session.last_saved = chrono::Utc::now();
        }
    }

    pub fn update_window_in_session(&mut self, window_id: &str, window_session: WindowSession) {
        if let Some(ref mut session) = self.current_session {
            if let Some(window) = session.windows.iter_mut().find(|w| w.id == window_id) {
                *window = window_session;
                session.last_saved = chrono::Utc::now();
            }
        }
    }

    pub fn add_tab_to_window(&mut self, window_id: &str, tab_session: TabSession) {
        if let Some(ref mut session) = self.current_session {
            if let Some(window) = session.windows.iter_mut().find(|w| w.id == window_id) {
                window.tabs.push(tab_session);
                session.last_saved = chrono::Utc::now();
            }
        }
    }

    pub fn remove_tab_from_window(&mut self, window_id: &str, tab_id: &str) {
        if let Some(ref mut session) = self.current_session {
            if let Some(window) = session.windows.iter_mut().find(|w| w.id == window_id) {
                window.tabs.retain(|t| t.id != tab_id);
                session.last_saved = chrono::Utc::now();
            }
        }
    }

    pub fn update_tab_in_window(&mut self, window_id: &str, tab_id: &str, tab_session: TabSession) {
        if let Some(ref mut session) = self.current_session {
            if let Some(window) = session.windows.iter_mut().find(|w| w.id == window_id) {
                if let Some(tab) = window.tabs.iter_mut().find(|t| t.id == tab_id) {
                    *tab = tab_session;
                    session.last_saved = chrono::Utc::now();
                }
            }
        }
    }

    pub fn update_tab_scroll_position(&mut self, window_id: &str, tab_id: &str, scroll: ScrollPosition) {
        if let Some(ref mut session) = self.current_session {
            if let Some(window) = session.windows.iter_mut().find(|w| w.id == window_id) {
                if let Some(tab) = window.tabs.iter_mut().find(|t| t.id == tab_id) {
                    tab.scroll_position = scroll;
                    session.last_saved = chrono::Utc::now();
                }
            }
        }
    }

    pub fn set_active_tab(&mut self, window_id: &str, tab_index: usize) {
        if let Some(ref mut session) = self.current_session {
            if let Some(window) = session.windows.iter_mut().find(|w| w.id == window_id) {
                if tab_index < window.tabs.len() {
                    window.active_tab_index = Some(tab_index);
                    session.last_saved = chrono::Utc::now();
                }
            }
        }
    }

    pub fn get_saved_sessions(&self) -> Vec<(&String, &SessionData)> {
        let mut sessions: Vec<(&String, &SessionData)> = self.saved_sessions.iter().collect();
        sessions.sort_by(|a, b| b.1.last_saved.cmp(&a.1.last_saved));
        sessions
    }

    pub fn delete_session(&mut self, session_id: &str) -> Result<(), String> {
        self.saved_sessions.remove(session_id)
            .ok_or("Session not found")?;
        Ok(())
    }

    pub fn clear_old_sessions(&mut self, days: i64) {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(days);
        self.saved_sessions.retain(|_, session| session.last_saved >= cutoff);
    }

    pub fn export_session(&self, session_id: &str) -> Result<String, String> {
        let session = self.saved_sessions.get(session_id)
            .ok_or("Session not found")?;
        
        serde_json::to_string_pretty(session)
            .map_err(|e| format!("Failed to export session: {}", e))
    }

    pub fn import_session(&mut self, data: &str) -> Result<String, String> {
        let session: SessionData = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse session data: {}", e))?;
        
        let session_id = Uuid::new_v4().to_string();
        self.saved_sessions.insert(session_id.clone(), session);
        
        Ok(session_id)
    }

    pub fn get_current_session(&self) -> Option<&SessionData> {
        self.current_session.as_ref()
    }

    pub fn enable_auto_save(&mut self, enabled: bool) {
        self.auto_save_enabled = enabled;
    }

    pub fn should_auto_save(&self) -> bool {
        self.auto_save_enabled
    }
}

#[tauri::command]
pub async fn create_session() -> Result<String, String> {
    let mut manager = SESSION_MANAGER.write().await;
    Ok(manager.create_session())
}

#[tauri::command]
pub async fn save_current_session() -> Result<String, String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.save_current_session()
}

#[tauri::command]
pub async fn restore_session(session_id: String) -> Result<SessionData, String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.restore_session(&session_id)
}

#[tauri::command]
pub async fn add_window_to_session(window_session: WindowSession) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.add_window_to_session(window_session);
    Ok(())
}

#[tauri::command]
pub async fn remove_window_from_session(window_id: String) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.remove_window_from_session(&window_id);
    Ok(())
}

#[tauri::command]
pub async fn update_window_in_session(window_id: String, window_session: WindowSession) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.update_window_in_session(&window_id, window_session);
    Ok(())
}

#[tauri::command]
pub async fn add_tab_to_window(window_id: String, tab_session: TabSession) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.add_tab_to_window(&window_id, tab_session);
    Ok(())
}

#[tauri::command]
pub async fn remove_tab_from_window(window_id: String, tab_id: String) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.remove_tab_from_window(&window_id, &tab_id);
    Ok(())
}

#[tauri::command]
pub async fn update_tab_in_window(window_id: String, tab_id: String, tab_session: TabSession) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.update_tab_in_window(&window_id, &tab_id, tab_session);
    Ok(())
}

#[tauri::command]
pub async fn update_tab_scroll_position(window_id: String, tab_id: String, scroll: ScrollPosition) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.update_tab_scroll_position(&window_id, &tab_id, scroll);
    Ok(())
}

#[tauri::command]
pub async fn set_session_active_tab(window_id: String, tab_index: usize) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.set_active_tab(&window_id, tab_index);
    Ok(())
}

#[tauri::command]
pub async fn get_saved_sessions() -> Result<Vec<(String, SessionData)>, String> {
    let manager = SESSION_MANAGER.read().await;
    Ok(manager.get_saved_sessions().into_iter().map(|(id, session)| (id.clone(), session.clone())).collect())
}

#[tauri::command]
pub async fn delete_session(session_id: String) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.delete_session(&session_id)
}

#[tauri::command]
pub async fn clear_old_sessions(days: i64) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.clear_old_sessions(days);
    Ok(())
}

#[tauri::command]
pub async fn export_session(session_id: String) -> Result<String, String> {
    let manager = SESSION_MANAGER.read().await;
    manager.export_session(&session_id)
}

#[tauri::command]
pub async fn import_session(data: String) -> Result<String, String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.import_session(&data)
}

#[tauri::command]
pub async fn get_current_session() -> Result<Option<SessionData>, String> {
    let manager = SESSION_MANAGER.read().await;
    Ok(manager.get_current_session().cloned())
}

#[tauri::command]
pub async fn enable_auto_save(enabled: bool) -> Result<(), String> {
    let mut manager = SESSION_MANAGER.write().await;
    manager.enable_auto_save(enabled);
    Ok(())
}