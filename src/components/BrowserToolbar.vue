<template>
  <div class="bg-white/80 dark:bg-slate-800/80 backdrop-blur-xl border-b border-slate-200/50 dark:border-slate-700/50 px-6 py-3 flex items-center space-x-4 shadow-sm">
    <div class="flex items-center space-x-2">
      <button
        @click="goBack"
        :disabled="!activeTab?.can_go_back"
        class="p-2.5 rounded-xl hover:bg-slate-100/80 dark:hover:bg-slate-700/50 disabled:opacity-40 disabled:cursor-not-allowed transition-all duration-200 hover:scale-105 active:scale-95 shadow-sm hover:shadow-md"
        title="Go back"
      >
        <ChevronLeftIcon class="w-5 h-5 text-slate-600 dark:text-slate-300" />
      </button>
      
      <button
        @click="goForward"
        :disabled="!activeTab?.can_go_forward"
        class="p-2.5 rounded-xl hover:bg-slate-100/80 dark:hover:bg-slate-700/50 disabled:opacity-40 disabled:cursor-not-allowed transition-all duration-200 hover:scale-105 active:scale-95 shadow-sm hover:shadow-md"
        title="Go forward"
      >
        <ChevronRightIcon class="w-5 h-5 text-slate-600 dark:text-slate-300" />
      </button>
      
      <button
        @click="reload"
        :disabled="!activeTab"
        class="p-2.5 rounded-xl hover:bg-slate-100/80 dark:hover:bg-slate-700/50 disabled:opacity-40 disabled:cursor-not-allowed transition-all duration-200 hover:scale-105 active:scale-95 shadow-sm hover:shadow-md"
        title="Reload"
      >
        <ArrowPathIcon class="w-5 h-5 text-slate-600 dark:text-slate-300" :class="{ 'animate-spin': activeTab?.is_loading }" />
      </button>
    </div>
    
    <div class="flex-1 relative">
      <div class="relative">
        <input
          v-model="urlInput"
          @keydown.enter="navigateToUrl"
          @focus="showSuggestions = true"
          @blur="hideSuggestions"
          type="text"
          placeholder="Search or enter address"
          class="w-full px-4 py-3 pl-12 pr-16 rounded-2xl border border-slate-200/60 dark:border-slate-600/60 bg-white/90 dark:bg-slate-700/90 backdrop-blur-sm text-slate-900 dark:text-slate-100 placeholder-slate-400 dark:placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-400/50 dark:focus:ring-blue-400/50 dark:focus:border-blue-500/50 transition-all duration-200 shadow-sm focus:shadow-lg hover:shadow-md"
        />
        
        <div class="absolute left-4 top-1/2 transform -translate-y-1/2">
          <LockClosedIcon v-if="isSecure" class="w-5 h-5 text-emerald-500 dark:text-emerald-400" />
          <GlobeAltIcon v-else class="w-5 h-5 text-slate-400 dark:text-slate-500" />
        </div>
        
        <div class="absolute right-4 top-1/2 transform -translate-y-1/2 flex items-center space-x-2">
          <button
            v-if="activeTab?.url && activeTab.url !== urlInput"
            @click="clearUrl"
            class="p-1.5 rounded-lg hover:bg-slate-200/80 dark:hover:bg-slate-600/50 transition-all duration-200 hover:scale-110 active:scale-95"
            title="Clear"
          >
            <XMarkIcon class="w-4 h-4 text-slate-500 dark:text-slate-400" />
          </button>
          
          <button
            @click="addBookmark"
            :class="[
              'p-1.5 rounded-lg hover:bg-slate-200/80 dark:hover:bg-slate-600/50 transition-all duration-200 hover:scale-110 active:scale-95',
              isBookmarked ? 'text-amber-500 dark:text-amber-400' : 'text-slate-400 dark:text-slate-500'
            ]"
            title="Bookmark this page"
          >
            <StarIcon class="w-4 h-4" :class="{ 'fill-current': isBookmarked }" />
          </button>
        </div>
      </div>
      
      <div
        v-if="showSuggestions && suggestions.length > 0"
        class="absolute top-full left-0 right-0 mt-2 bg-white/95 dark:bg-slate-800/95 backdrop-blur-xl border border-slate-200/60 dark:border-slate-700/60 rounded-2xl shadow-2xl z-50 max-h-80 overflow-y-auto scrollbar-thin scrollbar-thumb-slate-300 dark:scrollbar-thumb-slate-600"
      >
        <div
          v-for="(suggestion, index) in suggestions"
          :key="index"
          @mousedown="selectSuggestion(suggestion)"
          class="px-5 py-3 hover:bg-slate-100/80 dark:hover:bg-slate-700/50 cursor-pointer flex items-center space-x-4 transition-all duration-200 first:rounded-t-2xl last:rounded-b-2xl hover:scale-[1.02] active:scale-[0.98]"
        >
          <div class="flex-shrink-0">
            <MagnifyingGlassIcon v-if="suggestion.type === 'search'" class="w-5 h-5 text-slate-400 dark:text-slate-500" />
            <ClockIcon v-else-if="suggestion.type === 'history'" class="w-5 h-5 text-blue-500 dark:text-blue-400" />
            <StarIcon v-else-if="suggestion.type === 'bookmark'" class="w-5 h-5 text-amber-500 dark:text-amber-400" />
            <GlobeAltIcon v-else class="w-5 h-5 text-slate-400 dark:text-slate-500" />
          </div>
          <div class="flex-1 min-w-0">
            <div class="text-sm font-semibold text-slate-900 dark:text-slate-100 truncate">
              {{ suggestion.title || suggestion.url }}
            </div>
            <div v-if="suggestion.url !== suggestion.title" class="text-xs text-slate-500 dark:text-slate-400 truncate mt-0.5">
              {{ suggestion.url }}
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <div class="flex items-center space-x-2">
      <button
        @click="toggleShields"
        :class="[
          'p-2.5 rounded-xl hover:bg-slate-100/80 dark:hover:bg-slate-700/50 transition-all duration-200 hover:scale-105 active:scale-95 shadow-sm hover:shadow-md',
          shieldsEnabled ? 'text-blue-600 dark:text-blue-400 bg-blue-50/50 dark:bg-blue-900/20' : 'text-slate-400 dark:text-slate-500'
        ]"
        title="Shields"
      >
        <ShieldCheckIcon class="w-5 h-5" />
      </button>
      
      <div class="relative">
        <button
          @click="showMenu = !showMenu"
          class="p-2.5 rounded-xl hover:bg-slate-100/80 dark:hover:bg-slate-700/50 transition-all duration-200 hover:scale-105 active:scale-95 shadow-sm hover:shadow-md"
          title="Menu"
        >
          <EllipsisVerticalIcon class="w-5 h-5 text-slate-600 dark:text-slate-300" />
        </button>
        
        <div
          v-if="showMenu"
          class="absolute right-0 top-full mt-2 w-56 bg-white/95 dark:bg-slate-800/95 backdrop-blur-xl border border-slate-200/60 dark:border-slate-700/60 rounded-2xl shadow-2xl z-50"
        >
          <div class="py-2">
            <button
              @click="openNewTab"
              class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-700/50 transition-all duration-200 first:rounded-t-2xl hover:scale-[1.02] active:scale-[0.98]"
            >
              New Tab
            </button>
            <button
              @click="openNewWindow"
              class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-700/50 transition-all duration-200 hover:scale-[1.02] active:scale-[0.98]"
            >
              New Window
            </button>
            <button
              @click="openPrivateWindow"
              class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-700/50 transition-all duration-200 hover:scale-[1.02] active:scale-[0.98]"
            >
              New Private Window
            </button>
            <hr class="my-2 border-slate-200/60 dark:border-slate-700/60" />
            <button
              @click="openBookmarks"
              class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-700/50 transition-all duration-200 hover:scale-[1.02] active:scale-[0.98]"
            >
              Bookmarks
            </button>
            <button
              @click="openHistory"
              class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-700/50 transition-all duration-200 hover:scale-[1.02] active:scale-[0.98]"
            >
              History
            </button>
            <button
              @click="openDownloads"
              class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-700/50 transition-all duration-200 hover:scale-[1.02] active:scale-[0.98]"
            >
              Downloads
            </button>
            <hr class="my-2 border-slate-200/60 dark:border-slate-700/60" />
            <button
              @click="openSettings"
              class="w-full text-left px-5 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-700/50 transition-all duration-200 last:rounded-b-2xl hover:scale-[1.02] active:scale-[0.98]"
            >
              Settings
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useBrowserStore } from '../stores/browser'
import { useBookmarksStore } from '../stores/bookmarks'
import { useHistoryStore } from '../stores/history'
import {
  ChevronLeftIcon,
  ChevronRightIcon,
  ArrowPathIcon,
  LockClosedIcon,
  GlobeAltIcon,
  XMarkIcon,
  StarIcon,
  MagnifyingGlassIcon,
  ClockIcon,
  ShieldCheckIcon,
  EllipsisVerticalIcon
} from '@heroicons/vue/24/outline'

