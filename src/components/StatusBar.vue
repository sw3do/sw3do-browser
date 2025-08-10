<template>
  <div class="h-7 bg-white/80 dark:bg-slate-800/80 backdrop-blur-xl border-t border-slate-200/50 dark:border-slate-700/50 px-4 flex items-center justify-between text-xs text-slate-600 dark:text-slate-400 shadow-lg">
    <div class="flex items-center space-x-3">
      <div v-if="activeTab" class="flex items-center space-x-2">
        <span v-if="activeTab.is_loading" class="flex items-center space-x-2">
          <div class="w-3 h-3 border-2 border-blue-500/70 border-t-transparent rounded-full animate-spin"></div>
          <span class="font-medium text-blue-600 dark:text-blue-400">Loading...</span>
        </span>
        <span v-else class="flex items-center space-x-2">
          <div class="w-2 h-2 bg-emerald-500 rounded-full shadow-sm animate-pulse"></div>
          <span class="font-medium text-emerald-600 dark:text-emerald-400">Ready</span>
        </span>
      </div>
      
      <div v-if="activeTab" class="flex items-center space-x-1.5 bg-slate-100/60 dark:bg-slate-700/60 px-2 py-1 rounded-lg">
        <span class="font-medium">Zoom:</span>
        <span class="font-mono">{{ Math.round(activeTab.zoom_level * 100) }}%</span>
      </div>
    </div>
    
    <div class="flex items-center space-x-4">
      <div v-if="connectionStatus" class="flex items-center space-x-1.5 bg-slate-100/60 dark:bg-slate-700/60 px-2 py-1 rounded-lg">
        <div class="w-2 h-2 rounded-full" :class="{
          'bg-emerald-500': connectionStatus === 'secure',
          'bg-yellow-500': connectionStatus === 'warning',
          'bg-red-500': connectionStatus === 'insecure'
        }"></div>
        <span class="capitalize font-medium">{{ connectionStatus }}</span>
      </div>
      
      <div v-if="activeTab" class="flex items-center space-x-1">
        <span class="font-mono text-slate-700 dark:text-slate-300 bg-slate-100/60 dark:bg-slate-700/60 px-2 py-1 rounded-lg">{{ formatUrl(activeTab.url) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useBrowserStore } from '../stores/browser'

const browserStore = useBrowserStore()

const activeTab = computed(() => {
  return browserStore.activeTab
})

const connectionStatus = computed(() => {
  if (!activeTab.value?.url) return null
  
  const url = activeTab.value.url
  if (url.startsWith('https://')) return 'secure'
  if (url.startsWith('http://')) return 'insecure'
  if (url.startsWith('about:') || url.startsWith('chrome:')) return 'secure'
  return 'warning'
})

const formatUrl = (url: string) => {
  if (!url) return ''
  try {
    const urlObj = new URL(url)
    return urlObj.hostname
  } catch {
    return url
  }
}
</script>