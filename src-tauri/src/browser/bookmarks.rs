use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub title: String,
    pub url: String,
    pub favicon: Option<String>,
    pub folder_id: Option<String>,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: Option<chrono::DateTime<chrono::Utc>>,
    pub visit_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkFolder {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub children: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkTree {
    pub folders: HashMap<String, BookmarkFolder>,
    pub bookmarks: HashMap<String, Bookmark>,
    pub root_folder_id: String,
}

static BOOKMARK_MANAGER: Lazy<RwLock<BookmarkManager>> = Lazy::new(|| {
    RwLock::new(BookmarkManager::new())
});

pub struct BookmarkManager {
    pub tree: BookmarkTree,
}

impl BookmarkManager {
    pub fn new() -> Self {
        let root_folder_id = Uuid::new_v4().to_string();
        let mut folders = HashMap::new();
        
        let root_folder = BookmarkFolder {
            id: root_folder_id.clone(),
            name: "Bookmarks".to_string(),
            parent_id: None,
            created_at: chrono::Utc::now(),
            children: Vec::new(),
        };
        
        folders.insert(root_folder_id.clone(), root_folder);
        
        let tree = BookmarkTree {
            folders,
            bookmarks: HashMap::new(),
            root_folder_id,
        };
        
        Self { tree }
    }

    pub fn add_bookmark(&mut self, title: &str, url: &str, folder_id: Option<&str>) -> Result<String, String> {
        let bookmark_id = Uuid::new_v4().to_string();
        let folder_id = folder_id.map(|s| s.to_string());
        
        if let Some(ref fid) = folder_id {
            if !self.tree.folders.contains_key(fid) {
                return Err("Folder not found".to_string());
            }
        }
        
        let bookmark = Bookmark {
            id: bookmark_id.clone(),
            title: title.to_string(),
            url: url.to_string(),
            favicon: None,
            folder_id: folder_id.clone(),
            tags: Vec::new(),
            created_at: chrono::Utc::now(),
            last_accessed: None,
            visit_count: 0,
        };
        
        self.tree.bookmarks.insert(bookmark_id.clone(), bookmark);
        
        let target_folder_id = folder_id.unwrap_or_else(|| self.tree.root_folder_id.clone());
        if let Some(folder) = self.tree.folders.get_mut(&target_folder_id) {
            folder.children.push(bookmark_id.clone());
        }
        
        Ok(bookmark_id)
    }

    pub fn create_folder(&mut self, name: &str, parent_id: Option<&str>) -> Result<String, String> {
        let folder_id = Uuid::new_v4().to_string();
        let parent_id = parent_id.map(|s| s.to_string());
        
        if let Some(ref pid) = parent_id {
            if !self.tree.folders.contains_key(pid) {
                return Err("Parent folder not found".to_string());
            }
        }
        
        let folder = BookmarkFolder {
            id: folder_id.clone(),
            name: name.to_string(),
            parent_id: parent_id.clone(),
            created_at: chrono::Utc::now(),
            children: Vec::new(),
        };
        
        self.tree.folders.insert(folder_id.clone(), folder);
        
        let target_parent_id = parent_id.unwrap_or_else(|| self.tree.root_folder_id.clone());
        if let Some(parent_folder) = self.tree.folders.get_mut(&target_parent_id) {
            parent_folder.children.push(folder_id.clone());
        }
        
        Ok(folder_id)
    }

    pub fn delete_bookmark(&mut self, bookmark_id: &str) -> Result<(), String> {
        let bookmark = self.tree.bookmarks.get(bookmark_id).ok_or("Bookmark not found")?;
        let folder_id = bookmark.folder_id.clone().unwrap_or_else(|| self.tree.root_folder_id.clone());
        
        self.tree.bookmarks.remove(bookmark_id);
        
        if let Some(folder) = self.tree.folders.get_mut(&folder_id) {
            folder.children.retain(|id| id != bookmark_id);
        }
        
        Ok(())
    }

    pub fn delete_folder(&mut self, folder_id: &str) -> Result<(), String> {
        if folder_id == self.tree.root_folder_id {
            return Err("Cannot delete root folder".to_string());
        }
        
        let folder = self.tree.folders.get(folder_id).ok_or("Folder not found")?;
        let parent_id = folder.parent_id.clone();
        let children = folder.children.clone();
        
        for child_id in children {
            if self.tree.bookmarks.contains_key(&child_id) {
                self.delete_bookmark(&child_id)?;
            } else if self.tree.folders.contains_key(&child_id) {
                self.delete_folder(&child_id)?;
            }
        }
        
        self.tree.folders.remove(folder_id);
        
        if let Some(parent_id) = parent_id {
            if let Some(parent_folder) = self.tree.folders.get_mut(&parent_id) {
                parent_folder.children.retain(|id| id != folder_id);
            }
        }
        
        Ok(())
    }

    pub fn update_bookmark(&mut self, bookmark_id: &str, title: Option<&str>, url: Option<&str>) -> Result<(), String> {
        let bookmark = self.tree.bookmarks.get_mut(bookmark_id).ok_or("Bookmark not found")?;
        
        if let Some(title) = title {
            bookmark.title = title.to_string();
        }
        
        if let Some(url) = url {
            bookmark.url = url.to_string();
        }
        
        Ok(())
    }

    pub fn move_bookmark(&mut self, bookmark_id: &str, new_folder_id: &str) -> Result<(), String> {
        if !self.tree.folders.contains_key(new_folder_id) {
            return Err("Target folder not found".to_string());
        }
        
        let bookmark = self.tree.bookmarks.get_mut(bookmark_id).ok_or("Bookmark not found")?;
        let old_folder_id = bookmark.folder_id.clone().unwrap_or_else(|| self.tree.root_folder_id.clone());
        
        if let Some(old_folder) = self.tree.folders.get_mut(&old_folder_id) {
            old_folder.children.retain(|id| id != bookmark_id);
        }
        
        bookmark.folder_id = Some(new_folder_id.to_string());
        
        if let Some(new_folder) = self.tree.folders.get_mut(new_folder_id) {
            new_folder.children.push(bookmark_id.to_string());
        }
        
        Ok(())
    }

    pub fn search_bookmarks(&self, query: &str) -> Vec<&Bookmark> {
        let query = query.to_lowercase();
        self.tree.bookmarks.values()
            .filter(|bookmark| {
                bookmark.title.to_lowercase().contains(&query) ||
                bookmark.url.to_lowercase().contains(&query) ||
                bookmark.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .collect()
    }

    pub fn get_folder_contents(&self, folder_id: &str) -> Result<(Vec<&BookmarkFolder>, Vec<&Bookmark>), String> {
        let folder = self.tree.folders.get(folder_id).ok_or("Folder not found")?;
        
        let mut subfolders = Vec::new();
        let mut bookmarks = Vec::new();
        
        for child_id in &folder.children {
            if let Some(subfolder) = self.tree.folders.get(child_id) {
                subfolders.push(subfolder);
            } else if let Some(bookmark) = self.tree.bookmarks.get(child_id) {
                bookmarks.push(bookmark);
            }
        }
        
        Ok((subfolders, bookmarks))
    }

    pub fn export_bookmarks(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.tree)
            .map_err(|e| format!("Failed to export bookmarks: {}", e))
    }

    pub fn import_bookmarks(&mut self, data: &str) -> Result<(), String> {
        let imported_tree: BookmarkTree = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse bookmark data: {}", e))?;
        
        self.tree = imported_tree;
        Ok(())
    }
}

#[tauri::command]
pub async fn add_bookmark(title: String, url: String, folder_id: Option<String>) -> Result<String, String> {
    let mut manager = BOOKMARK_MANAGER.write().await;
    manager.add_bookmark(&title, &url, folder_id.as_deref())
}

#[tauri::command]
pub async fn create_bookmark_folder(name: String, parent_id: Option<String>) -> Result<String, String> {
    let mut manager = BOOKMARK_MANAGER.write().await;
    manager.create_folder(&name, parent_id.as_deref())
}

#[tauri::command]
pub async fn delete_bookmark(bookmark_id: String) -> Result<(), String> {
    let mut manager = BOOKMARK_MANAGER.write().await;
    manager.delete_bookmark(&bookmark_id)
}

#[tauri::command]
pub async fn delete_bookmark_folder(folder_id: String) -> Result<(), String> {
    let mut manager = BOOKMARK_MANAGER.write().await;
    manager.delete_folder(&folder_id)
}

#[tauri::command]
pub async fn update_bookmark(bookmark_id: String, title: Option<String>, url: Option<String>) -> Result<(), String> {
    let mut manager = BOOKMARK_MANAGER.write().await;
    manager.update_bookmark(&bookmark_id, title.as_deref(), url.as_deref())
}

#[tauri::command]
pub async fn move_bookmark(bookmark_id: String, new_folder_id: String) -> Result<(), String> {
    let mut manager = BOOKMARK_MANAGER.write().await;
    manager.move_bookmark(&bookmark_id, &new_folder_id)
}

#[tauri::command]
pub async fn search_bookmarks(query: String) -> Result<Vec<Bookmark>, String> {
    let manager = BOOKMARK_MANAGER.read().await;
    Ok(manager.search_bookmarks(&query).into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_bookmark_tree() -> Result<BookmarkTree, String> {
    let manager = BOOKMARK_MANAGER.read().await;
    Ok(manager.tree.clone())
}

#[tauri::command]
pub async fn get_folder_contents(folder_id: String) -> Result<(Vec<BookmarkFolder>, Vec<Bookmark>), String> {
    let manager = BOOKMARK_MANAGER.read().await;
    let (folders, bookmarks) = manager.get_folder_contents(&folder_id)?;
    Ok((folders.into_iter().cloned().collect(), bookmarks.into_iter().cloned().collect()))
}

#[tauri::command]
pub async fn export_bookmarks() -> Result<String, String> {
    let manager = BOOKMARK_MANAGER.read().await;
    manager.export_bookmarks()
}

#[tauri::command]
pub async fn import_bookmarks(data: String) -> Result<(), String> {
    let mut manager = BOOKMARK_MANAGER.write().await;
    manager.import_bookmarks(&data)
}