<template>
  <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="bg-white shadow rounded-lg">
        <div class="px-6 py-4 border-b border-gray-200">
          <div class="flex items-center justify-between">
            <div>
              <h1 class="text-2xl font-bold text-gray-900">Browsing History</h1>
              <p class="mt-1 text-sm text-gray-600">View and manage your browsing history</p>
            </div>
            <div class="flex items-center space-x-3">
              <button class="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                <FunnelIcon class="-ml-1 mr-2 h-4 w-4" />
                Filter
              </button>
              <button class="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
                <TrashIcon class="-ml-1 mr-2 h-4 w-4" />
                Clear History
              </button>
            </div>
          </div>
        </div>
        
        <div class="p-6">
          <div class="flex flex-col lg:flex-row gap-6">
            <div class="lg:w-1/4">
              <div class="bg-gray-50 rounded-lg p-4">
                <h3 class="text-sm font-medium text-gray-900 mb-3">Time Range</h3>
                <nav class="space-y-1">
                  <button 
                    v-for="range in timeRanges" 
                    :key="range.id"
                    @click="selectedRange = range.id"
                    :class="[
                      selectedRange === range.id
                        ? 'bg-blue-100 text-blue-700'
                        : 'text-gray-700 hover:bg-gray-100',
                      'group flex items-center px-2 py-2 text-sm font-medium rounded-md w-full text-left'
                    ]"
                  >
                    <component :is="range.icon" class="mr-3 h-5 w-5" />
                    {{ range.name }}
                    <span class="ml-auto text-xs text-gray-500">{{ range.count }}</span>
                  </button>
                </nav>
              </div>
            </div>
            
            <div class="lg:w-3/4">
              <div class="mb-4">
                <div class="relative">
                  <MagnifyingGlassIcon class="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-gray-400" />
                  <input 
                    type="text" 
                    placeholder="Search history..." 
                    class="pl-10 pr-4 py-2 w-full border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    v-model="searchQuery"
                  >
                </div>
              </div>
              
              <div class="space-y-6">
                <div v-for="(group, date) in groupedHistory" :key="date">
                  <h3 class="text-sm font-medium text-gray-900 mb-3 sticky top-0 bg-white py-2">{{ formatDateGroup(date) }}</h3>
                  <div class="space-y-2">
                    <div 
                      v-for="item in group" 
                      :key="item.id"
                      class="flex items-center p-3 bg-white border border-gray-200 rounded-lg hover:shadow-md transition-shadow group"
                    >
                      <div class="flex-shrink-0">
                        <img :src="item.favicon" :alt="item.title" class="h-6 w-6 rounded" onerror="this.src='data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTEyIDJMMTMuMDkgOC4yNkwyMCA5TDEzLjA5IDE1Ljc0TDEyIDIyTDEwLjkxIDE1Ljc0TDQgOUwxMC45MSA4LjI2TDEyIDJaIiBmaWxsPSIjOTQ5NEE0Ii8+Cjwvc3ZnPgo='">
                      </div>
                      <div class="ml-4 flex-1 min-w-0">
                        <div class="flex items-center justify-between">
                          <h4 class="text-sm font-medium text-gray-900 truncate group-hover:text-blue-600">{{ item.title }}</h4>
                          <div class="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity">
                            <button class="text-gray-400 hover:text-blue-600" title="Visit">
                              <ArrowTopRightOnSquareIcon class="h-4 w-4" />
                            </button>
                            <button class="text-gray-400 hover:text-red-600" title="Remove">
                              <XMarkIcon class="h-4 w-4" />
                            </button>
                          </div>
                        </div>
                        <p class="text-sm text-gray-500 truncate">{{ item.url }}</p>
                        <div class="flex items-center mt-1">
                          <ClockIcon class="h-3 w-3 text-gray-400 mr-1" />
                          <span class="text-xs text-gray-500">{{ formatTime(item.visitedAt) }}</span>
                          <span class="mx-2 text-gray-300">•</span>
                          <span class="text-xs text-gray-500">{{ item.visitCount }} visits</span>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              
              <div v-if="Object.keys(groupedHistory).length === 0" class="text-center py-12">
                <ClockIcon class="mx-auto h-12 w-12 text-gray-400" />
                <h3 class="mt-2 text-sm font-medium text-gray-900">No history found</h3>
                <p class="mt-1 text-sm text-gray-500">Your browsing history will appear here.</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { 
  FunnelIcon,
  TrashIcon,
  MagnifyingGlassIcon,
  ClockIcon,
  CalendarDaysIcon,
  ArrowTopRightOnSquareIcon,
  XMarkIcon
} from '@heroicons/vue/24/outline'

