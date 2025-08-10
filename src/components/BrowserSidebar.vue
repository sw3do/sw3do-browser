<template>
  <div class="w-64 bg-white/90 dark:bg-slate-800/90 backdrop-blur-xl border-r border-slate-200/50 dark:border-slate-700/50 flex flex-col shadow-xl">
    <div class="p-6 border-b border-slate-200/50 dark:border-slate-700/50">
      <h2 class="text-xl font-bold text-slate-900 dark:text-slate-100 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">Browser</h2>
    </div>
    
    <nav class="flex-1 p-6 space-y-3">
      <router-link
        to="/"
        class="flex items-center px-4 py-3 text-sm font-semibold rounded-xl transition-all duration-200 transform hover:scale-105"
        :class="$route.path === '/' ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-lg shadow-blue-500/25' : 'text-slate-700 hover:bg-slate-100/80 dark:text-slate-300 dark:hover:bg-slate-700/50 hover:shadow-md'"
      >
        <GlobeAltIcon class="w-5 h-5 mr-3" />
        Browser
      </router-link>
      
      <router-link
        to="/bookmarks"
        class="flex items-center px-4 py-3 text-sm font-semibold rounded-xl transition-all duration-200 transform hover:scale-105"
        :class="$route.path === '/bookmarks' ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-lg shadow-blue-500/25' : 'text-slate-700 hover:bg-slate-100/80 dark:text-slate-300 dark:hover:bg-slate-700/50 hover:shadow-md'"
      >
        <BookmarkIcon class="w-5 h-5 mr-3" />
        Bookmarks
        <span class="ml-auto text-xs bg-white/20 dark:bg-slate-800/60 px-2.5 py-1 rounded-full font-medium backdrop-blur-sm">
          {{ bookmarksStore.bookmarks.length }}
        </span>
      </router-link>
      
      <router-link
        to="/history"
        class="flex items-center px-4 py-3 text-sm font-semibold rounded-xl transition-all duration-200 transform hover:scale-105"
        :class="$route.path === '/history' ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-lg shadow-blue-500/25' : 'text-slate-700 hover:bg-slate-100/80 dark:text-slate-300 dark:hover:bg-slate-700/50 hover:shadow-md'"
      >
        <ClockIcon class="w-5 h-5 mr-3" />
        History
      </router-link>
      
      <router-link
        to="/downloads"
        class="flex items-center px-4 py-3 text-sm font-semibold rounded-xl transition-all duration-200 transform hover:scale-105"
        :class="$route.path === '/downloads' ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-lg shadow-blue-500/25' : 'text-slate-700 hover:bg-slate-100/80 dark:text-slate-300 dark:hover:bg-slate-700/50 hover:shadow-md'"
      >
        <ArrowDownTrayIcon class="w-5 h-5 mr-3" />
        Downloads
        <span v-if="activeDownloads > 0" class="ml-auto text-xs bg-emerald-500/90 text-white px-2.5 py-1 rounded-full font-medium animate-pulse">
          {{ activeDownloads }}
        </span>
      </router-link>
      
      <router-link
        to="/dashboard"
        class="flex items-center px-4 py-3 text-sm font-semibold rounded-xl transition-all duration-200 transform hover:scale-105"
        :class="$route.path === '/dashboard' ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-lg shadow-blue-500/25' : 'text-slate-700 hover:bg-slate-100/80 dark:text-slate-300 dark:hover:bg-slate-700/50 hover:shadow-md'"
      >
        <ChartBarIcon class="w-5 h-5 mr-3" />
        Dashboard
      </router-link>
      
      <router-link
        to="/plugins"
        class="flex items-center px-4 py-3 text-sm font-semibold rounded-xl transition-all duration-200 transform hover:scale-105"
        :class="$route.path === '/plugins' ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-lg shadow-blue-500/25' : 'text-slate-700 hover:bg-slate-100/80 dark:text-slate-300 dark:hover:bg-slate-700/50 hover:shadow-md'"
      >
        <PuzzlePieceIcon class="w-5 h-5 mr-3" />
        Plugins
        <span class="ml-auto text-xs bg-white/20 dark:bg-slate-800/60 px-2.5 py-1 rounded-full font-medium backdrop-blur-sm">
          {{ enabledPlugins }}
        </span>
      </router-link>
      
      <router-link
        to="/sessions"
        class="flex items-center px-4 py-3 text-sm font-semibold rounded-xl transition-all duration-200 transform hover:scale-105"
        :class="$route.path === '/sessions' ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-lg shadow-blue-500/25' : 'text-slate-700 hover:bg-slate-100/80 dark:text-slate-300 dark:hover:bg-slate-700/50 hover:shadow-md'"
      >
        <RectangleStackIcon class="w-5 h-5 mr-3" />
        Sessions
      </router-link>
      
      <router-link
        to="/settings"
        class="flex items-center px-4 py-3 text-sm font-semibold rounded-xl transition-all duration-200 transform hover:scale-105"
        :class="$route.path === '/settings' ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-lg shadow-blue-500/25' : 'text-slate-700 hover:bg-slate-100/80 dark:text-slate-300 dark:hover:bg-slate-700/50 hover:shadow-md'"
      >
        <CogIcon class="w-5 h-5 mr-3" />
        Settings
      </router-link>
    </nav>
    
    <div class="p-6 border-t border-slate-200/50 dark:border-slate-700/50">
      <div class="bg-gradient-to-r from-emerald-50 to-green-50 dark:from-emerald-900/20 dark:to-green-900/20 p-4 rounded-xl border border-emerald-200/50 dark:border-emerald-700/50">
        <div class="flex items-center justify-between text-sm text-slate-700 dark:text-slate-300 mb-3">
          <span class="font-semibold flex items-center">
            <div class="w-2 h-2 bg-emerald-500 rounded-full mr-2 animate-pulse"></div>
            Privacy Shield
          </span>
          <div class="flex items-center space-x-1 bg-emerald-100/80 dark:bg-emerald-800/40 px-2.5 py-1 rounded-lg">
            <span class="text-xs font-bold text-emerald-700 dark:text-emerald-300">{{ totalBlocked }}</span>
          </div>
        </div>
        <div class="text-xs text-slate-600 dark:text-slate-400 font-medium">
          {{ formatBandwidthSaved }} saved
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import {
  GlobeAltIcon,
  BookmarkIcon,
  ClockIcon,
  ArrowDownTrayIcon,
  ChartBarIcon,
  PuzzlePieceIcon,
  RectangleStackIcon,
  CogIcon
} from '@heroicons/vue/24/outline'
import { useBookmarksStore } from '../stores/bookmarks'
import { useDownloadsStore } from '../stores/downloads'
import { usePluginsStore } from '../stores/plugins'
import { usePrivacyStore } from '../stores/privacy'

const bookmarksStore = useBookmarksStore()
const downloadsStore = useDownloadsStore()
const pluginsStore = usePluginsStore()
const privacyStore = usePrivacyStore()

const activeDownloads = computed(() => {
  return downloadsStore.downloads.filter(d => d.status === 'downloading').length
})

const enabledPlugins = computed(() => {
  return pluginsStore.plugins.filter(p => p.enabled).length
})

const totalBlocked = computed(() => {
  return privacyStore.getTotalBlocked()
})

const formatBandwidthSaved = computed(() => {
  return privacyStore.formatBandwidthSaved()
})

onMounted(async () => {
  try {
    await Promise.all([
      bookmarksStore.loadBookmarks(),
      downloadsStore.loadDownloads(),
      pluginsStore.loadPlugins(),
      privacyStore.loadBlockingStats()
    ])
  } catch (error) {
    console.error('Failed to load sidebar data:', error)
  }
})
</script>