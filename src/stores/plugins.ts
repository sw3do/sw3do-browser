import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export type PluginPermission = 'storage' | 'network' | 'filesystem' | 'notifications' | 'tabs' | 'bookmarks' | 'history' | 'downloads'
export type PluginHook = 'page_load' | 'tab_created' | 'tab_closed' | 'bookmark_added' | 'download_started' | 'url_changed'

export interface PluginSetting {
  key: string
  value: any
  type: 'string' | 'number' | 'boolean' | 'array' | 'object'
  description?: string
}

export interface PluginManifest {
  name: string
  version: string
  description: string
  author: string
  permissions: PluginPermission[]
  hooks: PluginHook[]
  settings: PluginSetting[]
  icon?: string
  homepage?: string
}

export interface Plugin {
  id: string
  manifest: PluginManifest
  enabled: boolean
  installed_at: string
  last_updated: string
  file_path: string
  settings: Record<string, any>
}

export interface PluginEvent {
  hook: PluginHook
  data: any
  timestamp: string
}

export interface PluginStats {
  total_plugins: number
  enabled_plugins: number
  disabled_plugins: number
  events_processed: number
  last_event_time?: string
}

export const usePluginsStore = defineStore('plugins', () => {
  const plugins = ref<Plugin[]>([])
  const isLoading = ref(false)

  async function loadPlugins(): Promise<void> {
    try {
      isLoading.value = true
      const pluginList = await invoke<Plugin[]>('get_all_plugins')
      plugins.value = pluginList
    } catch (error) {
      console.error('Failed to load plugins:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function installPlugin(filePath: string): Promise<string> {
    try {
      isLoading.value = true
      const pluginId = await invoke<string>('install_plugin', { filePath })
      await loadPlugins()
      return pluginId
    } catch (error) {
      console.error('Failed to install plugin:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function uninstallPlugin(pluginId: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('uninstall_plugin', { pluginId })
      plugins.value = plugins.value.filter(plugin => plugin.id !== pluginId)
    } catch (error) {
      console.error('Failed to uninstall plugin:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function enablePlugin(pluginId: string): Promise<void> {
    try {
      await invoke('enable_plugin', { pluginId })
      const plugin = plugins.value.find(p => p.id === pluginId)
      if (plugin) {
        plugin.enabled = true
      }
    } catch (error) {
      console.error('Failed to enable plugin:', error)
      throw error
    }
  }

  async function disablePlugin(pluginId: string): Promise<void> {
    try {
      await invoke('disable_plugin', { pluginId })
      const plugin = plugins.value.find(p => p.id === pluginId)
      if (plugin) {
        plugin.enabled = false
      }
    } catch (error) {
      console.error('Failed to disable plugin:', error)
      throw error
    }
  }

  async function getPlugin(pluginId: string): Promise<Plugin | null> {
    try {
      const plugin = await invoke<Plugin>('get_plugin', { pluginId })
      return plugin
    } catch (error) {
      console.error('Failed to get plugin:', error)
      return null
    }
  }

  async function getEnabledPlugins(): Promise<Plugin[]> {
    try {
      const enabledPlugins = await invoke<Plugin[]>('get_enabled_plugins')
      return enabledPlugins
    } catch (error) {
      console.error('Failed to get enabled plugins:', error)
      return []
    }
  }

  async function updatePluginSetting(pluginId: string, key: string, value: any): Promise<void> {
    try {
      await invoke('update_plugin_setting', { pluginId, key, value })
      const plugin = plugins.value.find(p => p.id === pluginId)
      if (plugin) {
        plugin.settings[key] = value
      }
    } catch (error) {
      console.error('Failed to update plugin setting:', error)
      throw error
    }
  }

  async function fetchPluginSetting(pluginId: string, key: string): Promise<any> {
    try {
      const value = await invoke('get_plugin_setting', { pluginId, key })
      return value
    } catch (error) {
      console.error('Failed to get plugin setting:', error)
      return null
    }
  }

  async function triggerPluginEvent(hook: PluginHook, data: any): Promise<void> {
    try {
      await invoke('trigger_plugin_event', { hook, data })
    } catch (error) {
      console.error('Failed to trigger plugin event:', error)
      throw error
    }
  }

  async function hasPluginPermission(pluginId: string, permission: PluginPermission): Promise<boolean> {
    try {
      const hasPermission = await invoke<boolean>('has_plugin_permission', { pluginId, permission })
      return hasPermission
    } catch (error) {
      console.error('Failed to check plugin permission:', error)
      return false
    }
  }

  async function getPluginsByHook(hook: PluginHook): Promise<Plugin[]> {
    try {
      const hookPlugins = await invoke<Plugin[]>('get_plugins_by_hook', { hook })
      return hookPlugins
    } catch (error) {
      console.error('Failed to get plugins by hook:', error)
      return []
    }
  }

  async function searchPlugins(query: string): Promise<Plugin[]> {
    try {
      const results = await invoke<Plugin[]>('search_plugins', { query })
      return results
    } catch (error) {
      console.error('Failed to search plugins:', error)
      return []
    }
  }

  async function exportPluginSettings(): Promise<string> {
    try {
      const data = await invoke<string>('export_plugin_settings')
      return data
    } catch (error) {
      console.error('Failed to export plugin settings:', error)
      throw error
    }
  }

  async function importPluginSettings(data: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('import_plugin_settings', { data })
      await loadPlugins()
    } catch (error) {
      console.error('Failed to import plugin settings:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function getPluginStats(): Promise<PluginStats> {
    try {
      const stats = await invoke<PluginStats>('get_plugin_stats')
      return stats
    } catch (error) {
      console.error('Failed to get plugin stats:', error)
      return {
        total_plugins: 0,
        enabled_plugins: 0,
        disabled_plugins: 0,
        events_processed: 0
      }
    }
  }

  async function validatePluginManifest(manifestPath: string): Promise<{ valid: boolean; errors: string[] }> {
    try {
      const result = await invoke<{ valid: boolean; errors: string[] }>('validate_plugin_manifest', { manifestPath })
      return result
    } catch (error) {
      console.error('Failed to validate plugin manifest:', error)
      return { valid: false, errors: ['Failed to validate manifest'] }
    }
  }

  function getPluginById(pluginId: string): Plugin | undefined {
    return plugins.value.find(plugin => plugin.id === pluginId)
  }

  function getEnabledPluginsSync(): Plugin[] {
    return plugins.value.filter(plugin => plugin.enabled)
  }

  function getDisabledPlugins(): Plugin[] {
    return plugins.value.filter(plugin => !plugin.enabled)
  }

  function getPluginsByPermission(permission: PluginPermission): Plugin[] {
    return plugins.value.filter(plugin => 
      plugin.manifest.permissions.includes(permission)
    )
  }

  function getPluginsByHookSync(hook: PluginHook): Plugin[] {
    return plugins.value.filter(plugin => 
      plugin.enabled && plugin.manifest.hooks.includes(hook)
    )
  }

  function hasPermission(plugin: Plugin, permission: PluginPermission): boolean {
    return plugin.manifest.permissions.includes(permission)
  }

  function hasHook(plugin: Plugin, hook: PluginHook): boolean {
    return plugin.manifest.hooks.includes(hook)
  }

  function getPluginSettingSync(plugin: Plugin, key: string): any {
    return plugin.settings[key]
  }

  function getPluginSettingWithDefault(plugin: Plugin, key: string, defaultValue: any): any {
    const setting = plugin.manifest.settings.find(s => s.key === key)
    if (setting && plugin.settings[key] !== undefined) {
      return plugin.settings[key]
    }
    return setting?.value ?? defaultValue
  }

  function formatPluginSize(_: Plugin): string {
    return 'Unknown'
  }

  function getPluginVersion(plugin: Plugin): string {
    return plugin.manifest.version
  }

  function isPluginOutdated(_: Plugin): boolean {
    return false
  }

  function getPluginDependencies(_: Plugin): string[] {
    return []
  }

  return {
    plugins,
    isLoading,
    loadPlugins,
    installPlugin,
    uninstallPlugin,
    enablePlugin,
    disablePlugin,
    getPlugin,
    getEnabledPlugins,
    updatePluginSetting,
    fetchPluginSetting,
    triggerPluginEvent,
    hasPluginPermission,
    getPluginsByHook,
    searchPlugins,
    exportPluginSettings,
    importPluginSettings,
    getPluginStats,
    validatePluginManifest,
    getPluginById,
    getEnabledPluginsSync,
    getDisabledPlugins,
    getPluginsByPermission,
    getPluginsByHookSync,
    hasPermission,
    hasHook,
    getPluginSettingSync,
    getPluginSettingWithDefault,
    formatPluginSize,
    getPluginVersion,
    isPluginOutdated,
    getPluginDependencies
  }
})