<template>
  <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="bg-white shadow rounded-lg">
        <div class="px-6 py-4 border-b border-gray-200">
          <h1 class="text-2xl font-bold text-gray-900">Settings</h1>
          <p class="mt-1 text-sm text-gray-600">Manage your browser preferences and configurations</p>
        </div>
        
        <div class="p-6">
          <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <div class="lg:col-span-1">
              <nav class="space-y-1">
                <button 
                  v-for="section in sections" 
                  :key="section.id"
                  @click="activeSection = section.id"
                  :class="[
                    activeSection === section.id
                      ? 'bg-blue-50 border-blue-500 text-blue-700'
                      : 'border-transparent text-gray-900 hover:bg-gray-50',
                    'group border-l-4 px-3 py-2 flex items-center text-sm font-medium w-full text-left'
                  ]"
                >
                  <component :is="section.icon" class="mr-3 h-5 w-5" />
                  {{ section.name }}
                </button>
              </nav>
            </div>
            
            <div class="lg:col-span-2">
              <div v-if="activeSection === 'general'" class="space-y-6">
                <div>
                  <h3 class="text-lg font-medium text-gray-900 mb-4">General Settings</h3>
                  <div class="space-y-4">
                    <div class="flex items-center justify-between">
                      <div>
                        <label class="text-sm font-medium text-gray-700">Default Search Engine</label>
                        <p class="text-sm text-gray-500">Choose your preferred search engine</p>
                      </div>
                      <select class="mt-1 block w-48 pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm rounded-md">
                        <option>Google</option>
                        <option>DuckDuckGo</option>
                        <option>Bing</option>
                      </select>
                    </div>
                    
                    <div class="flex items-center justify-between">
                      <div>
                        <label class="text-sm font-medium text-gray-700">Homepage</label>
                        <p class="text-sm text-gray-500">Set your default homepage</p>
                      </div>
                      <input type="url" class="mt-1 block w-48 px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm" placeholder="https://example.com">
                    </div>
                  </div>
                </div>
              </div>
              
              <div v-if="activeSection === 'privacy'" class="space-y-6">
                <div>
                  <h3 class="text-lg font-medium text-gray-900 mb-4">Privacy & Security</h3>
                  <div class="space-y-4">
                    <div class="flex items-center justify-between">
                      <div>
                        <label class="text-sm font-medium text-gray-700">Block Trackers</label>
                        <p class="text-sm text-gray-500">Prevent websites from tracking your activity</p>
                      </div>
                      <button type="button" class="bg-blue-600 relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-offset-2">
                        <span class="translate-x-5 pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"></span>
                      </button>
                    </div>
                    
                    <div class="flex items-center justify-between">
                      <div>
                        <label class="text-sm font-medium text-gray-700">Clear Cookies on Exit</label>
                        <p class="text-sm text-gray-500">Automatically clear cookies when closing browser</p>
                      </div>
                      <button type="button" class="bg-gray-200 relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-offset-2">
                        <span class="translate-x-0 pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"></span>
                      </button>
                    </div>
                  </div>
                </div>
              </div>
              
              <div v-if="activeSection === 'appearance'" class="space-y-6">
                <div>
                  <h3 class="text-lg font-medium text-gray-900 mb-4">Appearance</h3>
                  <div class="space-y-4">
                    <div class="flex items-center justify-between">
                      <div>
                        <label class="text-sm font-medium text-gray-700">Theme</label>
                        <p class="text-sm text-gray-500">Choose your preferred theme</p>
                      </div>
                      <select class="mt-1 block w-48 pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm rounded-md">
                        <option>Light</option>
                        <option>Dark</option>
                        <option>System</option>
                      </select>
                    </div>
                  </div>
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
import { ref } from 'vue'
import { CogIcon, ShieldCheckIcon, PaintBrushIcon } from '@heroicons/vue/24/outline'

const activeSection = ref('general')

const sections = [
  { id: 'general', name: 'General', icon: CogIcon },
  { id: 'privacy', name: 'Privacy & Security', icon: ShieldCheckIcon },
  { id: 'appearance', name: 'Appearance', icon: PaintBrushIcon }
]
</script>