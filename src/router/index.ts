import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Browser',
    component: () => import('../views/BrowserView.vue'),
    meta: {
      title: 'SW3DO Browser'
    }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('../views/SettingsView.vue'),
    meta: {
      title: 'Settings - SW3DO Browser'
    }
  },
  {
    path: '/bookmarks',
    name: 'Bookmarks',
    component: () => import('../views/BookmarksView.vue'),
    meta: {
      title: 'Bookmarks - SW3DO Browser'
    }
  },
  {
    path: '/history',
    name: 'History',
    component: () => import('../views/HistoryView.vue'),
    meta: {
      title: 'History - SW3DO Browser'
    }
  },
  {
    path: '/downloads',
    name: 'Downloads',
    component: () => import('../views/DownloadsView.vue'),
    meta: {
      title: 'Downloads - SW3DO Browser'
    }
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: () => import('../views/DashboardView.vue'),
    meta: {
      title: 'Privacy Dashboard - SW3DO Browser'
    }
  },
  {
    path: '/plugins',
    name: 'Plugins',
    component: () => import('../views/PluginsView.vue'),
    meta: {
      title: 'Plugins - SW3DO Browser'
    }
  },
  {
    path: '/sessions',
    name: 'Sessions',
    component: () => import('../views/SessionsView.vue'),
    meta: {
      title: 'Sessions - SW3DO Browser'
    }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach((to, _, next) => {
  if (to.meta?.title) {
    document.title = to.meta.title as string
  }
  next()
})

export default router