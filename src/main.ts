import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import Home from './views/Home.vue'
import { i18n } from './i18n'
import './style.css'
import { initStorage, getItem, STORAGE_KEYS } from './services/storage'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Home },
    { path: '/onboarding', component: () => import('./views/Onboarding.vue') },
    { path: '/management', component: () => import('./views/Management.vue') },
    { path: '/scenery', redirect: '/management?tab=scenery' },
    { path: '/settings', component: () => import('./views/Settings.vue') },
  ],
})

// Initialize storage and setup navigation guard
async function initApp() {
  // Initialize Tauri Store first
  await initStorage()

  // Setup navigation guard with async storage check
  router.beforeEach(async (to, _from, next) => {
    const completed = await getItem<string>(STORAGE_KEYS.ONBOARDING_COMPLETED)
    if (completed !== 'true' && to.path !== '/onboarding') {
      next('/onboarding')
      return
    }
    next()
  })

  const pinia = createPinia()
  const app = createApp(App)

  app.use(pinia)
  app.use(router)
  app.use(i18n)

  // Initialize stores that need async loading
  const { useAppStore } = await import('./stores/app')
  const { useThemeStore } = await import('./stores/theme')
  const { useLockStore } = await import('./stores/lock')
  const { useUpdateStore } = await import('./stores/update')
  const { useSceneryStore } = await import('./stores/scenery')

  const appStore = useAppStore()
  const themeStore = useThemeStore()
  const lockStore = useLockStore()
  const updateStore = useUpdateStore()
  const sceneryStore = useSceneryStore()

  // Initialize all stores in parallel
  await Promise.all([
    appStore.initStore(),
    themeStore.initStore(),
    lockStore.initStore(),
    updateStore.initStore(),
    sceneryStore.initStore(),
  ])

  app.mount('#app')

  // Hide loading screen after Vue app is mounted
  setTimeout(() => {
    const loadingScreen = document.getElementById('loading-screen')
    if (loadingScreen) {
      loadingScreen.classList.add('fade-out')
      setTimeout(() => {
        loadingScreen.remove()
      }, 300)
    }
  }, 100)
}

initApp()
