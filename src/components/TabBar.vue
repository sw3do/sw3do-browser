<template>
  <div class="bg-slate-100/80 dark:bg-slate-800/80 backdrop-blur-sm border-b border-slate-200/50 dark:border-slate-700/50">
    <div class="flex items-center">
      <div class="flex-1 flex items-center overflow-x-auto scrollbar-thin scrollbar-thumb-slate-300 dark:scrollbar-thumb-slate-600 scrollbar-track-transparent">
        <div
          v-for="tab in tabs"
          :key="tab.id"
          class="flex items-center min-w-0 max-w-xs group relative transition-all duration-200"
          :class="{
            'bg-white/90 dark:bg-slate-700/90 backdrop-blur-sm border-r border-slate-200/60 dark:border-slate-600/60 shadow-sm': tab.id === activeTabId,
            'bg-slate-50/50 dark:bg-slate-750/50 border-r border-slate-200/40 dark:border-slate-600/40 hover:bg-slate-100/70 dark:hover:bg-slate-700/70 hover:shadow-sm': tab.id !== activeTabId
          }"
        >
          <button
            @click="setActiveTab(tab.id)"
            class="flex items-center px-5 py-3 min-w-0 flex-1 text-left focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:ring-inset transition-all duration-200 hover:scale-[1.02] active:scale-[0.98]"
          >
            <div class="flex items-center min-w-0 flex-1">
              <img
                v-if="tab.favicon"
                :src="tab.favicon"
                :alt="tab.title"
                class="w-4 h-4 mr-3 flex-shrink-0 rounded-sm"
                @error="handleFaviconError"
              >
              <GlobeAltIcon v-else class="w-4 h-4 mr-3 flex-shrink-0 text-slate-400 dark:text-slate-500" />
              
              <span class="truncate text-sm font-semibold text-slate-900 dark:text-slate-100">
                {{ tab.title || 'New Tab' }}
              </span>
              
              <div class="flex items-center ml-3 space-x-2">
                <SpeakerXMarkIcon
                  v-if="tab.is_muted"
                  class="w-3.5 h-3.5 text-slate-400 dark:text-slate-500 flex-shrink-0"
                />
                <div
                  v-if="tab.is_loading"
                  class="w-3.5 h-3.5 border-2 border-blue-500 dark:border-blue-400 border-t-transparent rounded-full animate-spin flex-shrink-0"
                ></div>
                <div
                  v-if="tab.is_pinned"
                  class="w-2 h-2 bg-blue-500 dark:bg-blue-400 rounded-full flex-shrink-0"
                ></div>
              </div>
            </div>
          </button>
          
          <div class="flex items-center pr-3">
            <button
              v-if="!tab.is_pinned"
              @click.stop="closeTab(tab.id)"
              class="p-1.5 rounded-lg hover:bg-slate-200/80 dark:hover:bg-slate-600/50 opacity-0 group-hover:opacity-100 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500/50 hover:scale-110 active:scale-95"
            >
              <XMarkIcon class="w-3.5 h-3.5 text-slate-500 dark:text-slate-400" />
            </button>
          </div>
          
          <div
            v-if="tab.id === activeTabId"
            class="absolute bottom-0 left-0 right-0 h-1 bg-gradient-to-r from-blue-500 to-blue-600 dark:from-blue-400 dark:to-blue-500 rounded-t-sm"
          ></div>
        </div>
      </div>
      
      <div class="flex items-center px-4 space-x-2">
        <button
          @click="createNewTab"
          class="p-2.5 rounded-xl hover:bg-slate-200/80 dark:hover:bg-slate-600/50 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500/50 hover:scale-105 active:scale-95 shadow-sm hover:shadow-md"
          title="New Tab (Ctrl+T)"
        >
          <PlusIcon class="w-4 h-4 text-slate-600 dark:text-slate-300" />
        </button>
        
        <div class="relative">
          <button
            @click="showTabMenu = !showTabMenu"
            class="p-2.5 rounded-xl hover:bg-slate-200/80 dark:hover:bg-slate-600/50 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500/50 hover:scale-105 active:scale-95 shadow-sm hover:shadow-md"
            title="Tab Options"
          >
            <EllipsisHorizontalIcon class="w-4 h-4 text-slate-600 dark:text-slate-300" />
          </button>
          
          <div
            v-if="showTabMenu"
            class="absolute right-0 top-full mt-2 w-52 bg-white/95 dark:bg-slate-700/95 backdrop-blur-xl rounded-2xl shadow-2xl border border-slate-200/60 dark:border-slate-600/60 z-50"
          >
            <div class="py-2">
              <button
                @click="createNewTab(); showTabMenu = false"
                class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-600/50 transition-all duration-200 first:rounded-t-2xl hover:scale-[1.02] active:scale-[0.98]"
              >
                New Tab
              </button>
              <button
                @click="createNewPrivateTab(); showTabMenu = false"
                class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-600/50 transition-all duration-200 hover:scale-[1.02] active:scale-[0.98]"
              >
                New Private Tab
              </button>
              <hr class="my-2 border-slate-200/60 dark:border-slate-600/60">
              <button
                @click="duplicateActiveTab(); showTabMenu = false"
                class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-600/50 transition-all duration-200 disabled:opacity-40 disabled:cursor-not-allowed hover:scale-[1.02] active:scale-[0.98]"
                :disabled="!activeTab"
              >
                Duplicate Tab
              </button>
              <button
                @click="pinActiveTab(); showTabMenu = false"
                class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-600/50 transition-all duration-200 disabled:opacity-40 disabled:cursor-not-allowed hover:scale-[1.02] active:scale-[0.98]"
                :disabled="!activeTab"
              >
                {{ activeTab?.is_pinned ? 'Unpin' : 'Pin' }} Tab
              </button>
              <button
                @click="muteActiveTab(); showTabMenu = false"
                class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-600/50 transition-all duration-200 disabled:opacity-40 disabled:cursor-not-allowed hover:scale-[1.02] active:scale-[0.98]"
                :disabled="!activeTab"
              >
                {{ activeTab?.is_muted ? 'Unmute' : 'Mute' }} Tab
              </button>
              <hr class="my-1 border-gray-200 dark:border-gray-600">
              <button
                @click="closeOtherTabs(); showTabMenu = false"
                class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600"
                :disabled="tabs.length <= 1"
              >
                Close Other Tabs
              </button>
              <button
                @click="closeTabsToRight(); showTabMenu = false"
                class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600"
                :disabled="!canCloseTabsToRight"
              >
                Close Tabs to Right
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue'
import {
  GlobeAltIcon,
  XMarkIcon,
  PlusIcon,
  EllipsisHorizontalIcon,
  SpeakerXMarkIcon
} from '@heroicons/vue/24/outline'
import { useBrowserStore } from '../stores/browser'

