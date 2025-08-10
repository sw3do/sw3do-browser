<template>
  <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-900">Privacy Dashboard</h1>
        <p class="mt-2 text-gray-600">Monitor and control your privacy settings and data protection</p>
      </div>
      
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex items-center">
            <div class="shrink-0">
              <ShieldCheckIcon class="h-8 w-8 text-green-500" />
            </div>
            <div class="ml-4">
              <h3 class="text-lg font-medium text-gray-900">Trackers Blocked</h3>
              <p class="text-2xl font-bold text-green-600">{{ blockingStats.trackers_blocked.toLocaleString() }}</p>
              <p class="text-sm text-gray-500">Total</p>
            </div>
          </div>
        </div>
        
        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex items-center">
            <div class="shrink-0">
              <EyeSlashIcon class="h-8 w-8 text-blue-500" />
            </div>
            <div class="ml-4">
              <h3 class="text-lg font-medium text-gray-900">Ads Blocked</h3>
              <p class="text-2xl font-bold text-blue-600">{{ blockingStats.ads_blocked.toLocaleString() }}</p>
              <p class="text-sm text-gray-500">Total</p>
            </div>
          </div>
        </div>
        
        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex items-center">
            <div class="shrink-0">
              <LockClosedIcon class="h-8 w-8 text-purple-500" />
            </div>
            <div class="ml-4">
              <h3 class="text-lg font-medium text-gray-900">HTTPS Upgrades</h3>
              <p class="text-2xl font-bold text-purple-600">{{ blockingStats.https_upgrades.toLocaleString() }}</p>
              <p class="text-sm text-gray-500">Total</p>
            </div>
          </div>
        </div>
      </div>
      
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div class="bg-white rounded-lg shadow">
          <div class="px-6 py-4 border-b border-gray-200">
            <h2 class="text-lg font-medium text-gray-900">Privacy Protection Status</h2>
          </div>
          <div class="p-6">
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <div class="flex items-center">
                  <ShieldCheckIcon class="h-5 w-5 text-green-500 mr-3" />
                  <span class="text-sm font-medium text-gray-900">Tracker Protection</span>
                </div>
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium" :class="privacySettings.global_tracker_blocking ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'">
                  {{ privacySettings.global_tracker_blocking ? 'Active' : 'Inactive' }}
                </span>
              </div>
              
              <div class="flex items-center justify-between">
                <div class="flex items-center">
                  <EyeSlashIcon class="h-5 w-5 text-blue-500 mr-3" />
                  <span class="text-sm font-medium text-gray-900">Ad Blocker</span>
                </div>
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium" :class="privacySettings.global_ad_blocking ? 'bg-blue-100 text-blue-800' : 'bg-red-100 text-red-800'">
                  {{ privacySettings.global_ad_blocking ? 'Active' : 'Inactive' }}
                </span>
              </div>
              
              <div class="flex items-center justify-between">
                <div class="flex items-center">
                  <LockClosedIcon class="h-5 w-5 text-purple-500 mr-3" />
                  <span class="text-sm font-medium text-gray-900">HTTPS Everywhere</span>
                </div>
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium" :class="privacySettings.global_https_only ? 'bg-purple-100 text-purple-800' : 'bg-red-100 text-red-800'">
                  {{ privacySettings.global_https_only ? 'Active' : 'Inactive' }}
                </span>
              </div>
              
              <div class="flex items-center justify-between">
                <div class="flex items-center">
                  <GlobeAltIcon class="h-5 w-5 text-yellow-500 mr-3" />
                  <span class="text-sm font-medium text-gray-900">DNS over HTTPS</span>
                </div>
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                  Active
                </span>
              </div>
              
              <div class="flex items-center justify-between">
                <div class="flex items-center">
                  <UserIcon class="h-5 w-5 text-red-500 mr-3" />
                  <span class="text-sm font-medium text-gray-900">Fingerprint Protection</span>
                </div>
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium" :class="privacySettings.global_fingerprinting_protection ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'">
                  {{ privacySettings.global_fingerprinting_protection ? 'Active' : 'Inactive' }}
                </span>
              </div>
            </div>
          </div>
        </div>
        
        <div class="bg-white rounded-lg shadow">
          <div class="px-6 py-4 border-b border-gray-200">
            <h2 class="text-lg font-medium text-gray-900">Recent Activity</h2>
          </div>
          <div class="p-6">
            <div class="flow-root">
              <ul class="-mb-8">
                <li v-for="(activity, index) in recentActivities" :key="activity.id">
                  <div class="relative pb-8" :class="{ 'pb-0': index === recentActivities.length - 1 }">
                    <span v-if="index !== recentActivities.length - 1" class="absolute top-4 left-4 -ml-px h-full w-0.5 bg-gray-200"></span>
                    <div class="relative flex space-x-3">
                      <div>
                        <span :class="getActivityIconClass(activity.type)" class="h-8 w-8 rounded-full flex items-center justify-center ring-8 ring-white">
                          <component :is="getActivityIcon(activity.type)" class="h-4 w-4 text-white" />
                        </span>
                      </div>
                      <div class="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                        <div>
                          <p class="text-sm text-gray-900">{{ activity.description }}</p>
                          <p class="text-xs text-gray-500">{{ activity.domain }}</p>
                        </div>
                        <div class="text-right text-xs whitespace-nowrap text-gray-500">
                          {{ formatTime(activity.timestamp) }}
                        </div>
                      </div>
                    </div>
                  </div>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
      
      <div class="mt-6 bg-white rounded-lg shadow">
        <div class="px-6 py-4 border-b border-gray-200">
          <h2 class="text-lg font-medium text-gray-900">Top Blocked Domains</h2>
        </div>
        <div class="p-6">
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <div v-for="domain in topBlockedDomains" :key="domain.name" class="bg-gray-50 rounded-lg p-4">
              <div class="flex items-center justify-between">
                <div>
                  <h4 class="text-sm font-medium text-gray-900">{{ domain.name }}</h4>
                  <p class="text-xs text-gray-500">{{ domain.category }}</p>
                </div>
                <div class="text-right">
                  <p class="text-lg font-bold text-red-600">{{ domain.blocked }}</p>
                  <p class="text-xs text-gray-500">blocked</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { 
  ShieldCheckIcon,
  EyeSlashIcon,
  LockClosedIcon,
  GlobeAltIcon,
  UserIcon,
  ExclamationTriangleIcon
} from '@heroicons/vue/24/outline'
import { usePrivacyStore } from '../stores/privacy'

