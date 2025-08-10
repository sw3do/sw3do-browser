<template>
  <div class="h-screen flex flex-col bg-gradient-to-br from-slate-50 via-white to-slate-100 dark:from-slate-900 dark:via-slate-800 dark:to-slate-900 transition-colors duration-300">
    <BrowserToolbar />
    <div class="flex-1 flex shadow-inner">
      <BrowserSidebar v-if="showSidebar" class="border-r border-slate-200/50 dark:border-slate-700/50" />
      <div class="flex-1 flex flex-col backdrop-blur-sm">
        <TabBar />
        <WebView />
      </div>
    </div>
    <StatusBar />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useBrowserStore } from '../stores/browser'
import BrowserToolbar from '../components/BrowserToolbar.vue'
import BrowserSidebar from '../components/BrowserSidebar.vue'
import TabBar from '../components/TabBar.vue'
import WebView from '../components/WebView.vue'
import StatusBar from '../components/StatusBar.vue'

const browserStore = useBrowserStore()
const showSidebar = ref(false)

onMounted(async () => {
  try {
    await browserStore.syncWindowState()
    await browserStore.syncTabState()
    
    if (browserStore.windows.size === 0) {
      const windowId = await browserStore.createWindow()
      await browserStore.createTab(windowId, 'https://www.google.com')
    }
  } catch (error) {
    console.error('Failed to initialize browser:', error)
  }
})
</script>