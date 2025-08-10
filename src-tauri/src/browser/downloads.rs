use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub file_path: PathBuf,
    pub mime_type: Option<String>,
    pub total_bytes: Option<u64>,
    pub downloaded_bytes: u64,
    pub status: DownloadStatus,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStats {
    pub total_downloads: u64,
    pub completed_downloads: u64,
    pub failed_downloads: u64,
    pub total_bytes_downloaded: u64,
    pub active_downloads: u64,
}

static DOWNLOAD_MANAGER: Lazy<RwLock<DownloadManager>> = Lazy::new(|| {
    RwLock::new(DownloadManager::new())
});

pub struct DownloadManager {
    pub downloads: HashMap<String, Download>,
    pub download_directory: PathBuf,
}

impl DownloadManager {
    pub fn new() -> Self {
        let download_directory = dirs::download_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        
        Self {
            downloads: HashMap::new(),
            download_directory,
        }
    }

    pub fn start_download(
        &mut self,
        url: &str,
        filename: Option<&str>,
        referrer: Option<&str>,
    ) -> Result<String, String> {
        let download_id = Uuid::new_v4().to_string();
        
        let filename = filename
            .map(|f| f.to_string())
            .or_else(|| self.extract_filename_from_url(url))
            .unwrap_or_else(|| format!("download_{}", download_id));
        
        let file_path = self.download_directory.join(&filename);
        
        let download = Download {
            id: download_id.clone(),
            url: url.to_string(),
            filename,
            file_path,
            mime_type: None,
            total_bytes: None,
            downloaded_bytes: 0,
            status: DownloadStatus::Pending,
            start_time: chrono::Utc::now(),
            end_time: None,
            error_message: None,
            referrer: referrer.map(|r| r.to_string()),
            user_agent: Some("Sw3do Browser/1.0".to_string()),
        };
        
        self.downloads.insert(download_id.clone(), download);
        Ok(download_id)
    }

    pub fn update_download_progress(
        &mut self,
        download_id: &str,
        downloaded_bytes: u64,
        total_bytes: Option<u64>,
    ) -> Result<(), String> {
        let download = self.downloads.get_mut(download_id)
            .ok_or("Download not found")?;
        
        download.downloaded_bytes = downloaded_bytes;
        if let Some(total) = total_bytes {
            download.total_bytes = Some(total);
        }
        download.status = DownloadStatus::InProgress;
        
        Ok(())
    }

    pub fn complete_download(&mut self, download_id: &str) -> Result<(), String> {
        let download = self.downloads.get_mut(download_id)
            .ok_or("Download not found")?;
        
        download.status = DownloadStatus::Completed;
        download.end_time = Some(chrono::Utc::now());
        
        Ok(())
    }

    pub fn fail_download(&mut self, download_id: &str, error: &str) -> Result<(), String> {
        let download = self.downloads.get_mut(download_id)
            .ok_or("Download not found")?;
        
        download.status = DownloadStatus::Failed;
        download.error_message = Some(error.to_string());
        download.end_time = Some(chrono::Utc::now());
        
        Ok(())
    }

    pub fn cancel_download(&mut self, download_id: &str) -> Result<(), String> {
        let download = self.downloads.get_mut(download_id)
            .ok_or("Download not found")?;
        
        download.status = DownloadStatus::Cancelled;
        download.end_time = Some(chrono::Utc::now());
        
        Ok(())
    }

    pub fn pause_download(&mut self, download_id: &str) -> Result<(), String> {
        let download = self.downloads.get_mut(download_id)
            .ok_or("Download not found")?;
        
        if matches!(download.status, DownloadStatus::InProgress) {
            download.status = DownloadStatus::Paused;
        }
        
        Ok(())
    }

    pub fn resume_download(&mut self, download_id: &str) -> Result<(), String> {
        let download = self.downloads.get_mut(download_id)
            .ok_or("Download not found")?;
        
        if matches!(download.status, DownloadStatus::Paused) {
            download.status = DownloadStatus::InProgress;
        }
        
        Ok(())
    }

    pub fn remove_download(&mut self, download_id: &str) -> Result<(), String> {
        self.downloads.remove(download_id)
            .ok_or("Download not found")?;
        Ok(())
    }

    pub fn clear_completed_downloads(&mut self) {
        self.downloads.retain(|_, download| {
            !matches!(download.status, DownloadStatus::Completed | DownloadStatus::Failed | DownloadStatus::Cancelled)
        });
    }

    pub fn get_downloads(&self) -> Vec<&Download> {
        let mut downloads: Vec<&Download> = self.downloads.values().collect();
        downloads.sort_by(|a, b| b.start_time.cmp(&a.start_time));
        downloads
    }

