import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface Bookmark {
  id: string
  title: string
  url: string
  favicon?: string
  folder_id?: string
  created_at: string
  updated_at: string
}

export interface BookmarkFolder {
  id: string
  name: string
  parent_id?: string
  created_at: string
  updated_at: string
}

export interface BookmarkTree {
  folders: BookmarkFolder[]
  bookmarks: Bookmark[]
}

export const useBookmarksStore = defineStore('bookmarks', () => {
  const bookmarks = ref<Bookmark[]>([])
  const folders = ref<BookmarkFolder[]>([])
  const isLoading = ref(false)

  async function loadBookmarks(): Promise<void> {
    try {
      isLoading.value = true
      const tree = await invoke<BookmarkTree>('get_bookmark_tree')
      bookmarks.value = tree.bookmarks
      folders.value = tree.folders
    } catch (error) {
      console.error('Failed to load bookmarks:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function addBookmark(bookmark: Omit<Bookmark, 'id' | 'created_at' | 'updated_at'>): Promise<string> {
    try {
      isLoading.value = true
      const bookmarkId = await invoke<string>('add_bookmark', {
        title: bookmark.title,
        url: bookmark.url,
        favicon: bookmark.favicon,
        folderId: bookmark.folder_id
      })
      
      await loadBookmarks()
      return bookmarkId
    } catch (error) {
      console.error('Failed to add bookmark:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function removeBookmark(bookmarkId: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('delete_bookmark', { bookmarkId })
      await loadBookmarks()
    } catch (error) {
      console.error('Failed to remove bookmark:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function removeBookmarkByUrl(url: string): Promise<void> {
    const bookmark = bookmarks.value.find(b => b.url === url)
    if (bookmark) {
      await removeBookmark(bookmark.id)
    }
  }

  async function updateBookmark(bookmarkId: string, updates: Partial<Bookmark>): Promise<void> {
    try {
      isLoading.value = true
      await invoke('update_bookmark', {
        bookmarkId,
        title: updates.title,
        url: updates.url,
        favicon: updates.favicon
      })
      await loadBookmarks()
    } catch (error) {
      console.error('Failed to update bookmark:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function createFolder(name: string, parentId?: string): Promise<string> {
    try {
      isLoading.value = true
      const folderId = await invoke<string>('create_bookmark_folder', {
        name,
        parentId
      })
      await loadBookmarks()
      return folderId
    } catch (error) {
      console.error('Failed to create folder:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function removeFolder(folderId: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('delete_bookmark_folder', { folderId })
      await loadBookmarks()
    } catch (error) {
      console.error('Failed to remove folder:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function searchBookmarks(query: string, limit = 10): Promise<Bookmark[]> {
    try {
      const results = await invoke<Bookmark[]>('search_bookmarks', { query, limit })
      return results
    } catch (error) {
      console.error('Failed to search bookmarks:', error)
      return []
    }
  }

  async function moveBookmark(bookmarkId: string, targetFolderId?: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('move_bookmark', {
        bookmarkId,
        targetFolderId
      })
      await loadBookmarks()
    } catch (error) {
      console.error('Failed to move bookmark:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function exportBookmarks(): Promise<string> {
    try {
      const data = await invoke<string>('export_bookmarks')
      return data
    } catch (error) {
      console.error('Failed to export bookmarks:', error)
      throw error
    }
  }

  async function importBookmarks(data: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('import_bookmarks', { data })
      await loadBookmarks()
    } catch (error) {
      console.error('Failed to import bookmarks:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  function getFolderContents(folderId?: string): { bookmarks: Bookmark[], folders: BookmarkFolder[] } {
    const folderBookmarks = bookmarks.value.filter(b => b.folder_id === folderId)
    const subFolders = folders.value.filter(f => f.parent_id === folderId)
    
    return {
      bookmarks: folderBookmarks,
      folders: subFolders
    }
  }

  function getBookmarkById(bookmarkId: string): Bookmark | undefined {
    return bookmarks.value.find(b => b.id === bookmarkId)
  }

  function getFolderById(folderId: string): BookmarkFolder | undefined {
    return folders.value.find(f => f.id === folderId)
  }

  return {
    bookmarks,
    folders,
    isLoading,
    loadBookmarks,
    addBookmark,
    removeBookmark,
    removeBookmarkByUrl,
    updateBookmark,
    createFolder,
    removeFolder,
    searchBookmarks,
    moveBookmark,
    exportBookmarks,
    importBookmarks,
    getFolderContents,
    getBookmarkById,
    getFolderById
  }
})