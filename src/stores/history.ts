import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface HistoryEntry {
  id: string
  url: string
  title: string
  visit_count: number
  last_visit: string
  favicon?: string
  created_at: string
}

export interface HistoryStats {
  total_visits: number
  unique_sites: number
  today_visits: number
  this_week_visits: number
  this_month_visits: number
}

export const useHistoryStore = defineStore('history', () => {
  const history = ref<HistoryEntry[]>([])
  const isLoading = ref(false)

  async function loadRecentHistory(limit = 50): Promise<void> {
    try {
      isLoading.value = true
      const entries = await invoke<HistoryEntry[]>('get_recent_history', { limit })
      history.value = entries
    } catch (error) {
      console.error('Failed to load recent history:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function addVisit(url: string, title: string): Promise<void> {
    try {
      await invoke('add_history_visit', { url, title })
    } catch (error) {
      console.error('Failed to add history visit:', error)
      throw error
    }
  }

  async function removeEntry(entryId: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('remove_history_entry', { entryId })
      history.value = history.value.filter(entry => entry.id !== entryId)
    } catch (error) {
      console.error('Failed to remove history entry:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function clearHistory(): Promise<void> {
    try {
      isLoading.value = true
      await invoke('clear_history')
      history.value = []
    } catch (error) {
      console.error('Failed to clear history:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function searchHistory(query: string, limit = 20): Promise<HistoryEntry[]> {
    try {
      const results = await invoke<HistoryEntry[]>('search_history', { query, limit })
      return results
    } catch (error) {
      console.error('Failed to search history:', error)
      return []
    }
  }

  async function getMostVisited(limit = 10): Promise<HistoryEntry[]> {
    try {
      const results = await invoke<HistoryEntry[]>('get_most_visited', { limit })
      return results
    } catch (error) {
      console.error('Failed to get most visited:', error)
      return []
    }
  }

  async function getHistoryByDate(date: string): Promise<HistoryEntry[]> {
    try {
      const results = await invoke<HistoryEntry[]>('get_history_by_date', { date })
      return results
    } catch (error) {
      console.error('Failed to get history by date:', error)
      return []
    }
  }

  async function getHistoryStats(): Promise<HistoryStats> {
    try {
      const stats = await invoke<HistoryStats>('get_history_stats')
      return stats
    } catch (error) {
      console.error('Failed to get history stats:', error)
      return {
        total_visits: 0,
        unique_sites: 0,
        today_visits: 0,
        this_week_visits: 0,
        this_month_visits: 0
      }
    }
  }

  async function getSuggestions(query: string, limit = 5): Promise<HistoryEntry[]> {
    try {
      const suggestions = await invoke<HistoryEntry[]>('get_history_suggestions', { query, limit })
      return suggestions
    } catch (error) {
      console.error('Failed to get history suggestions:', error)
      return []
    }
  }

  async function updateFavicon(url: string, favicon: string): Promise<void> {
    try {
      await invoke('update_history_favicon', { url, favicon })
      
      const entry = history.value.find(h => h.url === url)
      if (entry) {
        entry.favicon = favicon
      }
    } catch (error) {
      console.error('Failed to update favicon:', error)
      throw error
    }
  }

  async function exportHistory(): Promise<string> {
    try {
      const data = await invoke<string>('export_history')
      return data
    } catch (error) {
      console.error('Failed to export history:', error)
      throw error
    }
  }

  async function importHistory(data: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('import_history', { data })
      await loadRecentHistory()
    } catch (error) {
      console.error('Failed to import history:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  function getEntryById(entryId: string): HistoryEntry | undefined {
    return history.value.find(entry => entry.id === entryId)
  }

  function getEntriesByDomain(domain: string): HistoryEntry[] {
    return history.value.filter(entry => {
      try {
        const url = new URL(entry.url)
        return url.hostname === domain
      } catch {
        return false
      }
    })
  }

  function groupByDate(): Record<string, HistoryEntry[]> {
    const grouped: Record<string, HistoryEntry[]> = {}
    
    for (const entry of history.value) {
      const date = new Date(entry.last_visit).toDateString()
      if (!grouped[date]) {
        grouped[date] = []
      }
      grouped[date].push(entry)
    }
    
    return grouped
  }

  return {
    history,
    isLoading,
    loadRecentHistory,
    addVisit,
    removeEntry,
    clearHistory,
    searchHistory,
    getMostVisited,
    getHistoryByDate,
    getHistoryStats,
    getSuggestions,
    updateFavicon,
    exportHistory,
    importHistory,
    getEntryById,
    getEntriesByDomain,
    groupByDate
  }
})