const router = useRouter()
const browserStore = useBrowserStore()
const bookmarksStore = useBookmarksStore()
const historyStore = useHistoryStore()

const urlInput = ref('')
const showSuggestions = ref(false)
const showMenu = ref(false)
const suggestions = ref<any[]>([])
const shieldsEnabled = ref(true)
const isBookmarked = ref(false)

const activeTab = computed(() => browserStore.activeTab)
const isSecure = computed(() => activeTab.value?.url?.startsWith('https://'))

watch(activeTab, (newTab) => {
  if (newTab) {
    urlInput.value = newTab.url
    checkBookmarkStatus()
  }
}, { immediate: true })

watch(urlInput, async (newValue) => {
  if (newValue && showSuggestions.value) {
    await updateSuggestions(newValue)
  }
})

async function updateSuggestions(query: string) {
  if (!query.trim()) {
    suggestions.value = []
    return
  }
  
  try {
    const [historyResults, bookmarkResults] = await Promise.all([
      historyStore.searchHistory(query, 5),
      bookmarksStore.searchBookmarks(query, 5)
    ])
    
    suggestions.value = [
      { type: 'search', title: `Search for "${query}"`, url: `https://www.google.com/search?q=${encodeURIComponent(query)}` },
      ...historyResults.map((h: any) => ({ type: 'history', title: h.title, url: h.url })),
      ...bookmarkResults.map((b: any) => ({ type: 'bookmark', title: b.title, url: b.url }))
    ]
  } catch (error) {
    console.error('Failed to get suggestions:', error)
    suggestions.value = []
  }
}

