import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ScrollPosition {
  x: number
  y: number
}

export interface WindowBounds {
  x: number
  y: number
  width: number
  height: number
}

export interface SessionHistoryEntry {
  url: string
  title: string
  timestamp: string
}

export interface TabSession {
  id: string
  url: string
  title: string
  is_active: boolean
  is_pinned: boolean
  is_muted: boolean
  zoom_level: number
  scroll_position: ScrollPosition
  history: SessionHistoryEntry[]
  history_index: number
  favicon?: string
  created_at: string
  last_accessed: string
}

export interface WindowSession {
  id: string
  is_private: boolean
  bounds: WindowBounds
  active_tab_id?: string
  tabs: TabSession[]
  created_at: string
  last_accessed: string
}

export interface SessionData {
  id: string
  name: string
  windows: WindowSession[]
  created_at: string
  last_saved: string
  auto_save: boolean
}

export const useSessionsStore = defineStore('sessions', () => {
  const sessions = ref<SessionData[]>([])
  const currentSession = ref<SessionData | null>(null)
  const isLoading = ref(false)
  const autoSaveEnabled = ref(false)

  async function loadSessions(): Promise<void> {
    try {
      isLoading.value = true
      const sessionList = await invoke<SessionData[]>('get_saved_sessions')
      sessions.value = sessionList
    } catch (error) {
      console.error('Failed to load sessions:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function createSession(name: string): Promise<string> {
    try {
      isLoading.value = true
      const sessionId = await invoke<string>('create_session', { name })
      await loadSessions()
      return sessionId
    } catch (error) {
      console.error('Failed to create session:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function saveCurrentSession(name?: string): Promise<string> {
    try {
      isLoading.value = true
      const sessionId = await invoke<string>('save_current_session', { name })
      await loadSessions()
      return sessionId
    } catch (error) {
      console.error('Failed to save current session:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function restoreSession(sessionId: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('restore_session', { sessionId })
      const session = sessions.value.find(s => s.id === sessionId)
      if (session) {
        currentSession.value = session
      }
    } catch (error) {
      console.error('Failed to restore session:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function deleteSession(sessionId: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('delete_session', { sessionId })
      sessions.value = sessions.value.filter(session => session.id !== sessionId)
      if (currentSession.value?.id === sessionId) {
        currentSession.value = null
      }
    } catch (error) {
      console.error('Failed to delete session:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function addWindowToSession(sessionId: string, windowId: string): Promise<void> {
    try {
      await invoke('add_window_to_session', { sessionId, windowId })
      await loadSessions()
    } catch (error) {
      console.error('Failed to add window to session:', error)
      throw error
    }
  }

  async function removeWindowFromSession(sessionId: string, windowId: string): Promise<void> {
    try {
      await invoke('remove_window_from_session', { sessionId, windowId })
      await loadSessions()
    } catch (error) {
      console.error('Failed to remove window from session:', error)
      throw error
    }
  }

  async function updateWindowInSession(sessionId: string, windowId: string, bounds: WindowBounds): Promise<void> {
    try {
      await invoke('update_window_in_session', { sessionId, windowId, bounds })
      const session = sessions.value.find(s => s.id === sessionId)
      if (session) {
        const window = session.windows.find(w => w.id === windowId)
        if (window) {
          window.bounds = bounds
        }
      }
    } catch (error) {
      console.error('Failed to update window in session:', error)
      throw error
    }
  }

  async function addTabToWindow(sessionId: string, windowId: string, tabId: string): Promise<void> {
    try {
      await invoke('add_tab_to_window', { sessionId, windowId, tabId })
      await loadSessions()
    } catch (error) {
      console.error('Failed to add tab to window:', error)
      throw error
    }
  }

  async function removeTabFromWindow(sessionId: string, windowId: string, tabId: string): Promise<void> {
    try {
      await invoke('remove_tab_from_window', { sessionId, windowId, tabId })
      await loadSessions()
    } catch (error) {
      console.error('Failed to remove tab from window:', error)
      throw error
    }
  }

  async function updateTabInWindow(sessionId: string, windowId: string, tabId: string, tabData: Partial<TabSession>): Promise<void> {
    try {
      await invoke('update_tab_in_window', { sessionId, windowId, tabId, ...tabData })
      const session = sessions.value.find(s => s.id === sessionId)
      if (session) {
        const window = session.windows.find(w => w.id === windowId)
        if (window) {
          const tab = window.tabs.find(t => t.id === tabId)
          if (tab) {
            Object.assign(tab, tabData)
          }
        }
      }
    } catch (error) {
      console.error('Failed to update tab in window:', error)
      throw error
    }
  }

  async function updateTabScrollPosition(sessionId: string, windowId: string, tabId: string, scrollPosition: ScrollPosition): Promise<void> {
    try {
      await invoke('update_tab_scroll_position', { sessionId, windowId, tabId, scrollPosition })
      const session = sessions.value.find(s => s.id === sessionId)
      if (session) {
        const window = session.windows.find(w => w.id === windowId)
        if (window) {
          const tab = window.tabs.find(t => t.id === tabId)
          if (tab) {
            tab.scroll_position = scrollPosition
          }
        }
      }
    } catch (error) {
      console.error('Failed to update tab scroll position:', error)
      throw error
    }
  }

  async function setActiveTab(sessionId: string, windowId: string, tabId: string): Promise<void> {
    try {
      await invoke('set_active_tab', { sessionId, windowId, tabId })
      const session = sessions.value.find(s => s.id === sessionId)
      if (session) {
        const window = session.windows.find(w => w.id === windowId)
        if (window) {
          window.tabs.forEach(tab => {
            tab.is_active = tab.id === tabId
          })
          window.active_tab_id = tabId
        }
      }
    } catch (error) {
      console.error('Failed to set active tab:', error)
      throw error
    }
  }

  async function clearOldSessions(daysOld: number = 30): Promise<void> {
    try {
      isLoading.value = true
      await invoke('clear_old_sessions', { daysOld })
      await loadSessions()
    } catch (error) {
      console.error('Failed to clear old sessions:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function exportSession(sessionId: string): Promise<string> {
    try {
      const data = await invoke<string>('export_session', { sessionId })
      return data
    } catch (error) {
      console.error('Failed to export session:', error)
      throw error
    }
  }

  async function importSession(data: string): Promise<string> {
    try {
      isLoading.value = true
      const sessionId = await invoke<string>('import_session', { data })
      await loadSessions()
      return sessionId
    } catch (error) {
      console.error('Failed to import session:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function getCurrentSession(): Promise<SessionData | null> {
    try {
      const session = await invoke<SessionData>('get_current_session')
      currentSession.value = session
      return session
    } catch (error) {
      console.error('Failed to get current session:', error)
      return null
    }
  }

  async function enableAutoSave(enabled: boolean): Promise<void> {
    try {
      await invoke('enable_auto_save', { enabled })
      autoSaveEnabled.value = enabled
    } catch (error) {
      console.error('Failed to enable auto save:', error)
      throw error
    }
  }

  function getSessionById(sessionId: string): SessionData | undefined {
    return sessions.value.find(session => session.id === sessionId)
  }

  function getSessionsByDateRange(startDate: Date, endDate: Date): SessionData[] {
    return sessions.value.filter(session => {
      const sessionDate = new Date(session.created_at)
      return sessionDate >= startDate && sessionDate <= endDate
    })
  }

  function getRecentSessions(limit: number = 10): SessionData[] {
    return sessions.value
      .sort((a, b) => new Date(b.last_saved).getTime() - new Date(a.last_saved).getTime())
      .slice(0, limit)
  }

  function getSessionStats(): { totalSessions: number; totalWindows: number; totalTabs: number } {
    const totalSessions = sessions.value.length
    const totalWindows = sessions.value.reduce((sum, session) => sum + session.windows.length, 0)
    const totalTabs = sessions.value.reduce((sum, session) => 
      sum + session.windows.reduce((windowSum, window) => windowSum + window.tabs.length, 0), 0
    )
    
    return { totalSessions, totalWindows, totalTabs }
  }

  function formatSessionDate(session: SessionData): string {
    const date = new Date(session.created_at)
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString()
  }

  function getSessionTabCount(session: SessionData): number {
    return session.windows.reduce((sum, window) => sum + window.tabs.length, 0)
  }

  function getSessionWindowCount(session: SessionData): number {
    return session.windows.length
  }

  function isSessionActive(session: SessionData): boolean {
    return currentSession.value?.id === session.id
  }

  function getActiveTabInWindow(window: WindowSession): TabSession | undefined {
    return window.tabs.find(tab => tab.is_active) || window.tabs[0]
  }

  function getPinnedTabsInWindow(window: WindowSession): TabSession[] {
    return window.tabs.filter(tab => tab.is_pinned)
  }

  function getUnpinnedTabsInWindow(window: WindowSession): TabSession[] {
    return window.tabs.filter(tab => !tab.is_pinned)
  }

  return {
    sessions,
    currentSession,
    isLoading,
    autoSaveEnabled,
    loadSessions,
    createSession,
    saveCurrentSession,
    restoreSession,
    deleteSession,
    addWindowToSession,
    removeWindowFromSession,
    updateWindowInSession,
    addTabToWindow,
    removeTabFromWindow,
    updateTabInWindow,
    updateTabScrollPosition,
    setActiveTab,
    clearOldSessions,
    exportSession,
    importSession,
    getCurrentSession,
    enableAutoSave,
    getSessionById,
    getSessionsByDateRange,
    getRecentSessions,
    getSessionStats,
    formatSessionDate,
    getSessionTabCount,
    getSessionWindowCount,
    isSessionActive,
    getActiveTabInWindow,
    getPinnedTabsInWindow,
    getUnpinnedTabsInWindow
  }
})