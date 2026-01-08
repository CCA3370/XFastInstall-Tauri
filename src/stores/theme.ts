import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

export const useThemeStore = defineStore('theme', () => {
  // Initialize from localStorage or system preference
  const isDark = ref(localStorage.getItem('theme') === 'dark' ||
    (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches))

  function toggleTheme() {
    isDark.value = !isDark.value
  }

  async function applyTheme() {
    if (isDark.value) {
      document.documentElement.classList.add('dark')
      localStorage.setItem('theme', 'dark')
    } else {
      document.documentElement.classList.remove('dark')
      localStorage.setItem('theme', 'light')
    }

    // Sync Tauri window theme with app theme
    try {
      const appWindow = getCurrentWindow()
      await appWindow.setTheme(isDark.value ? 'dark' : 'light')
    } catch (e) {
      // Ignore errors (e.g., running in browser during dev)
      console.debug('Failed to set window theme:', e)
    }
  }

  // Watch for changes and apply
  watch(isDark, applyTheme, { immediate: true })

  return {
    isDark,
    toggleTheme
  }
})
