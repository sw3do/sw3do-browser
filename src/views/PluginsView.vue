<template>
  <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="bg-white shadow rounded-lg">
        <div class="px-6 py-4 border-b border-gray-200">
          <div class="flex items-center justify-between">
            <div>
              <h1 class="text-2xl font-bold text-gray-900">Plugins</h1>
              <p class="mt-1 text-sm text-gray-600">Extend your browser with powerful plugins</p>
            </div>
            <button class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
              <PlusIcon class="-ml-1 mr-2 h-5 w-5" />
              Install Plugin
            </button>
          </div>
        </div>
        
        <div class="p-6">
          <div class="flex flex-col lg:flex-row gap-6">
            <div class="lg:w-1/4">
              <div class="bg-gray-50 rounded-lg p-4">
                <h3 class="text-sm font-medium text-gray-900 mb-3">Categories</h3>
                <nav class="space-y-1">
                  <button 
                    v-for="category in categories" 
                    :key="category.id"
                    @click="selectedCategory = category.id"
                    :class="[
                      selectedCategory === category.id
                        ? 'bg-blue-100 text-blue-700'
                        : 'text-gray-700 hover:bg-gray-100',
                      'group flex items-center px-2 py-2 text-sm font-medium rounded-md w-full text-left'
                    ]"
                  >
                    <component :is="category.icon" class="mr-3 h-5 w-5" />
                    {{ category.name }}
                    <span class="ml-auto text-xs text-gray-500">{{ category.count }}</span>
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
                    placeholder="Search plugins..." 
                    class="pl-10 pr-4 py-2 w-full border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    v-model="searchQuery"
                  >
                </div>
              </div>
              
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div 
                  v-for="plugin in filteredPlugins" 
                  :key="plugin.id"
                  class="bg-white border border-gray-200 rounded-lg p-6 hover:shadow-md transition-shadow"
                >
                  <div class="flex items-start justify-between">
                    <div class="flex items-center">
                      <div class="shrink-0">
                        <div class="h-12 w-12 rounded-lg flex items-center justify-center" :class="plugin.iconColor">
                          <component :is="plugin.icon" class="h-6 w-6 text-white" />
                        </div>
                      </div>
                      <div class="ml-4">
                        <h3 class="text-lg font-medium text-gray-900">{{ plugin.name }}</h3>
                        <p class="text-sm text-gray-500">by {{ plugin.author }}</p>
                      </div>
                    </div>
                    <div class="flex items-center space-x-2">
                      <span :class="getStatusBadgeClass(plugin.status)" class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium">
                        {{ plugin.status }}
                      </span>
                    </div>
                  </div>
                  
                  <p class="mt-4 text-sm text-gray-600">{{ plugin.description }}</p>
                  
                  <div class="mt-4 flex items-center justify-between">
                    <div class="flex items-center space-x-4 text-xs text-gray-500">
                      <div class="flex items-center">
                        <StarIcon class="h-4 w-4 text-yellow-400 mr-1" />
                        {{ plugin.rating }}
                      </div>
                      <div class="flex items-center">
                        <ArrowDownTrayIcon class="h-4 w-4 mr-1" />
                        {{ plugin.downloads }}
                      </div>
                      <div>v{{ plugin.version }}</div>
                    </div>
                    <div class="flex items-center space-x-2">
                      <button 
                        v-if="plugin.status === 'installed'"
                        class="inline-flex items-center px-3 py-1.5 border border-gray-300 shadow-sm text-xs font-medium rounded text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                      >
                        <CogIcon class="-ml-1 mr-1 h-3 w-3" />
                        Settings
                      </button>
                      <button 
                        v-if="plugin.status === 'installed'"
                        class="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                      >
                        <TrashIcon class="-ml-1 mr-1 h-3 w-3" />
                        Remove
                      </button>
                      <button 
                        v-else
                        class="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                      >
                        <ArrowDownTrayIcon class="-ml-1 mr-1 h-3 w-3" />
                        Install
                      </button>
                    </div>
                  </div>
                  
                  <div class="mt-4 flex flex-wrap gap-2">
                    <span 
                      v-for="tag in plugin.tags" 
                      :key="tag"
                      class="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-gray-100 text-gray-800"
                    >
                      {{ tag }}
                    </span>
                  </div>
                </div>
              </div>
              
              <div v-if="filteredPlugins.length === 0" class="text-center py-12">
                <PuzzlePieceIcon class="mx-auto h-12 w-12 text-gray-400" />
                <h3 class="mt-2 text-sm font-medium text-gray-900">No plugins found</h3>
                <p class="mt-1 text-sm text-gray-500">Try adjusting your search or category filter.</p>
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
  PlusIcon,
  MagnifyingGlassIcon,
  PuzzlePieceIcon,
  ShieldCheckIcon,
  EyeSlashIcon,
  PaintBrushIcon,
  CodeBracketIcon,
  StarIcon,
  ArrowDownTrayIcon,
  CogIcon,
  TrashIcon,
  WrenchScrewdriverIcon
} from '@heroicons/vue/24/outline'

