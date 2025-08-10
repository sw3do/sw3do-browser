<template>
  <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="bg-white shadow rounded-lg">
        <div class="px-6 py-4 border-b border-gray-200">
          <div class="flex items-center justify-between">
            <div>
              <h1 class="text-2xl font-bold text-gray-900">Downloads</h1>
              <p class="mt-1 text-sm text-gray-600">Manage your downloaded files</p>
            </div>
            <div class="flex items-center space-x-3">
              <button class="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                <FolderOpenIcon class="-ml-1 mr-2 h-4 w-4" />
                Open Folder
              </button>
              <button class="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
                <TrashIcon class="-ml-1 mr-2 h-4 w-4" />
                Clear All
              </button>
            </div>
          </div>
        </div>
        
        <div class="p-6">
          <div class="flex flex-col lg:flex-row gap-6">
            <div class="lg:w-1/4">
              <div class="bg-gray-50 rounded-lg p-4">
                <h3 class="text-sm font-medium text-gray-900 mb-3">Filter by Status</h3>
                <nav class="space-y-1">
                  <button 
                    v-for="status in statusFilters" 
                    :key="status.id"
                    @click="selectedStatus = status.id"
                    :class="[
                      selectedStatus === status.id
                        ? 'bg-blue-100 text-blue-700'
                        : 'text-gray-700 hover:bg-gray-100',
                      'group flex items-center px-2 py-2 text-sm font-medium rounded-md w-full text-left'
                    ]"
                  >
                    <component :is="status.icon" class="mr-3 h-5 w-5" />
                    {{ status.name }}
                    <span class="ml-auto text-xs text-gray-500">{{ status.count }}</span>
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
                    placeholder="Search downloads..." 
                    class="pl-10 pr-4 py-2 w-full border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    v-model="searchQuery"
                  >
                </div>
              </div>
              
              <div class="space-y-3">
                <div 
                  v-for="download in filteredDownloads" 
                  :key="download.id"
                  class="flex items-center p-4 bg-white border border-gray-200 rounded-lg hover:shadow-md transition-shadow"
                >
                  <div class="shrink-0">
                    <div class="h-10 w-10 rounded-lg flex items-center justify-center" :class="getFileTypeColor(download.type)">
                      <component :is="getFileIcon(download.type)" class="h-6 w-6 text-white" />
                    </div>
                  </div>
                  <div class="ml-4 flex-1 min-w-0">
                    <div class="flex items-center justify-between">
                      <h4 class="text-sm font-medium text-gray-900 truncate">{{ download.filename }}</h4>
                      <div class="flex items-center space-x-2">
                        <span :class="getStatusBadgeClass(download.status)" class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium">
                          {{ download.status }}
                        </span>
                        <div class="flex items-center space-x-1">
                          <button v-if="download.status === 'completed'" class="text-gray-400 hover:text-blue-600" title="Open">
                            <FolderOpenIcon class="h-4 w-4" />
                          </button>
                          <button v-if="download.status === 'downloading'" class="text-gray-400 hover:text-red-600" title="Cancel">
                            <XMarkIcon class="h-4 w-4" />
                          </button>
                          <button class="text-gray-400 hover:text-red-600" title="Remove">
                            <TrashIcon class="h-4 w-4" />
                          </button>
                        </div>
                      </div>
                    </div>
                    <p class="text-sm text-gray-500 truncate">{{ download.url }}</p>
                    <div class="flex items-center justify-between mt-2">
                      <div class="flex items-center text-xs text-gray-500">
                        <span>{{ formatFileSize(download.size) }}</span>
                        <span class="mx-2">•</span>
                        <span>{{ formatDate(download.startedAt) }}</span>
                      </div>
                      <div v-if="download.status === 'downloading'" class="flex items-center">
                        <div class="w-32 bg-gray-200 rounded-full h-2 mr-2">
                          <div class="bg-blue-600 h-2 rounded-full transition-all duration-300" :style="{ width: download.progress + '%' }"></div>
                        </div>
                        <span class="text-xs text-gray-500">{{ download.progress }}%</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              
              <div v-if="filteredDownloads.length === 0" class="text-center py-12">
                <ArrowDownTrayIcon class="mx-auto h-12 w-12 text-gray-400" />
                <h3 class="mt-2 text-sm font-medium text-gray-900">No downloads found</h3>
                <p class="mt-1 text-sm text-gray-500">Your downloaded files will appear here.</p>
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
  FolderOpenIcon,
  TrashIcon,
  MagnifyingGlassIcon,
  ArrowDownTrayIcon,
  XMarkIcon,
  CheckCircleIcon,
  ClockIcon,
  ExclamationTriangleIcon,
  DocumentIcon,
  PhotoIcon,
  VideoCameraIcon,
  MusicalNoteIcon,
  ArchiveBoxIcon
} from '@heroicons/vue/24/outline'

