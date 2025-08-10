use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub url: String,
    pub title: String,
    pub visit_time: chrono::DateTime<chrono::Utc>,
    pub visit_count: u32,
    pub last_visit: chrono::DateTime<chrono::Utc>,
    pub favicon: Option<String>,
    pub is_private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStats {
    pub total_visits: u64,
    pub unique_sites: u64,
    pub today_visits: u64,
    pub this_week_visits: u64,
    pub this_month_visits: u64,
}

static HISTORY_MANAGER: Lazy<RwLock<HistoryManager>> = Lazy::new(|| {
    RwLock::new(HistoryManager::new())
});

pub struct HistoryManager {
    pub entries: HashMap<String, HistoryEntry>,
    pub url_to_id: HashMap<String, String>,
}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            url_to_id: HashMap::new(),
        }
    }

    pub fn add_visit(&mut self, url: &str, title: &str, is_private: bool) -> String {
        if is_private {
            return String::new();
        }

        let now = chrono::Utc::now();
        
        if let Some(entry_id) = self.url_to_id.get(url) {
            if let Some(entry) = self.entries.get_mut(entry_id) {
                entry.visit_count += 1;
                entry.last_visit = now;
                entry.title = title.to_string();
                return entry_id.clone();
            }
        }
        
        let entry_id = Uuid::new_v4().to_string();
        let entry = HistoryEntry {
            id: entry_id.clone(),
            url: url.to_string(),
            title: title.to_string(),
            visit_time: now,
            visit_count: 1,
            last_visit: now,
            favicon: None,
            is_private,
        };
        
        self.entries.insert(entry_id.clone(), entry);
        self.url_to_id.insert(url.to_string(), entry_id.clone());
        
        entry_id
    }

    pub fn remove_entry(&mut self, entry_id: &str) -> Result<(), String> {
        if let Some(entry) = self.entries.remove(entry_id) {
            self.url_to_id.remove(&entry.url);
            Ok(())
        } else {
            Err("Entry not found".to_string())
        }
    }

    pub fn clear_history(&mut self, time_range: Option<chrono::Duration>) {
        if let Some(duration) = time_range {
            let cutoff = chrono::Utc::now() - duration;
            let entries_to_remove: Vec<String> = self.entries
                .iter()
                .filter(|(_, entry)| entry.last_visit >= cutoff)
                .map(|(id, _)| id.clone())
                .collect();
            
            for entry_id in entries_to_remove {
                if let Some(entry) = self.entries.remove(&entry_id) {
                    self.url_to_id.remove(&entry.url);
                }
            }
        } else {
            self.entries.clear();
            self.url_to_id.clear();
        }
    }

    pub fn search_history(&self, query: &str, limit: Option<usize>) -> Vec<&HistoryEntry> {
        let query = query.to_lowercase();
        let mut results: Vec<&HistoryEntry> = self.entries
            .values()
            .filter(|entry| {
                entry.title.to_lowercase().contains(&query) ||
                entry.url.to_lowercase().contains(&query)
            })
            .collect();
        
        results.sort_by(|a, b| b.last_visit.cmp(&a.last_visit));
        
        if let Some(limit) = limit {
            results.truncate(limit);
        }
        
        results
    }

    pub fn get_recent_history(&self, limit: usize) -> Vec<&HistoryEntry> {
        let mut entries: Vec<&HistoryEntry> = self.entries.values().collect();
        entries.sort_by(|a, b| b.last_visit.cmp(&a.last_visit));
        entries.truncate(limit);
        entries
    }

    pub fn get_most_visited(&self, limit: usize) -> Vec<&HistoryEntry> {
        let mut entries: Vec<&HistoryEntry> = self.entries.values().collect();
        entries.sort_by(|a, b| b.visit_count.cmp(&a.visit_count));
        entries.truncate(limit);
        entries
    }

    pub fn get_history_by_date(&self, date: chrono::NaiveDate) -> Vec<&HistoryEntry> {
        let start_of_day = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
        let end_of_day = date.and_hms_opt(23, 59, 59).unwrap().and_utc();
        
        let mut entries: Vec<&HistoryEntry> = self.entries
            .values()
            .filter(|entry| entry.last_visit >= start_of_day && entry.last_visit <= end_of_day)
            .collect();
        
        entries.sort_by(|a, b| b.last_visit.cmp(&a.last_visit));
        entries
    }

    pub fn get_stats(&self) -> HistoryStats {
        let now = chrono::Utc::now();
        let today = now.date_naive();
        let week_ago = now - chrono::Duration::days(7);
        let month_ago = now - chrono::Duration::days(30);
        
        let total_visits = self.entries.values().map(|e| e.visit_count as u64).sum();
        let unique_sites = self.entries.len() as u64;
        
        let today_visits = self.entries
            .values()
            .filter(|entry| entry.last_visit.date_naive() == today)
            .map(|e| e.visit_count as u64)
            .sum();
        
        let this_week_visits = self.entries
            .values()
            .filter(|entry| entry.last_visit >= week_ago)
            .map(|e| e.visit_count as u64)
            .sum();
        
        let this_month_visits = self.entries
            .values()
            .filter(|entry| entry.last_visit >= month_ago)
            .map(|e| e.visit_count as u64)
            .sum();
        
        HistoryStats {
            total_visits,
            unique_sites,
            today_visits,
            this_week_visits,
            this_month_visits,
        }
    }

    pub fn get_suggestions(&self, partial_url: &str, limit: usize) -> Vec<&HistoryEntry> {
        let partial_url = partial_url.to_lowercase();
        let mut suggestions: Vec<&HistoryEntry> = self.entries
            .values()
            .filter(|entry| {
                entry.url.to_lowercase().starts_with(&partial_url) ||
                entry.title.to_lowercase().contains(&partial_url)
            })
            .collect();
        
        suggestions.sort_by(|a, b| {
            let a_score = a.visit_count as f64 * (1.0 / (chrono::Utc::now() - a.last_visit).num_hours() as f64).max(0.01);
            let b_score = b.visit_count as f64 * (1.0 / (chrono::Utc::now() - b.last_visit).num_hours() as f64).max(0.01);
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        suggestions.truncate(limit);
        suggestions
    }

    pub fn update_favicon(&mut self, url: &str, favicon: &str) {
        if let Some(entry_id) = self.url_to_id.get(url) {
            if let Some(entry) = self.entries.get_mut(entry_id) {
                entry.favicon = Some(favicon.to_string());
            }
        }
    }

    pub fn export_history(&self) -> Result<String, String> {
        let entries: Vec<&HistoryEntry> = self.entries.values().collect();
        serde_json::to_string_pretty(&entries)
            .map_err(|e| format!("Failed to export history: {}", e))
    }

    pub fn import_history(&mut self, data: &str) -> Result<(), String> {
        let imported_entries: Vec<HistoryEntry> = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse history data: {}", e))?;
        
        for entry in imported_entries {
            self.url_to_id.insert(entry.url.clone(), entry.id.clone());
            self.entries.insert(entry.id.clone(), entry);
        }
        
        Ok(())
    }
}

#[tauri::command]
pub async fn add_history_visit(url: String, title: String, is_private: bool) -> Result<String, String> {
    let mut manager = HISTORY_MANAGER.write().await;
    Ok(manager.add_visit(&url, &title, is_private))
}

#[tauri::command]
pub async fn remove_history_entry(entry_id: String) -> Result<(), String> {
    let mut manager = HISTORY_MANAGER.write().await;
    manager.remove_entry(&entry_id)
}

#[tauri::command]
pub async fn clear_history(hours: Option<i64>) -> Result<(), String> {
    let mut manager = HISTORY_MANAGER.write().await;
    let duration = hours.map(|h| chrono::Duration::hours(h));
    manager.clear_history(duration);
    Ok(())
}

#[tauri::command]
pub async fn search_history(query: String, limit: Option<usize>) -> Result<Vec<HistoryEntry>, String> {
    let manager = HISTORY_MANAGER.read().await;
    Ok(manager.search_history(&query, limit).into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_recent_history(limit: usize) -> Result<Vec<HistoryEntry>, String> {
    let manager = HISTORY_MANAGER.read().await;
    Ok(manager.get_recent_history(limit).into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_most_visited(limit: usize) -> Result<Vec<HistoryEntry>, String> {
    let manager = HISTORY_MANAGER.read().await;
    Ok(manager.get_most_visited(limit).into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_history_by_date(date: String) -> Result<Vec<HistoryEntry>, String> {
    let manager = HISTORY_MANAGER.read().await;
    let parsed_date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format: {}", e))?;
    Ok(manager.get_history_by_date(parsed_date).into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_history_stats() -> Result<HistoryStats, String> {
    let manager = HISTORY_MANAGER.read().await;
    Ok(manager.get_stats())
}

#[tauri::command]
pub async fn get_history_suggestions(partial_url: String, limit: usize) -> Result<Vec<HistoryEntry>, String> {
    let manager = HISTORY_MANAGER.read().await;
    Ok(manager.get_suggestions(&partial_url, limit).into_iter().cloned().collect())
}

#[tauri::command]
pub async fn update_history_favicon(url: String, favicon: String) -> Result<(), String> {
    let mut manager = HISTORY_MANAGER.write().await;
    manager.update_favicon(&url, &favicon);
    Ok(())
}

#[tauri::command]
pub async fn export_history() -> Result<String, String> {
    let manager = HISTORY_MANAGER.read().await;
    manager.export_history()
}

#[tauri::command]
pub async fn import_history(data: String) -> Result<(), String> {
    let mut manager = HISTORY_MANAGER.write().await;
    manager.import_history(&data)
}