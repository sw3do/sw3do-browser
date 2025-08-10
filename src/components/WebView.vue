<template>
  <div class="flex-1 bg-gradient-to-br from-white via-slate-50 to-slate-100 dark:from-slate-800 dark:via-slate-700 dark:to-slate-800 relative">
    <div v-if="currentTab" class="h-full">
      <div 
        v-if="currentTab.url && currentTab.url !== 'about:blank'"
        class="w-full h-full bg-gradient-to-br from-white/90 via-slate-50/90 to-slate-100/90 dark:from-slate-800/90 dark:via-slate-700/90 dark:to-slate-800/90 backdrop-blur-sm flex items-center justify-center"
      >
        <div class="text-center p-8 bg-white/60 dark:bg-slate-800/60 backdrop-blur-xl rounded-3xl shadow-2xl border border-slate-200/50 dark:border-slate-700/50">
          <div class="text-6xl mb-6 animate-pulse">🌐</div>
          <h3 class="text-2xl font-bold text-slate-800 dark:text-slate-200 mb-4">
            {{ currentTab.title || 'Loading...' }}
          </h3>
          <p class="text-sm text-slate-600 dark:text-slate-400 mb-6 font-mono bg-slate-100/80 dark:bg-slate-700/80 px-4 py-2 rounded-xl">
            {{ currentTab.url }}
          </p>
          <div class="text-xs text-slate-500 dark:text-slate-400 bg-slate-50/80 dark:bg-slate-800/80 px-4 py-2 rounded-lg">
            Web content is displayed in a separate webview window
          </div>
        </div>
      </div>
      <div v-else class="h-full flex items-center justify-center bg-gradient-to-br from-slate-100/80 via-white/80 to-slate-200/80 dark:from-slate-700/80 dark:via-slate-600/80 dark:to-slate-700/80">
        <div class="text-center p-12 bg-white/70 dark:bg-slate-800/70 backdrop-blur-xl rounded-3xl shadow-2xl border border-slate-200/50 dark:border-slate-700/50">
          <div class="text-8xl mb-8 animate-bounce">🌐</div>
          <h2 class="text-3xl font-bold text-slate-800 dark:text-slate-200 mb-4">
            New Tab
          </h2>
          <p class="text-slate-600 dark:text-slate-400 text-lg">
            Enter a URL in the address bar to start browsing
          </p>
        </div>
      </div>
    </div>
    <div v-else class="h-full flex items-center justify-center bg-gradient-to-br from-slate-100/80 via-white/80 to-slate-200/80 dark:from-slate-700/80 dark:via-slate-600/80 dark:to-slate-700/80">
      <div class="text-center p-12 bg-white/70 dark:bg-slate-800/70 backdrop-blur-xl rounded-3xl shadow-2xl border border-slate-200/50 dark:border-slate-700/50">
        <div class="text-8xl mb-8 animate-pulse">📄</div>
        <h2 class="text-3xl font-bold text-slate-800 dark:text-slate-200 mb-4">
          No Tab Selected
        </h2>
        <p class="text-slate-600 dark:text-slate-400 text-lg">
          Create a new tab to start browsing
        </p>
      </div>
    </div>
    
    <div v-if="isLoading" class="absolute inset-0 bg-white bg-opacity-75 dark:bg-gray-800 dark:bg-opacity-75 flex items-center justify-center">
      <div class="flex items-center space-x-2">
        <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"></div>
        <span class="text-gray-600 dark:text-gray-400">Loading...</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, watch, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useBrowserStore } from '../stores/browser'

const browserStore = useBrowserStore()
let previousTabId: string | null = null

const currentTab = computed(() => {
  return browserStore.activeTab
})

const isLoading = computed(() => {
  return currentTab.value?.is_loading || false
})

async function createWebviewForTab(tab: any) {
  if (!tab || !tab.url || tab.url === 'about:blank') return
  
  try {
    await invoke('create_webview_tab', {
      tabId: tab.id,
      url: tab.url
    })
    
    await invoke('show_webview_tab', {
      tabId: tab.id
    })
    
    browserStore.updateTabState(tab.id, {
      is_loading: false
    })
  } catch (error) {
    console.error('Failed to create webview for tab:', error)
    browserStore.updateTabState(tab.id, {
      is_loading: false,
      title: 'Failed to load page'
    })
  }
}

async function hideWebviewForTab(tabId: string) {
  try {
    await invoke('hide_webview_tab', { tabId })
  } catch (error) {
    console.error('Failed to hide webview for tab:', error)
  }
}

watch(
  () => currentTab.value,
  async (newTab, oldTab) => {
    if (oldTab && oldTab.id !== newTab?.id) {
      await hideWebviewForTab(oldTab.id)
    }
    
    if (newTab && newTab.id !== previousTabId) {
      previousTabId = newTab.id
      
      if (newTab.url && newTab.url !== 'about:blank') {
        browserStore.updateTabState(newTab.id, {
          is_loading: true
        })
        await createWebviewForTab(newTab)
      }
    }
  },
  { immediate: true }
)

watch(
  () => currentTab.value?.url,
  async (newUrl, oldUrl) => {
    if (newUrl && newUrl !== oldUrl && currentTab.value) {
      if (newUrl !== 'about:blank') {
        browserStore.updateTabState(currentTab.value.id, {
          is_loading: true
        })
        
        try {
          await invoke('navigate_webview_tab', {
            tabId: currentTab.value.id,
            url: newUrl
          })
          
          browserStore.updateTabState(currentTab.value.id, {
            is_loading: false
          })
        } catch (error) {
          console.error('Failed to navigate webview:', error)
          browserStore.updateTabState(currentTab.value.id, {
            is_loading: false,
            title: 'Failed to load page'
          })
        }
      }
    }
  }
)

onMounted(() => {
  if (currentTab.value && currentTab.value.url && currentTab.value.url !== 'about:blank') {
    createWebviewForTab(currentTab.value)
  }
})

onUnmounted(() => {
  if (currentTab.value) {
    hideWebviewForTab(currentTab.value.id)
  }
})
</script>