const selectedStatus = ref('all')
const searchQuery = ref('')

const statusFilters = [
  { id: 'all', name: 'All Downloads', icon: ArrowDownTrayIcon, count: 8 },
  { id: 'completed', name: 'Completed', icon: CheckCircleIcon, count: 5 },
  { id: 'downloading', name: 'Downloading', icon: ClockIcon, count: 2 },
  { id: 'failed', name: 'Failed', icon: ExclamationTriangleIcon, count: 1 }
]

const downloads = [
  {
    id: 1,
    filename: 'sw3do-browser-setup.dmg',
    url: 'https://github.com/sw3do/browser/releases/download/v1.0.0/sw3do-browser-setup.dmg',
    size: 125829120,
    type: 'application',
    status: 'completed',
    progress: 100,
    startedAt: new Date('2024-01-15T14:30:00')
  },
  {
    id: 2,
    filename: 'presentation.pdf',
    url: 'https://example.com/files/presentation.pdf',
    size: 5242880,
    type: 'document',
    status: 'completed',
    progress: 100,
    startedAt: new Date('2024-01-15T13:45:00')
  },
  {
    id: 3,
    filename: 'vacation-photos.zip',
    url: 'https://example.com/files/vacation-photos.zip',
    size: 52428800,
    type: 'archive',
    status: 'downloading',
    progress: 67,
    startedAt: new Date('2024-01-15T15:20:00')
  },
  {
    id: 4,
    filename: 'music-album.mp3',
    url: 'https://example.com/files/music-album.mp3',
    size: 10485760,
    type: 'audio',
    status: 'downloading',
    progress: 23,
    startedAt: new Date('2024-01-15T15:45:00')
  },
  {
    id: 5,
    filename: 'video-tutorial.mp4',
    url: 'https://example.com/files/video-tutorial.mp4',
    size: 104857600,
    type: 'video',
    status: 'failed',
    progress: 0,
    startedAt: new Date('2024-01-15T12:30:00')
  }
]

const filteredDownloads = computed(() => {
  let filtered = downloads
  
  if (selectedStatus.value !== 'all') {
    filtered = filtered.filter(download => download.status === selectedStatus.value)
  }
  
  if (searchQuery.value) {
    filtered = filtered.filter(download => 
      download.filename.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      download.url.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
  }
  
  return filtered.sort((a, b) => b.startedAt.getTime() - a.startedAt.getTime())
})

const getFileIcon = (type: string) => {
  switch (type) {
    case 'image': return PhotoIcon
    case 'video': return VideoCameraIcon
    case 'audio': return MusicalNoteIcon
    case 'archive': return ArchiveBoxIcon
    case 'document': return DocumentIcon
    default: return DocumentIcon
  }
}

const getFileTypeColor = (type: string) => {
  switch (type) {
    case 'image': return 'bg-green-500'
    case 'video': return 'bg-red-500'
    case 'audio': return 'bg-purple-500'
    case 'archive': return 'bg-yellow-500'
    case 'document': return 'bg-blue-500'
    default: return 'bg-gray-500'
  }
}

const getStatusBadgeClass = (status: string) => {
  switch (status) {
    case 'completed': return 'bg-green-100 text-green-800'
    case 'downloading': return 'bg-blue-100 text-blue-800'
    case 'failed': return 'bg-red-100 text-red-800'
    default: return 'bg-gray-100 text-gray-800'
  }
}

const formatFileSize = (bytes: number) => {
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  if (bytes === 0) return '0 Bytes'
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i]
}

const formatDate = (date: Date) => {
  return date.toLocaleDateString('en-US', { 
    year: 'numeric', 
    month: 'short', 
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>