import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface Tab {
  id: string
  window_id: string
  url: string
  title: string
  favicon?: string
  is_loading: boolean
  is_pinned: boolean
  is_muted: boolean
  is_private: boolean
  zoom_level: number
  can_go_back: boolean
  can_go_forward: boolean
  created_at: string
  last_accessed: string
}

export interface BrowserWindow {
  id: string
  is_private: boolean
  tabs: string[]
  active_tab_id?: string
  created_at: string
}

export const useBrowserStore = defineStore('browser', () => {
  const windows = ref<Map<string, BrowserWindow>>(new Map())
  const tabs = ref<Map<string, Tab>>(new Map())
  const activeWindowId = ref<string | null>(null)
  const isLoading = ref(false)

  const activeWindow = computed(() => {
    if (!activeWindowId.value) return null
    return windows.value.get(activeWindowId.value) || null
  })

  const activeTab = computed(() => {
    if (!activeWindow.value?.active_tab_id) return null
    return tabs.value.get(activeWindow.value.active_tab_id) || null
  })

  const windowTabs = computed(() => {
    return (windowId: string) => {
      const window = windows.value.get(windowId)
      if (!window) return []
      return window.tabs.map(tabId => tabs.value.get(tabId)).filter(Boolean) as Tab[]
    }
  })

  const totalTabs = computed(() => tabs.value.size)
  const privateTabs = computed(() => {
    return Array.from(tabs.value.values()).filter(tab => tab.is_private).length
  })
  const pinnedTabs = computed(() => {
    return Array.from(tabs.value.values()).filter(tab => tab.is_pinned).length
  })

  async function createWindow(isPrivate = false): Promise<string> {
    try {
      isLoading.value = true
      const windowId = await invoke<string>('create_browser_window', { isPrivate })
      
      const newWindow: BrowserWindow = {
        id: windowId,
        is_private: isPrivate,
        tabs: [],
        created_at: new Date().toISOString()
      }
      
      windows.value.set(windowId, newWindow)
      
      if (!activeWindowId.value) {
        activeWindowId.value = windowId
      }
      
      await syncWindowState()
      
      return windowId
    } catch (error) {
      console.error('Failed to create window:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function closeWindow(windowId: string): Promise<void> {
    try {
      isLoading.value = true
      
      const window = windows.value.get(windowId)
      if (window) {
        for (const tabId of window.tabs) {
          tabs.value.delete(tabId)
        }
      }
      
      await invoke('close_browser_window', { windowId })
      
      windows.value.delete(windowId)
      
      if (activeWindowId.value === windowId) {
        const remainingWindows = Array.from(windows.value.keys())
        activeWindowId.value = remainingWindows.length > 0 ? remainingWindows[0] : null
      }
      
      await syncWindowState()
    } catch (error) {
      console.error('Failed to close window:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function createTab(windowId: string, url: string, isPrivate = false): Promise<string> {
    try {
      isLoading.value = true
      const tabId = await invoke<string>('create_tab', { windowId, url, isPrivate })
      
      const newTab: Tab = {
        id: tabId,
        window_id: windowId,
        url,
        title: url,
        is_loading: true,
        is_pinned: false,
        is_muted: false,
        is_private: isPrivate,
        zoom_level: 1.0,
        can_go_back: false,
        can_go_forward: false,
        created_at: new Date().toISOString(),
        last_accessed: new Date().toISOString()
      }
      
      tabs.value.set(tabId, newTab)
      
      const window = windows.value.get(windowId)
      if (window) {
        window.tabs.push(tabId)
        if (!window.active_tab_id) {
          window.active_tab_id = tabId
        }
      }
      
      return tabId
    } catch (error) {
      console.error('Failed to create tab:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function closeTab(tabId: string): Promise<void> {
    try {
      isLoading.value = true
      
      try {
        await invoke('close_webview_tab', { tabId })
      } catch (webviewError) {
        console.warn('Failed to close webview for tab:', webviewError)
      }
      
      await invoke('close_tab', { tabId })
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        const window = windows.value.get(tab.window_id)
        if (window) {
          window.tabs = window.tabs.filter(id => id !== tabId)
          
          if (window.active_tab_id === tabId) {
            window.active_tab_id = window.tabs.length > 0 ? window.tabs[0] : undefined
          }
        }
      }
      
      tabs.value.delete(tabId)
    } catch (error) {
      console.error('Failed to close tab:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function setActiveTab(windowId: string, tabId: string): Promise<void> {
    try {
      await invoke('set_active_tab', { windowId, tabId })
      
      const window = windows.value.get(windowId)
      if (window) {
        window.active_tab_id = tabId
      }
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        tab.last_accessed = new Date().toISOString()
      }
    } catch (error) {
      console.error('Failed to set active tab:', error)
      throw error
    }
  }

  async function updateTabUrl(tabId: string, url: string, title?: string): Promise<void> {
    try {
      await invoke('update_tab_url', { tabId, url, title })
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        tab.url = url
        if (title) {
          tab.title = title
        }
        tab.last_accessed = new Date().toISOString()
      }
    } catch (error) {
      console.error('Failed to update tab URL:', error)
      throw error
    }
  }

  async function duplicateTab(tabId: string): Promise<string> {
    try {
      isLoading.value = true
      const newTabId = await invoke<string>('duplicate_tab', { tabId })
      
      const originalTab = tabs.value.get(tabId)
      if (originalTab) {
        const duplicatedTab: Tab = {
          ...originalTab,
          id: newTabId,
          created_at: new Date().toISOString(),
          last_accessed: new Date().toISOString()
        }
        
        tabs.value.set(newTabId, duplicatedTab)
        
        const window = windows.value.get(originalTab.window_id)
        if (window) {
          const originalIndex = window.tabs.indexOf(tabId)
          window.tabs.splice(originalIndex + 1, 0, newTabId)
        }
      }
      
      return newTabId
    } catch (error) {
      console.error('Failed to duplicate tab:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function pinTab(tabId: string): Promise<void> {
    try {
      await invoke('pin_tab', { tabId })
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        tab.is_pinned = true
      }
    } catch (error) {
      console.error('Failed to pin tab:', error)
      throw error
    }
  }

  async function unpinTab(tabId: string): Promise<void> {
    try {
      await invoke('unpin_tab', { tabId })
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        tab.is_pinned = false
      }
    } catch (error) {
      console.error('Failed to unpin tab:', error)
      throw error
    }
  }

  async function muteTab(tabId: string): Promise<void> {
    try {
      await invoke('mute_tab', { tabId })
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        tab.is_muted = true
      }
    } catch (error) {
      console.error('Failed to mute tab:', error)
      throw error
    }
  }

  async function unmuteTab(tabId: string): Promise<void> {
    try {
      await invoke('unmute_tab', { tabId })
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        tab.is_muted = false
      }
    } catch (error) {
      console.error('Failed to unmute tab:', error)
      throw error
    }
  }

  async function reloadTab(tabId: string): Promise<void> {
    try {
      await invoke('reload_tab', { tabId })
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        tab.is_loading = true
      }
    } catch (error) {
      console.error('Failed to reload tab:', error)
      throw error
    }
  }

  async function goBack(tabId: string): Promise<void> {
    try {
      await invoke('go_back', { tabId })
    } catch (error) {
      console.error('Failed to go back:', error)
      throw error
    }
  }

  async function goForward(tabId: string): Promise<void> {
    try {
      await invoke('go_forward', { tabId })
    } catch (error) {
      console.error('Failed to go forward:', error)
      throw error
    }
  }

  async function zoomIn(tabId: string): Promise<number> {
    try {
      const newZoom = await invoke<number>('zoom_in', { tabId })
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        tab.zoom_level = newZoom
      }
      
      return newZoom
    } catch (error) {
      console.error('Failed to zoom in:', error)
      throw error
    }
  }

  async function zoomOut(tabId: string): Promise<number> {
    try {
      const newZoom = await invoke<number>('zoom_out', { tabId })
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        tab.zoom_level = newZoom
      }
      
      return newZoom
    } catch (error) {
      console.error('Failed to zoom out:', error)
      throw error
    }
  }

  async function resetZoom(tabId: string): Promise<number> {
    try {
      const newZoom = await invoke<number>('reset_zoom', { tabId })
      
      const tab = tabs.value.get(tabId)
      if (tab) {
        tab.zoom_level = newZoom
      }
      
      return newZoom
    } catch (error) {
      console.error('Failed to reset zoom:', error)
      throw error
    }
  }

  function setActiveWindow(windowId: string): void {
    activeWindowId.value = windowId
  }

  function updateTabState(tabId: string, updates: Partial<Tab>): void {
    const tab = tabs.value.get(tabId)
    if (tab) {
      Object.assign(tab, updates)
    }
  }

  async function syncWindowState(): Promise<void> {
    try {
      const backendWindows = await invoke<BrowserWindow[]>('get_all_windows')
      
      for (const backendWindow of backendWindows) {
        const existingWindow = windows.value.get(backendWindow.id)
        if (!existingWindow) {
          windows.value.set(backendWindow.id, backendWindow)
        } else {
          Object.assign(existingWindow, backendWindow)
        }
      }
      
      const backendWindowIds = new Set(backendWindows.map(w => w.id))
      for (const [windowId] of windows.value) {
        if (!backendWindowIds.has(windowId)) {
          windows.value.delete(windowId)
        }
      }
    } catch (error) {
      console.error('Failed to sync window state:', error)
    }
  }

  async function syncTabState(): Promise<void> {
    try {
      const backendTabs = await invoke<Tab[]>('get_all_tabs')
      
      for (const backendTab of backendTabs) {
        const existingTab = tabs.value.get(backendTab.id)
        if (!existingTab) {
          tabs.value.set(backendTab.id, backendTab)
        } else {
          Object.assign(existingTab, backendTab)
        }
      }
      
      const backendTabIds = new Set(backendTabs.map(t => t.id))
      for (const [tabId] of tabs.value) {
        if (!backendTabIds.has(tabId)) {
          tabs.value.delete(tabId)
        }
      }
    } catch (error) {
      console.error('Failed to sync tab state:', error)
    }
  }

  return {
    windows,
    tabs,
    activeWindowId,
    isLoading,
    activeWindow,
    activeTab,
    windowTabs,
    totalTabs,
    privateTabs,
    pinnedTabs,
    createWindow,
    closeWindow,
    createTab,
    closeTab,
    setActiveTab,
    updateTabUrl,
    duplicateTab,
    pinTab,
    unpinTab,
    muteTab,
    unmuteTab,
    reloadTab,
    goBack,
    goForward,
    zoomIn,
    zoomOut,
    resetZoom,
    setActiveWindow,
    updateTabState,
    syncWindowState,
    syncTabState
  }
})