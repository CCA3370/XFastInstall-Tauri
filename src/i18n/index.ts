import { createI18n } from 'vue-i18n'
import zh from './zh'
import en from './en'

// 获取系统语言
const getSystemLanguage = (): string => {
  const lang = navigator.language || 'en'
  return lang.startsWith('zh') ? 'zh' : 'en'
}

export const i18n = createI18n({
  legacy: false,
  locale: getSystemLanguage(),
  fallbackLocale: 'en',
  messages: {
    zh,
    en
  }
})

export default i18n