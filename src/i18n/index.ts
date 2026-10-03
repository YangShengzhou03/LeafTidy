import { ref } from 'vue'
import type { Locale } from './types'
import { fragments } from './dicts'

const SETTINGS_KEY = 'leaf-tidy-settings'

function loadLocale(): Locale {
  try {
    const saved = JSON.parse(localStorage.getItem(SETTINGS_KEY) || '{}')
    return saved.language === 'zh-TW' || saved.language === 'en' ? saved.language : 'zh-CN'
  } catch {
    return 'zh-CN'
  }
}

export const locale = ref<Locale>(loadLocale())

export function setLanguage(l: Locale) {
  locale.value = l
  document.documentElement.lang = l
  try {
    const saved = JSON.parse(localStorage.getItem(SETTINGS_KEY) || '{}')
    saved.language = l
    localStorage.setItem(SETTINGS_KEY, JSON.stringify(saved))
  } catch {
    // 持久化失败不影响本次会话内的语言切换
  }
}

const dict = new Map<string, Record<Locale, string>>()
for (const fragment of fragments) {
  for (const [key, value] of Object.entries(fragment)) {
    dict.set(key, value as Record<Locale, string>)
  }
}

export function t(key: string, params?: Record<string, string | number>): string {
  const entry = dict.get(key)
  let text = entry ? (entry[locale.value] ?? entry['zh-CN']) : key
  if (params) {
    for (const [name, value] of Object.entries(params)) {
      text = text.replace(new RegExp(`\\{${name}\\}`, 'g'), String(value))
    }
  }
  return text
}

document.documentElement.lang = locale.value
