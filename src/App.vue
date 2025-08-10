<script setup lang="ts">
</script>

<template>
  <div id="app" class="w-screen h-screen overflow-hidden bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800">
    <router-view />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  font-size: 14px;
  line-height: 1.5;
  color: theme('colors.slate.900');
  background-color: theme('colors.slate.50');
  font-feature-settings: 'cv02', 'cv03', 'cv04', 'cv11';
  font-optical-sizing: auto;
}

@media (prefers-color-scheme: dark) {
  body {
    color: theme('colors.slate.100');
    background-color: theme('colors.slate.900');
  }
}
</style>