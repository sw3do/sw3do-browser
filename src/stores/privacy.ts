import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface FilterList {
  id: string
  name: string
  url: string
  enabled: boolean
  last_updated: string
  rules_count: number
  description?: string
}

export interface SiteShields {
  domain: string
  ads_blocked: boolean
  trackers_blocked: boolean
  third_party_cookies_blocked: boolean
  fingerprinting_blocked: boolean
  https_only: boolean
  scripts_blocked: boolean
}

export interface BlockingStats {
  ads_blocked: number
  trackers_blocked: number
  cookies_blocked: number
  fingerprinting_attempts_blocked: number
  https_upgrades: number
  scripts_blocked: number
  bandwidth_saved: number
  time_saved: number
}

export interface GlobalPrivacySettings {
  global_ad_blocking: boolean
  global_tracker_blocking: boolean
  global_cookie_blocking: boolean
  global_fingerprinting_protection: boolean
  global_https_only: boolean
  global_script_blocking: boolean
  auto_update_filters: boolean
  update_frequency: number
}

export const usePrivacyStore = defineStore('privacy', () => {
  const filterLists = ref<FilterList[]>([])
  const siteShields = ref<Record<string, SiteShields>>({})
  const blockingStats = ref<BlockingStats>({
    ads_blocked: 0,
    trackers_blocked: 0,
    cookies_blocked: 0,
    fingerprinting_attempts_blocked: 0,
    https_upgrades: 0,
    scripts_blocked: 0,
    bandwidth_saved: 0,
    time_saved: 0
  })
  const privacySettings = ref<GlobalPrivacySettings>({
    global_ad_blocking: true,
    global_tracker_blocking: true,
    global_cookie_blocking: false,
    global_fingerprinting_protection: true,
    global_https_only: false,
    global_script_blocking: false,
    auto_update_filters: true,
    update_frequency: 24
  })
  const isLoading = ref(false)

  async function loadFilterLists(): Promise<void> {
    try {
      isLoading.value = true
      const lists = await invoke<FilterList[]>('get_filter_lists')
      filterLists.value = lists
    } catch (error) {
      console.error('Failed to load filter lists:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function addFilterList(name: string, url: string, description?: string): Promise<string> {
    try {
      isLoading.value = true
      const listId = await invoke<string>('add_filter_list', {
        name,
        url,
        description
      })
      await loadFilterLists()
      return listId
    } catch (error) {
      console.error('Failed to add filter list:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function removeFilterList(listId: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('remove_filter_list', { listId })
      filterLists.value = filterLists.value.filter(list => list.id !== listId)
    } catch (error) {
      console.error('Failed to remove filter list:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function toggleFilterList(listId: string, enabled: boolean): Promise<void> {
    try {
      await invoke('toggle_filter_list', { listId, enabled })
      const list = filterLists.value.find(l => l.id === listId)
      if (list) {
        list.enabled = enabled
      }
    } catch (error) {
      console.error('Failed to toggle filter list:', error)
      throw error
    }
  }

  async function updateFilterLists(): Promise<void> {
    try {
      isLoading.value = true
      await invoke('update_filter_lists')
      await loadFilterLists()
    } catch (error) {
      console.error('Failed to update filter lists:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function loadSiteShields(domain: string): Promise<SiteShields> {
    try {
      const shields = await invoke<SiteShields>('get_site_shields', { domain })
      siteShields.value[domain] = shields
      return shields
    } catch (error) {
      console.error('Failed to load site shields:', error)
      const defaultShields: SiteShields = {
        domain,
        ads_blocked: privacySettings.value.global_ad_blocking,
        trackers_blocked: privacySettings.value.global_tracker_blocking,
        third_party_cookies_blocked: privacySettings.value.global_cookie_blocking,
        fingerprinting_blocked: privacySettings.value.global_fingerprinting_protection,
        https_only: privacySettings.value.global_https_only,
        scripts_blocked: privacySettings.value.global_script_blocking
      }
      siteShields.value[domain] = defaultShields
      return defaultShields
    }
  }

  async function updateSiteShields(domain: string, shields: Partial<SiteShields>): Promise<void> {
    try {
      await invoke('update_site_shields', { domain, ...shields })
      if (siteShields.value[domain]) {
        siteShields.value[domain] = { ...siteShields.value[domain], ...shields }
      }
    } catch (error) {
      console.error('Failed to update site shields:', error)
      throw error
    }
  }

  async function resetSiteShields(domain: string): Promise<void> {
    try {
      await invoke('reset_site_shields', { domain })
      delete siteShields.value[domain]
    } catch (error) {
      console.error('Failed to reset site shields:', error)
      throw error
    }
  }

  async function loadBlockingStats(): Promise<void> {
    try {
      const stats = await invoke<BlockingStats>('get_blocking_stats')
      blockingStats.value = stats
    } catch (error) {
      console.error('Failed to load blocking stats:', error)
      throw error
    }
  }

  async function resetBlockingStats(): Promise<void> {
    try {
      await invoke('reset_blocking_stats')
      blockingStats.value = {
        ads_blocked: 0,
        trackers_blocked: 0,
        cookies_blocked: 0,
        fingerprinting_attempts_blocked: 0,
        https_upgrades: 0,
        scripts_blocked: 0,
        bandwidth_saved: 0,
        time_saved: 0
      }
    } catch (error) {
      console.error('Failed to reset blocking stats:', error)
      throw error
    }
  }

  async function loadPrivacySettings(): Promise<void> {
    try {
      const settings = await invoke<GlobalPrivacySettings>('get_privacy_settings')
      privacySettings.value = settings
    } catch (error) {
      console.error('Failed to load privacy settings:', error)
      throw error
    }
  }

  async function updatePrivacySettings(updates: Partial<GlobalPrivacySettings>): Promise<void> {
    try {
      await invoke('update_privacy_settings', updates)
      privacySettings.value = { ...privacySettings.value, ...updates }
    } catch (error) {
      console.error('Failed to update privacy settings:', error)
      throw error
    }
  }

  async function checkUrl(url: string): Promise<{ blocked: boolean; reason?: string }> {
    try {
      const result = await invoke<{ blocked: boolean; reason?: string }>('check_url', { url })
      return result
    } catch (error) {
      console.error('Failed to check URL:', error)
      return { blocked: false }
    }
  }

  async function addCustomRule(rule: string): Promise<void> {
    try {
      await invoke('add_custom_rule', { rule })
    } catch (error) {
      console.error('Failed to add custom rule:', error)
      throw error
    }
  }

  async function removeCustomRule(rule: string): Promise<void> {
    try {
      await invoke('remove_custom_rule', { rule })
    } catch (error) {
      console.error('Failed to remove custom rule:', error)
      throw error
    }
  }

  async function getCustomRules(): Promise<string[]> {
    try {
      const rules = await invoke<string[]>('get_custom_rules')
      return rules
    } catch (error) {
      console.error('Failed to get custom rules:', error)
      return []
    }
  }

  async function exportPrivacyData(): Promise<string> {
    try {
      const data = await invoke<string>('export_privacy_data')
      return data
    } catch (error) {
      console.error('Failed to export privacy data:', error)
      throw error
    }
  }

  async function importPrivacyData(data: string): Promise<void> {
    try {
      isLoading.value = true
      await invoke('import_privacy_data', { data })
      await loadFilterLists()
      await loadPrivacySettings()
      await loadBlockingStats()
    } catch (error) {
      console.error('Failed to import privacy data:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  function getSiteShields(domain: string): SiteShields | null {
    return siteShields.value[domain] || null
  }

  function getTotalBlocked(): number {
    return blockingStats.value.ads_blocked + 
           blockingStats.value.trackers_blocked + 
           blockingStats.value.cookies_blocked + 
           blockingStats.value.fingerprinting_attempts_blocked + 
           blockingStats.value.scripts_blocked
  }

  function formatBandwidthSaved(): string {
    const bytes = blockingStats.value.bandwidth_saved
    const units = ['B', 'KB', 'MB', 'GB']
    let size = bytes
    let unitIndex = 0
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024
      unitIndex++
    }
    
    return `${size.toFixed(1)} ${units[unitIndex]}`
  }

  function formatTimeSaved(): string {
    const seconds = blockingStats.value.time_saved
    
    if (seconds < 60) {
      return `${seconds}s`
    } else if (seconds < 3600) {
      return `${Math.round(seconds / 60)}m`
    } else if (seconds < 86400) {
      return `${Math.round(seconds / 3600)}h`
    } else {
      return `${Math.round(seconds / 86400)}d`
    }
  }

  return {
    filterLists,
    siteShields,
    blockingStats,
    privacySettings,
    isLoading,
    loadFilterLists,
    addFilterList,
    removeFilterList,
    toggleFilterList,
    updateFilterLists,
    loadSiteShields,
    updateSiteShields,
    resetSiteShields,
    loadBlockingStats,
    resetBlockingStats,
    loadPrivacySettings,
    updatePrivacySettings,
    checkUrl,
    addCustomRule,
    removeCustomRule,
    getCustomRules,
    exportPrivacyData,
    importPrivacyData,
    getSiteShields,
    getTotalBlocked,
    formatBandwidthSaved,
    formatTimeSaved
  }
})