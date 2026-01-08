import { createI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import zh from './zh'
import en from './en'

// 获取系统语言
const getSystemLanguage = (): string => {
  const lang = navigator.language || 'en'
  return lang.startsWith('zh') ? 'zh' : 'en'
}

const initialLocale = getSystemLanguage()

// Removed blocking invoke call from module top-level to improve startup speed
// Use syncLocaleToBackend() in App.vue onMounted instead

export const i18n = createI18n({
  legacy: false,
  locale: initialLocale,
  fallbackLocale: 'en',
  messages: {
    zh,
    en
  }
})

// Non-blocking function to sync initial locale with backend (call in App.vue)
export function syncLocaleToBackend() {
  invoke('set_log_locale', { locale: i18n.global.locale.value }).catch(() => {
    // Ignore errors during initialization
  })
}

// Helper function to sync locale with backend when user changes language
export async function setLocale(locale: string) {
  i18n.global.locale.value = locale as 'en' | 'zh'
  try {
    await invoke('set_log_locale', { locale })
  } catch (e) {
    console.debug('Failed to set backend locale:', e)
  }
}

export default i18n