const privacyStore = usePrivacyStore()

const recentActivities = ref([
  {
    id: 1,
    type: 'tracker_blocked',
    description: 'Blocked tracking script',
    domain: 'google-analytics.com',
    timestamp: new Date('2025-01-15T15:30:00')
  },
  {
    id: 2,
    type: 'ad_blocked',
    description: 'Blocked advertisement',
    domain: 'doubleclick.net',
    timestamp: new Date('2025-01-15T15:25:00')
  },
  {
    id: 3,
    type: 'https_upgrade',
    description: 'Upgraded to HTTPS',
    domain: 'example.com',
    timestamp: new Date('2025-01-15T15:20:00')
  },
  {
    id: 4,
    type: 'fingerprint_blocked',
    description: 'Blocked fingerprinting attempt',
    domain: 'facebook.com',
    timestamp: new Date('2025-01-15T15:15:00')
  },
  {
    id: 5,
    type: 'tracker_blocked',
    description: 'Blocked social media tracker',
    domain: 'twitter.com',
    timestamp: new Date('2025-01-15T15:10:00')
  }
])

const topBlockedDomains = ref([
  { name: 'google-analytics.com', category: 'Analytics', blocked: 89 },
  { name: 'doubleclick.net', category: 'Advertising', blocked: 67 },
  { name: 'facebook.com', category: 'Social Media', blocked: 45 },
  { name: 'googlesyndication.com', category: 'Advertising', blocked: 34 },
  { name: 'amazon-adsystem.com', category: 'Advertising', blocked: 28 },
  { name: 'twitter.com', category: 'Social Media', blocked: 23 }
])

const blockingStats = computed(() => privacyStore.blockingStats)
const privacySettings = computed(() => privacyStore.privacySettings)

const getActivityIcon = (type: string) => {
  switch (type) {
    case 'tracker_blocked': return ShieldCheckIcon
    case 'ad_blocked': return EyeSlashIcon
    case 'https_upgrade': return LockClosedIcon
    case 'fingerprint_blocked': return UserIcon
    default: return ExclamationTriangleIcon
  }
}

const getActivityIconClass = (type: string) => {
  switch (type) {
    case 'tracker_blocked': return 'bg-green-500'
    case 'ad_blocked': return 'bg-blue-500'
    case 'https_upgrade': return 'bg-purple-500'
    case 'fingerprint_blocked': return 'bg-red-500'
    default: return 'bg-gray-500'
  }
}

const formatTime = (date: Date) => {
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / 60000)
  
  if (minutes < 1) return 'Just now'
  if (minutes < 60) return `${minutes}m ago`
  
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours}h ago`
  
  const days = Math.floor(hours / 24)
  return `${days}d ago`
}

onMounted(async () => {
  try {
    await privacyStore.loadBlockingStats()
    await privacyStore.loadPrivacySettings()
  } catch (error) {
    console.error('Failed to load dashboard data:', error)
  }
})
</script>