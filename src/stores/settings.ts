import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface GeneralSettings {
  homepage: string
  new_tab_page: string
  search_engine: string
  download_directory: string
  ask_where_to_save: boolean
  restore_tabs_on_startup: boolean
  show_bookmarks_bar: boolean
  show_home_button: boolean
}

export interface PrivacySettings {
  clear_browsing_data_on_exit: boolean
  send_do_not_track: boolean
  block_third_party_cookies: boolean
  enable_safe_browsing: boolean
  use_secure_dns: boolean
  dns_provider: string
}

export interface AppearanceSettings {
  theme: 'light' | 'dark' | 'system'
  font_size: number
  font_family: string
  show_tab_previews: boolean
  compact_mode: boolean
  show_sidebar: boolean
}

export interface SearchEngine {
  id: string
  name: string
  search_url: string
  suggestion_url?: string
  is_default: boolean
}

export interface BrowserSettings {
  general: GeneralSettings
  privacy: PrivacySettings
  appearance: AppearanceSettings
  search_engines: SearchEngine[]
}

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<BrowserSettings | null>(null)
  const isLoading = ref(false)

  async function loadSettings(): Promise<void> {
    try {
      isLoading.value = true
      const browserSettings = await invoke<BrowserSettings>('get_settings')
      settings.value = browserSettings
    } catch (error) {
      console.error('Failed to load settings:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function updateGeneralSettings(updates: Partial<GeneralSettings>): Promise<void> {
    try {
      isLoading.value = true
      await invoke('update_general_settings', updates)
      if (settings.value) {
        settings.value.general = { ...settings.value.general, ...updates }
      }
    } catch (error) {
      console.error('Failed to update general settings:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function updatePrivacySettings(updates: Partial<PrivacySettings>): Promise<void> {
    try {
      isLoading.value = true
      await invoke('update_privacy_settings', updates)
      if (settings.value) {
        settings.value.privacy = { ...settings.value.privacy, ...updates }
      }
    } catch (error) {
      console.error('Failed to update privacy settings:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function updateAppearanceSettings(updates: Partial<AppearanceSettings>): Promise<void> {
    try {
      isLoading.value = true
      await invoke('update_appearance_settings', updates)
      if (settings.value) {
        settings.value.appearance = { ...settings.value.appearance, ...updates }
      }
    } catch (error) {
      console.error('Failed to update appearance settings:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function addSearchEngine(searchEngine: Omit<SearchEngine, 'id' | 'is_default'>): Promise<string> {
    try {
      isLoading.value = true
      const engineId = await invoke<string>('add_search_engine', {
        name: searchEngine.name,
        searchUrl: searchEngine.search_url,
        suggestionUrl: searchEngine.suggestion_url
      })
      
      await loadSettings()
      return engineId
    } catch (error) {
      console.error('Failed to add search engine:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function removeSearchEngine(engineId: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('remove_search_engine', { engineId })
      await loadSettings()
    } catch (error) {
      console.error('Failed to remove search engine:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function setDefaultSearchEngine(engineId: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('set_default_search_engine', { engineId })
      await loadSettings()
    } catch (error) {
      console.error('Failed to set default search engine:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function resetToDefaults(): Promise<void> {
    try {
      isLoading.value = true
      await invoke('reset_settings_to_defaults')
      await loadSettings()
    } catch (error) {
      console.error('Failed to reset settings:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function exportSettings(): Promise<string> {
    try {
      const data = await invoke<string>('export_settings')
      return data
    } catch (error) {
      console.error('Failed to export settings:', error)
      throw error
    }
  }

  async function importSettings(data: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('import_settings', { data })
      await loadSettings()
    } catch (error) {
      console.error('Failed to import settings:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function getSearchUrl(query: string): Promise<string> {
    try {
      const url = await invoke<string>('get_search_url', { query })
      return url
    } catch (error) {
      console.error('Failed to get search URL:', error)
      return `https://www.google.com/search?q=${encodeURIComponent(query)}`
    }
  }

  async function getSuggestionUrl(query: string): Promise<string | null> {
    try {
      const url = await invoke<string>('get_suggestion_url', { query })
      return url
    } catch (error) {
      console.error('Failed to get suggestion URL:', error)
      return null
    }
  }

  function getDefaultSearchEngine(): SearchEngine | null {
    if (!settings.value) return null
    return settings.value.search_engines.find(engine => engine.is_default) || null
  }

  function applyTheme(): void {
    if (!settings.value) return
    
    const { theme } = settings.value.appearance
    const root = document.documentElement
    
    if (theme === 'dark') {
      root.classList.add('dark')
    } else if (theme === 'light') {
      root.classList.remove('dark')
    } else {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      if (prefersDark) {
        root.classList.add('dark')
      } else {
        root.classList.remove('dark')
      }
    }
  }

  return {
    settings,
    isLoading,
    loadSettings,
    updateGeneralSettings,
    updatePrivacySettings,
    updateAppearanceSettings,
    addSearchEngine,
    removeSearchEngine,
    setDefaultSearchEngine,
    resetToDefaults,
    exportSettings,
    importSettings,
    getSearchUrl,
    getSuggestionUrl,
    getDefaultSearchEngine,
    applyTheme
  }
})