function selectSuggestion(suggestion: any) {
  urlInput.value = suggestion.url
  showSuggestions.value = false
  navigateToUrl()
}

function hideSuggestions() {
  setTimeout(() => {
    showSuggestions.value = false
  }, 200)
}

async function navigateToUrl() {
  if (!activeTab.value || !browserStore.activeWindow) return
  
  let url = urlInput.value.trim()
  if (!url) return
  
  if (!url.includes('://')) {
    if (url.includes('.') && !url.includes(' ')) {
      url = `https://${url}`
    } else {
      url = `https://www.google.com/search?q=${encodeURIComponent(url)}`
    }
  }
  
  try {
    await browserStore.updateTabUrl(activeTab.value.id, url)
    await historyStore.addVisit(url, activeTab.value.title || url)
  } catch (error) {
    console.error('Failed to navigate:', error)
  }
}

function clearUrl() {
  urlInput.value = ''
}

async function goBack() {
  if (!activeTab.value || !browserStore.activeWindow) return
  try {
    await browserStore.goBack(activeTab.value.id)
  } catch (error) {
    console.error('Failed to go back:', error)
  }
}

async function goForward() {
  if (!activeTab.value || !browserStore.activeWindow) return
  try {
    await browserStore.goForward(activeTab.value.id)
  } catch (error) {
    console.error('Failed to go forward:', error)
  }
}

async function reload() {
  if (!activeTab.value || !browserStore.activeWindow) return
  try {
    await browserStore.reloadTab(activeTab.value.id)
  } catch (error) {
    console.error('Failed to reload:', error)
  }
}

async function addBookmark() {
  if (!activeTab.value || !browserStore.activeWindow) return
  
  try {
    if (isBookmarked.value) {
      await bookmarksStore.removeBookmarkByUrl(activeTab.value.url)
    } else {
      await bookmarksStore.addBookmark({
        title: activeTab.value.title,
        url: activeTab.value.url,
        favicon: activeTab.value.favicon
      })
    }
    await checkBookmarkStatus()
  } catch (error) {
    console.error('Failed to toggle bookmark:', error)
  }
}

async function checkBookmarkStatus() {
  if (!activeTab.value || !browserStore.activeWindow) return
  
  try {
    const bookmarks = await bookmarksStore.searchBookmarks(activeTab.value.url, 1)
    isBookmarked.value = bookmarks.some((b: any) => b.url === activeTab.value?.url)
  } catch (error) {
    console.error('Failed to check bookmark status:', error)
    isBookmarked.value = false
  }
}

function toggleShields() {
  shieldsEnabled.value = !shieldsEnabled.value
}

async function openNewTab() {
  try {
    if (!browserStore.activeWindow) {
      const windowId = await browserStore.createWindow()
      await browserStore.createTab(windowId, 'https://www.google.com')
    } else {
      await browserStore.createTab(browserStore.activeWindow.id, 'https://www.google.com')
    }
    showMenu.value = false
  } catch (error) {
    console.error('Failed to open new tab:', error)
  }
}

async function openNewWindow() {
  try {
    const windowId = await browserStore.createWindow()
    await browserStore.createTab(windowId, 'https://www.google.com')
    showMenu.value = false
  } catch (error) {
    console.error('Failed to open new window:', error)
  }
}

async function openPrivateWindow() {
  try {
    const windowId = await browserStore.createWindow(true)
    await browserStore.createTab(windowId, 'https://www.google.com', true)
    showMenu.value = false
  } catch (error) {
    console.error('Failed to open private window:', error)
  }
}

function openBookmarks() {
  router.push('/bookmarks')
  showMenu.value = false
}

function openHistory() {
  router.push('/history')
  showMenu.value = false
}

function openDownloads() {
  router.push('/downloads')
  showMenu.value = false
}

function openSettings() {
  router.push('/settings')
  showMenu.value = false
}

function handleClickOutside(event: Event) {
  const target = event.target as Element
  if (!target.closest('.relative')) {
    showMenu.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>