    pub fn get_active_downloads(&self) -> Vec<&Download> {
        self.downloads.values()
            .filter(|download| matches!(download.status, DownloadStatus::InProgress | DownloadStatus::Paused))
            .collect()
    }

    pub fn get_download_stats(&self) -> DownloadStats {
        let total_downloads = self.downloads.len() as u64;
        let completed_downloads = self.downloads.values()
            .filter(|d| matches!(d.status, DownloadStatus::Completed))
            .count() as u64;
        let failed_downloads = self.downloads.values()
            .filter(|d| matches!(d.status, DownloadStatus::Failed))
            .count() as u64;
        let total_bytes_downloaded = self.downloads.values()
            .map(|d| d.downloaded_bytes)
            .sum();
        let active_downloads = self.downloads.values()
            .filter(|d| matches!(d.status, DownloadStatus::InProgress | DownloadStatus::Paused))
            .count() as u64;
        
        DownloadStats {
            total_downloads,
            completed_downloads,
            failed_downloads,
            total_bytes_downloaded,
            active_downloads,
        }
    }

    pub fn set_download_directory(&mut self, path: PathBuf) -> Result<(), String> {
        if !path.exists() {
            std::fs::create_dir_all(&path)
                .map_err(|e| format!("Failed to create download directory: {}", e))?;
        }
        
        if !path.is_dir() {
            return Err("Path is not a directory".to_string());
        }
        
        self.download_directory = path;
        Ok(())
    }

    fn extract_filename_from_url(&self, url: &str) -> Option<String> {
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(segments) = parsed_url.path_segments() {
                if let Some(last_segment) = segments.last() {
                    if !last_segment.is_empty() {
                        return Some(last_segment.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn get_download_progress(&self, download_id: &str) -> Option<f64> {
        let download = self.downloads.get(download_id)?;
        
        if let Some(total_bytes) = download.total_bytes {
            if total_bytes > 0 {
                return Some((download.downloaded_bytes as f64 / total_bytes as f64) * 100.0);
            }
        }
        
        None
    }

    pub fn export_downloads(&self) -> Result<String, String> {
        let downloads: Vec<&Download> = self.downloads.values().collect();
        serde_json::to_string_pretty(&downloads)
            .map_err(|e| format!("Failed to export downloads: {}", e))
    }
}

#[tauri::command]
pub async fn start_download(url: String, filename: Option<String>, referrer: Option<String>) -> Result<String, String> {
    let mut manager = DOWNLOAD_MANAGER.write().await;
    manager.start_download(&url, filename.as_deref(), referrer.as_deref())
}

#[tauri::command]
pub async fn cancel_download(download_id: String) -> Result<(), String> {
    let mut manager = DOWNLOAD_MANAGER.write().await;
    manager.cancel_download(&download_id)
}

#[tauri::command]
pub async fn pause_download(download_id: String) -> Result<(), String> {
    let mut manager = DOWNLOAD_MANAGER.write().await;
    manager.pause_download(&download_id)
}

#[tauri::command]
pub async fn resume_download(download_id: String) -> Result<(), String> {
    let mut manager = DOWNLOAD_MANAGER.write().await;
    manager.resume_download(&download_id)
}

#[tauri::command]
pub async fn remove_download(download_id: String) -> Result<(), String> {
    let mut manager = DOWNLOAD_MANAGER.write().await;
    manager.remove_download(&download_id)
}

#[tauri::command]
pub async fn clear_completed_downloads() -> Result<(), String> {
    let mut manager = DOWNLOAD_MANAGER.write().await;
    manager.clear_completed_downloads();
    Ok(())
}

#[tauri::command]
pub async fn get_downloads() -> Result<Vec<Download>, String> {
    let manager = DOWNLOAD_MANAGER.read().await;
    Ok(manager.get_downloads().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_active_downloads() -> Result<Vec<Download>, String> {
    let manager = DOWNLOAD_MANAGER.read().await;
    Ok(manager.get_active_downloads().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_download_stats() -> Result<DownloadStats, String> {
    let manager = DOWNLOAD_MANAGER.read().await;
    Ok(manager.get_download_stats())
}

#[tauri::command]
pub async fn set_download_directory(path: String) -> Result<(), String> {
    let mut manager = DOWNLOAD_MANAGER.write().await;
    manager.set_download_directory(PathBuf::from(path))
}

#[tauri::command]
pub async fn get_download_progress(download_id: String) -> Result<Option<f64>, String> {
    let manager = DOWNLOAD_MANAGER.read().await;
    Ok(manager.get_download_progress(&download_id))
}

#[tauri::command]
pub async fn export_downloads() -> Result<String, String> {
    let manager = DOWNLOAD_MANAGER.read().await;
    manager.export_downloads()
}