const selectedRange = ref('today')
const searchQuery = ref('')

const timeRanges = [
  { id: 'today', name: 'Today', icon: ClockIcon, count: 15 },
  { id: 'yesterday', name: 'Yesterday', icon: ClockIcon, count: 23 },
  { id: 'week', name: 'This Week', icon: CalendarDaysIcon, count: 89 },
  { id: 'month', name: 'This Month', icon: CalendarDaysIcon, count: 234 }
]

const historyItems = [
  {
    id: 1,
    title: 'GitHub - sw3do/browser: Modern web browser',
    url: 'https://github.com/sw3do/browser',
    favicon: 'https://github.com/favicon.ico',
    visitedAt: new Date('2024-01-15T14:30:00'),
    visitCount: 5
  },
  {
    id: 2,
    title: 'Tailwind CSS - Rapidly build modern websites',
    url: 'https://tailwindcss.com',
    favicon: 'https://tailwindcss.com/favicon.ico',
    visitedAt: new Date('2024-01-15T13:45:00'),
    visitCount: 3
  },
  {
    id: 3,
    title: 'Vue.js - The Progressive JavaScript Framework',
    url: 'https://vuejs.org',
    favicon: 'https://vuejs.org/favicon.ico',
    visitedAt: new Date('2024-01-15T12:20:00'),
    visitCount: 8
  },
  {
    id: 4,
    title: 'Stack Overflow - Where Developers Learn',
    url: 'https://stackoverflow.com',
    favicon: 'https://stackoverflow.com/favicon.ico',
    visitedAt: new Date('2024-01-14T16:15:00'),
    visitCount: 12
  }
]

const filteredHistory = computed(() => {
  let filtered = historyItems
  
  if (searchQuery.value) {
    filtered = filtered.filter(item => 
      item.title.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      item.url.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
  }
  
  const now = new Date()
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
  const yesterday = new Date(today.getTime() - 24 * 60 * 60 * 1000)
  const weekAgo = new Date(today.getTime() - 7 * 24 * 60 * 60 * 1000)
  const monthAgo = new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000)
  
  switch (selectedRange.value) {
    case 'today':
      filtered = filtered.filter(item => item.visitedAt >= today)
      break
    case 'yesterday':
      filtered = filtered.filter(item => item.visitedAt >= yesterday && item.visitedAt < today)
      break
    case 'week':
      filtered = filtered.filter(item => item.visitedAt >= weekAgo)
      break
    case 'month':
      filtered = filtered.filter(item => item.visitedAt >= monthAgo)
      break
  }
  
  return filtered
})

const groupedHistory = computed(() => {
  const groups: Record<string, typeof historyItems> = {}
  
  filteredHistory.value.forEach(item => {
    const date = item.visitedAt.toDateString()
    if (!groups[date]) {
      groups[date] = []
    }
    groups[date].push(item)
  })
  
  Object.keys(groups).forEach(date => {
    groups[date].sort((a, b) => b.visitedAt.getTime() - a.visitedAt.getTime())
  })
  
  return groups
})

const formatDateGroup = (dateString: string) => {
  const date = new Date(dateString)
  const today = new Date()
  const yesterday = new Date(today.getTime() - 24 * 60 * 60 * 1000)
  
  if (date.toDateString() === today.toDateString()) {
    return 'Today'
  } else if (date.toDateString() === yesterday.toDateString()) {
    return 'Yesterday'
  } else {
    return date.toLocaleDateString('en-US', { 
      weekday: 'long',
      year: 'numeric', 
      month: 'long', 
      day: 'numeric' 
    })
  }
}

const formatTime = (date: Date) => {
  return date.toLocaleTimeString('en-US', { 
    hour: '2-digit', 
    minute: '2-digit'
  })
}
</script>