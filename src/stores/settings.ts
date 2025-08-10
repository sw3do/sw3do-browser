import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface GeneralSettings {
  homepage: string
  new_tab_page: string
  default_search_engine: string
  restore_tabs_on_startup: boolean
  show_bookmarks_bar: boolean
  enable_notifications: boolean
  language: string
}

export interface PrivacySettings {
  block_ads: boolean
  block_trackers: boolean
  block_third_party_cookies: boolean
  enable_fingerprinting_protection: boolean
  https_only_mode: boolean
  clear_data_on_exit: boolean
  send_do_not_track: boolean
  enable_private_browsing_by_default: boolean
}

export interface AppearanceSettings {
  theme: string
  font_family: string
  font_size: number
  zoom_level: number
  show_tab_previews: boolean
  compact_mode: boolean
  custom_css?: string
}

export interface SearchSettings {
  search_engines: Record<string, SearchEngine>
  default_engine: string
  enable_search_suggestions: boolean
  show_search_in_address_bar: boolean
}

export interface SearchEngine {
  name: string
  url: string
  suggest_url?: string
  icon?: string
}

export interface DownloadSettings {
  download_directory: string
  ask_where_to_save: boolean
  auto_open_downloads: boolean
  clear_downloads_on_exit: boolean
}

export interface AdvancedSettings {
  enable_javascript: boolean
  enable_images: boolean
  enable_plugins: boolean
  enable_webgl: boolean
  enable_webrtc: boolean
  user_agent?: string
  proxy_settings: ProxySettings
  developer_mode: boolean
}

export interface ProxySettings {
  proxy_type: 'None' | 'Http' | 'Https' | 'Socks4' | 'Socks5'
  host?: string
  port?: number
  username?: string
  password?: string
}

export interface BrowserSettings {
  general: GeneralSettings
  privacy: PrivacySettings
  appearance: AppearanceSettings
  search: SearchSettings
  downloads: DownloadSettings
  advanced: AdvancedSettings
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

  async function addSearchEngine(id: string, searchEngine: Omit<SearchEngine, 'icon'>): Promise<void> {
    try {
      isLoading.value = true
      await invoke('add_search_engine', {
        id,
        engine: {
          name: searchEngine.name,
          url: searchEngine.url,
          suggest_url: searchEngine.suggest_url,
          icon: null
        }
      })
      
      await loadSettings()
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
    if (!settings.value?.search?.search_engines) return null
    const engines = settings.value.search.search_engines
    const defaultEngineId = settings.value.search.default_engine
    return engines[defaultEngineId] || null
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