const browserStore = useBrowserStore()
const showTabMenu = ref(false)

const tabs = computed(() => {
  if (!browserStore.activeWindowId) return []
  return browserStore.windowTabs(browserStore.activeWindowId)
})
const activeTabId = computed(() => browserStore.activeTab?.id)
const activeTab = computed(() => browserStore.activeTab)

const canCloseTabsToRight = computed(() => {
  if (!activeTab.value || tabs.value.length <= 1) return false
  const activeIndex = tabs.value.findIndex((tab: any) => tab.id === activeTab.value?.id)
  return activeIndex < tabs.value.length - 1
})

async function setActiveTab(tabId: string) {
  try {
    if (!browserStore.activeWindowId) {
      console.error('No active window')
      return
    }
    await browserStore.setActiveTab(browserStore.activeWindowId, tabId)
  } catch (error) {
    console.error('Failed to set active tab:', error)
  }
}

async function closeTab(tabId: string) {
  try {
    await browserStore.closeTab(tabId)
  } catch (error) {
    console.error('Failed to close tab:', error)
  }
}

async function createNewTab() {
  try {
    let windowId = browserStore.activeWindowId
    if (!windowId) {
      windowId = await browserStore.createWindow()
    }
    await browserStore.createTab(windowId, 'https://www.google.com')
  } catch (error) {
    console.error('Failed to create new tab:', error)
  }
}

async function createNewPrivateTab() {
  try {
    const windowId = await browserStore.createWindow(true)
    await browserStore.createTab(windowId, 'https://www.google.com', true)
  } catch (error) {
    console.error('Failed to create new private tab:', error)
  }
}

async function duplicateActiveTab() {
  if (!activeTab.value) return
  
  try {
    await browserStore.duplicateTab(activeTab.value.id)
  } catch (error) {
    console.error('Failed to duplicate tab:', error)
  }
}

async function pinActiveTab() {
  if (!activeTab.value) return
  
  try {
    if (activeTab.value.is_pinned) {
      await browserStore.unpinTab(activeTab.value.id)
    } else {
      await browserStore.pinTab(activeTab.value.id)
    }
  } catch (error) {
    console.error('Failed to pin/unpin tab:', error)
  }
}

async function muteActiveTab() {
  if (!activeTab.value) return
  
  try {
    if (activeTab.value.is_muted) {
      await browserStore.unmuteTab(activeTab.value.id)
    } else {
      await browserStore.muteTab(activeTab.value.id)
    }
  } catch (error) {
    console.error('Failed to mute/unmute tab:', error)
  }
}

async function closeOtherTabs() {
  if (!activeTab.value) return
  
  try {
    const otherTabs = tabs.value.filter((tab: any) => tab.id !== activeTab.value?.id && !tab.is_pinned)
    for (const tab of otherTabs) {
      await browserStore.closeTab(tab.id)
    }
  } catch (error) {
    console.error('Failed to close other tabs:', error)
  }
}

async function closeTabsToRight() {
  if (!activeTab.value) return
  
  try {
    const activeIndex = tabs.value.findIndex((tab: any) => tab.id === activeTab.value?.id)
    const tabsToClose = tabs.value.slice(activeIndex + 1).filter((tab: any) => !tab.is_pinned)
    
    for (const tab of tabsToClose) {
      await browserStore.closeTab(tab.id)
    }
  } catch (error) {
    console.error('Failed to close tabs to right:', error)
  }
}

function handleFaviconError(event: Event) {
  const img = event.target as HTMLImageElement
  img.style.display = 'none'
}

function handleKeydown(event: KeyboardEvent) {
  if (event.ctrlKey || event.metaKey) {
    switch (event.key) {
      case 't':
        event.preventDefault()
        createNewTab()
        break
      case 'w':
        event.preventDefault()
        if (activeTab.value) {
          closeTab(activeTab.value.id)
        }
        break
      case 'Tab':
        event.preventDefault()
        const currentIndex = tabs.value.findIndex((tab: any) => tab.id === activeTabId.value)
        const nextIndex = event.shiftKey 
          ? (currentIndex - 1 + tabs.value.length) % tabs.value.length
          : (currentIndex + 1) % tabs.value.length
        if (tabs.value[nextIndex]) {
          setActiveTab((tabs.value[nextIndex] as any).id)
        }
        break
    }
  }
}

function handleClickOutside(event: Event) {
  const target = event.target as HTMLElement
  if (!target.closest('.relative')) {
    showTabMenu.value = false
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
</style>