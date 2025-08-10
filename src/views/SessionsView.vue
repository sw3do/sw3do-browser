<template>
  <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="bg-white shadow rounded-lg">
        <div class="px-6 py-4 border-b border-gray-200">
          <div class="flex items-center justify-between">
            <div>
              <h1 class="text-2xl font-bold text-gray-900">Sessions</h1>
              <p class="mt-1 text-sm text-gray-600">Manage your browsing sessions and restore previous states</p>
            </div>
            <div class="flex items-center space-x-3">
              <button class="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                <BookmarkIcon class="-ml-1 mr-2 h-4 w-4" />
                Save Current Session
              </button>
              <button class="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                <PlusIcon class="-ml-1 mr-2 h-4 w-4" />
                New Session
              </button>
            </div>
          </div>
        </div>
        
        <div class="p-6">
          <div class="mb-6">
            <div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <div class="flex items-center">
                <div class="shrink-0">
                  <InformationCircleIcon class="h-5 w-5 text-blue-400" />
                </div>
                <div class="ml-3">
                  <h3 class="text-sm font-medium text-blue-800">Current Session</h3>
                  <div class="mt-2 text-sm text-blue-700">
                    <p>{{ currentSession.tabCount }} tabs open • Started {{ formatTime(currentSession.startedAt) }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <div class="mb-4">
            <div class="relative">
              <MagnifyingGlassIcon class="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-gray-400" />
              <input 
                type="text" 
                placeholder="Search sessions..." 
                class="pl-10 pr-4 py-2 w-full border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                v-model="searchQuery"
              >
            </div>
          </div>
          
          <div class="space-y-4">
            <div 
              v-for="session in filteredSessions" 
              :key="session.id"
              class="bg-white border border-gray-200 rounded-lg p-6 hover:shadow-md transition-shadow"
            >
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <div class="flex items-center">
                    <h3 class="text-lg font-medium text-gray-900">{{ session.name }}</h3>
                    <span v-if="session.isActive" class="ml-2 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                      Active
                    </span>
                  </div>
                  <p class="mt-1 text-sm text-gray-500">{{ session.description }}</p>
                  <div class="mt-2 flex items-center space-x-4 text-sm text-gray-500">
                    <div class="flex items-center">
                      <WindowIcon class="h-4 w-4 mr-1" />
                      {{ session.windowCount }} windows
                    </div>
                    <div class="flex items-center">
                      <RectangleStackIcon class="h-4 w-4 mr-1" />
                      {{ session.tabCount }} tabs
                    </div>
                    <div class="flex items-center">
                      <ClockIcon class="h-4 w-4 mr-1" />
                      {{ formatDate(session.createdAt) }}
                    </div>
                  </div>
                </div>
                <div class="flex items-center space-x-2">
                  <button 
                    v-if="!session.isActive"
                    class="inline-flex items-center px-3 py-1.5 border border-transparent text-sm font-medium rounded text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                  >
                    <ArrowPathIcon class="-ml-1 mr-1 h-4 w-4" />
                    Restore
                  </button>
                  <button class="text-gray-400 hover:text-gray-600" title="Edit">
                    <PencilIcon class="h-4 w-4" />
                  </button>
                  <button class="text-gray-400 hover:text-red-600" title="Delete">
                    <TrashIcon class="h-4 w-4" />
                  </button>
                </div>
              </div>
              
              <div class="mt-4">
                <div class="flex items-center justify-between mb-2">
                  <h4 class="text-sm font-medium text-gray-700">Recent Tabs</h4>
                  <button 
                    @click="session.showAllTabs = !session.showAllTabs"
                    class="text-xs text-blue-600 hover:text-blue-800"
                  >
                    {{ session.showAllTabs ? 'Show Less' : 'Show All' }}
                  </button>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2">
                  <div 
                    v-for="(tab, index) in session.showAllTabs ? session.tabs : session.tabs.slice(0, 6)" 
                    :key="index"
                    class="flex items-center p-2 bg-gray-50 rounded border hover:bg-gray-100 transition-colors"
                  >
                    <img :src="tab.favicon" :alt="tab.title" class="h-4 w-4 mr-2 rounded" onerror="this.src='data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTEyIDJMMTMuMDkgOC4yNkwyMCA5TDEzLjA5IDE1Ljc0TDEyIDIyTDEwLjkxIDE1Ljc0TDQgOUwxMC45MSA4LjI2TDEyIDJaIiBmaWxsPSIjOTQ5NEE0Ii8+Cjwvc3ZnPgo='">
                    <div class="flex-1 min-w-0">
                      <p class="text-xs font-medium text-gray-900 truncate">{{ tab.title }}</p>
                      <p class="text-xs text-gray-500 truncate">{{ tab.url }}</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <div v-if="filteredSessions.length === 0" class="text-center py-12">
            <RectangleStackIcon class="mx-auto h-12 w-12 text-gray-400" />
            <h3 class="mt-2 text-sm font-medium text-gray-900">No sessions found</h3>
            <p class="mt-1 text-sm text-gray-500">Create your first session to get started.</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { 
  BookmarkIcon,
  PlusIcon,
  InformationCircleIcon,
  MagnifyingGlassIcon,
  WindowIcon,
  RectangleStackIcon,
  ClockIcon,
  ArrowPathIcon,
  PencilIcon,
  TrashIcon
} from '@heroicons/vue/24/outline'

const searchQuery = ref('')

const currentSession = {
  tabCount: 8,
  startedAt: new Date('2024-01-15T09:30:00')
}

const sessions = ref([
  {
    id: 1,
    name: 'Work Session',
    description: 'Development and productivity tools',
    windowCount: 2,
    tabCount: 12,
    createdAt: new Date('2024-01-15T09:00:00'),
    isActive: true,
    showAllTabs: false,
    tabs: [
      { title: 'GitHub - sw3do/browser', url: 'https://github.com/sw3do/browser', favicon: 'https://github.com/favicon.ico' },
      { title: 'Stack Overflow', url: 'https://stackoverflow.com', favicon: 'https://stackoverflow.com/favicon.ico' },
      { title: 'Vue.js Documentation', url: 'https://vuejs.org/guide/', favicon: 'https://vuejs.org/favicon.ico' },
      { title: 'Tailwind CSS', url: 'https://tailwindcss.com', favicon: 'https://tailwindcss.com/favicon.ico' },
      { title: 'TypeScript Handbook', url: 'https://www.typescriptlang.org/docs/', favicon: 'https://www.typescriptlang.org/favicon.ico' },
      { title: 'MDN Web Docs', url: 'https://developer.mozilla.org', favicon: 'https://developer.mozilla.org/favicon.ico' }
    ]
  },
  {
    id: 2,
    name: 'Research Session',
    description: 'Articles and documentation for new features',
    windowCount: 1,
    tabCount: 8,
    createdAt: new Date('2024-01-14T14:30:00'),
    isActive: false,
    showAllTabs: false,
    tabs: [
      { title: 'Browser Architecture Overview', url: 'https://example.com/browser-arch', favicon: 'https://example.com/favicon.ico' },
      { title: 'Web Security Best Practices', url: 'https://example.com/security', favicon: 'https://example.com/favicon.ico' },
      { title: 'Performance Optimization', url: 'https://example.com/performance', favicon: 'https://example.com/favicon.ico' },
      { title: 'Privacy Features Implementation', url: 'https://example.com/privacy', favicon: 'https://example.com/favicon.ico' }
    ]
  },
  {
    id: 3,
    name: 'Design Session',
    description: 'UI/UX inspiration and design resources',
    windowCount: 1,
    tabCount: 6,
    createdAt: new Date('2024-01-13T16:45:00'),
    isActive: false,
    showAllTabs: false,
    tabs: [
      { title: 'Dribbble - Browser UI', url: 'https://dribbble.com/shots/browser-ui', favicon: 'https://dribbble.com/favicon.ico' },
      { title: 'Figma Community', url: 'https://figma.com/community', favicon: 'https://figma.com/favicon.ico' },
      { title: 'Material Design', url: 'https://material.io', favicon: 'https://material.io/favicon.ico' },
      { title: 'Apple Human Interface Guidelines', url: 'https://developer.apple.com/design/', favicon: 'https://developer.apple.com/favicon.ico' }
    ]
  },
  {
    id: 4,
    name: 'Testing Session',
    description: 'QA and testing resources',
    windowCount: 1,
    tabCount: 4,
    createdAt: new Date('2024-01-12T11:20:00'),
    isActive: false,
    showAllTabs: false,
    tabs: [
      { title: 'Jest Documentation', url: 'https://jestjs.io', favicon: 'https://jestjs.io/favicon.ico' },
      { title: 'Cypress Testing', url: 'https://cypress.io', favicon: 'https://cypress.io/favicon.ico' },
      { title: 'Testing Library', url: 'https://testing-library.com', favicon: 'https://testing-library.com/favicon.ico' },
      { title: 'Playwright', url: 'https://playwright.dev', favicon: 'https://playwright.dev/favicon.ico' }
    ]
  }
])

const filteredSessions = computed(() => {
  if (!searchQuery.value) {
    return sessions.value
  }
  
  return sessions.value.filter(session => 
    session.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    session.description.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    session.tabs.some(tab => 
      tab.title.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      tab.url.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
  )
})

const formatDate = (date: Date) => {
  return date.toLocaleDateString('en-US', { 
    year: 'numeric', 
    month: 'short', 
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const formatTime = (date: Date) => {
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const hours = Math.floor(diff / (1000 * 60 * 60))
  const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60))
  
  if (hours > 0) {
    return `${hours}h ${minutes}m ago`
  } else {
    return `${minutes}m ago`
  }
}
</script>