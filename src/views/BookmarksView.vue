<template>
  <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="bg-white shadow rounded-lg">
        <div class="px-6 py-4 border-b border-gray-200">
          <div class="flex items-center justify-between">
            <div>
              <h1 class="text-2xl font-bold text-gray-900">Bookmarks</h1>
              <p class="mt-1 text-sm text-gray-600">Manage your saved bookmarks and folders</p>
            </div>
            <button class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
              <PlusIcon class="-ml-1 mr-2 h-5 w-5" />
              Add Bookmark
            </button>
          </div>
        </div>
        
        <div class="p-6">
          <div class="flex flex-col lg:flex-row gap-6">
            <div class="lg:w-1/4">
              <div class="bg-gray-50 rounded-lg p-4">
                <h3 class="text-sm font-medium text-gray-900 mb-3">Folders</h3>
                <nav class="space-y-1">
                  <button 
                    v-for="folder in folders" 
                    :key="folder.id"
                    @click="selectedFolder = folder.id"
                    :class="[
                      selectedFolder === folder.id
                        ? 'bg-blue-100 text-blue-700'
                        : 'text-gray-700 hover:bg-gray-100',
                      'group flex items-center px-2 py-2 text-sm font-medium rounded-md w-full text-left'
                    ]"
                  >
                    <FolderIcon class="mr-3 h-5 w-5" />
                    {{ folder.name }}
                    <span class="ml-auto text-xs text-gray-500">{{ folder.count }}</span>
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
                    placeholder="Search bookmarks..." 
                    class="pl-10 pr-4 py-2 w-full border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    v-model="searchQuery"
                  >
                </div>
              </div>
              
              <div class="space-y-3">
                <div 
                  v-for="bookmark in filteredBookmarks" 
                  :key="bookmark.id"
                  class="flex items-center p-4 bg-white border border-gray-200 rounded-lg hover:shadow-md transition-shadow"
                >
                  <div class="shrink-0">
                    <img :src="bookmark.favicon" :alt="bookmark.title" class="h-6 w-6 rounded" onerror="this.src='data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTEyIDJMMTMuMDkgOC4yNkwyMCA5TDEzLjA5IDE1Ljc0TDEyIDIyTDEwLjkxIDE1Ljc0TDQgOUwxMC45MSA4LjI2TDEyIDJaIiBmaWxsPSIjOTQ5NEE0Ii8+Cjwvc3ZnPgo='">
                  </div>
                  <div class="ml-4 flex-1 min-w-0">
                    <div class="flex items-center justify-between">
                      <h4 class="text-sm font-medium text-gray-900 truncate">{{ bookmark.title }}</h4>
                      <div class="flex items-center space-x-2">
                        <button class="text-gray-400 hover:text-gray-600">
                          <PencilIcon class="h-4 w-4" />
                        </button>
                        <button class="text-gray-400 hover:text-red-600">
                          <TrashIcon class="h-4 w-4" />
                        </button>
                      </div>
                    </div>
                    <p class="text-sm text-gray-500 truncate">{{ bookmark.url }}</p>
                    <div class="flex items-center mt-2">
                      <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
                        {{ bookmark.folder }}
                      </span>
                      <span class="ml-2 text-xs text-gray-500">{{ formatDate(bookmark.createdAt) }}</span>
                    </div>
                  </div>
                </div>
              </div>
              
              <div v-if="filteredBookmarks.length === 0" class="text-center py-12">
                <BookmarkIcon class="mx-auto h-12 w-12 text-gray-400" />
                <h3 class="mt-2 text-sm font-medium text-gray-900">No bookmarks found</h3>
                <p class="mt-1 text-sm text-gray-500">Get started by adding your first bookmark.</p>
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
  FolderIcon, 
  MagnifyingGlassIcon, 
  BookmarkIcon,
  PencilIcon,
  TrashIcon
} from '@heroicons/vue/24/outline'

const selectedFolder = ref('all')
const searchQuery = ref('')

const folders = [
  { id: 'all', name: 'All Bookmarks', count: 12 },
  { id: 'work', name: 'Work', count: 5 },
  { id: 'personal', name: 'Personal', count: 4 },
  { id: 'development', name: 'Development', count: 3 }
]

const bookmarks = [
  {
    id: 1,
    title: 'GitHub',
    url: 'https://github.com',
    favicon: 'https://github.com/favicon.ico',
    folder: 'Development',
    createdAt: new Date('2024-01-15')
  },
  {
    id: 2,
    title: 'Stack Overflow',
    url: 'https://stackoverflow.com',
    favicon: 'https://stackoverflow.com/favicon.ico',
    folder: 'Development',
    createdAt: new Date('2024-01-10')
  },
  {
    id: 3,
    title: 'Gmail',
    url: 'https://gmail.com',
    favicon: 'https://gmail.com/favicon.ico',
    folder: 'Work',
    createdAt: new Date('2024-01-08')
  }
]

const filteredBookmarks = computed(() => {
  let filtered = bookmarks
  
  if (selectedFolder.value !== 'all') {
    const folderName = folders.find(f => f.id === selectedFolder.value)?.name
    filtered = filtered.filter(bookmark => bookmark.folder === folderName)
  }
  
  if (searchQuery.value) {
    filtered = filtered.filter(bookmark => 
      bookmark.title.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      bookmark.url.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
  }
  
  return filtered
})

const formatDate = (date: Date) => {
  return date.toLocaleDateString('en-US', { 
    year: 'numeric', 
    month: 'short', 
    day: 'numeric' 
  })
}
</script>