const selectedCategory = ref('all')
const searchQuery = ref('')

const categories = [
  { id: 'all', name: 'All Plugins', icon: PuzzlePieceIcon, count: 12 },
  { id: 'privacy', name: 'Privacy & Security', icon: ShieldCheckIcon, count: 4 },
  { id: 'productivity', name: 'Productivity', icon: WrenchScrewdriverIcon, count: 3 },
  { id: 'appearance', name: 'Appearance', icon: PaintBrushIcon, count: 2 },
  { id: 'developer', name: 'Developer Tools', icon: CodeBracketIcon, count: 3 }
]

const plugins = [
  {
    id: 1,
    name: 'uBlock Origin',
    author: 'Raymond Hill',
    description: 'An efficient wide-spectrum content blocker. Easy on memory and CPU.',
    category: 'privacy',
    status: 'installed',
    rating: 4.8,
    downloads: '10M+',
    version: '1.45.2',
    icon: EyeSlashIcon,
    iconColor: 'bg-red-500',
    tags: ['Ad Blocker', 'Privacy', 'Security']
  },
  {
    id: 2,
    name: 'Dark Reader',
    author: 'Dark Reader Ltd',
    description: 'Dark mode for every website. Take care of your eyes, use dark theme for night and daily browsing.',
    category: 'appearance',
    status: 'installed',
    rating: 4.6,
    downloads: '5M+',
    version: '4.9.58',
    icon: PaintBrushIcon,
    iconColor: 'bg-gray-800',
    tags: ['Dark Mode', 'Theme', 'Accessibility']
  },
  {
    id: 3,
    name: 'Privacy Badger',
    author: 'Electronic Frontier Foundation',
    description: 'Automatically learns to block invisible trackers.',
    category: 'privacy',
    status: 'available',
    rating: 4.4,
    downloads: '2M+',
    version: '2023.1.17',
    icon: ShieldCheckIcon,
    iconColor: 'bg-green-500',
    tags: ['Privacy', 'Tracker Blocking', 'EFF']
  },
  {
    id: 4,
    name: 'React Developer Tools',
    author: 'Meta',
    description: 'Adds React debugging tools to the browser Developer Tools.',
    category: 'developer',
    status: 'available',
    rating: 4.7,
    downloads: '3M+',
    version: '4.27.1',
    icon: CodeBracketIcon,
    iconColor: 'bg-blue-500',
    tags: ['React', 'Developer', 'Debugging']
  },
  {
    id: 5,
    name: 'Grammarly',
    author: 'Grammarly Inc.',
    description: 'Write your best with Grammarly for Chrome.',
    category: 'productivity',
    status: 'available',
    rating: 4.5,
    downloads: '10M+',
    version: '14.1097.0',
    icon: WrenchScrewdriverIcon,
    iconColor: 'bg-green-600',
    tags: ['Writing', 'Grammar', 'Productivity']
  },
  {
    id: 6,
    name: 'LastPass',
    author: 'LastPass',
    description: 'LastPass, an award-winning password manager, saves your passwords and gives you secure access.',
    category: 'privacy',
    status: 'available',
    rating: 4.2,
    downloads: '8M+',
    version: '4.95.0',
    icon: ShieldCheckIcon,
    iconColor: 'bg-red-600',
    tags: ['Password Manager', 'Security', 'Privacy']
  }
]

const filteredPlugins = computed(() => {
  let filtered = plugins
  
  if (selectedCategory.value !== 'all') {
    filtered = filtered.filter(plugin => plugin.category === selectedCategory.value)
  }
  
  if (searchQuery.value) {
    filtered = filtered.filter(plugin => 
      plugin.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      plugin.description.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      plugin.tags.some(tag => tag.toLowerCase().includes(searchQuery.value.toLowerCase()))
    )
  }
  
  return filtered
})

const getStatusBadgeClass = (status: string) => {
  switch (status) {
    case 'installed': return 'bg-green-100 text-green-800'
    case 'available': return 'bg-gray-100 text-gray-800'
    case 'updating': return 'bg-blue-100 text-blue-800'
    default: return 'bg-gray-100 text-gray-800'
  }
}
</script>