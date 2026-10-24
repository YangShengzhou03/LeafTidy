/// 应用设置的唯一读写入口。
/// App.vue / SettingsPage / AiClassifyPage 共用同一份 localStorage 结构，
/// 避免各自 merge 时字段互相覆盖（例如设置页保存主题时丢掉默认模型）。
export const SETTINGS_KEY = 'leaf-tidy-settings'

export type ThemeMode = 'dark' | 'light'
export type CloseAction = 'exit' | 'minimize' | 'tray'

export interface AppSettings {
  theme?: ThemeMode
  closeAction?: CloseAction
  defaultModelId?: string
  customModels?: Array<{ id: string; name: string }>
}

export function loadSettings(): AppSettings {
  try {
    return JSON.parse(localStorage.getItem(SETTINGS_KEY) || '{}') as AppSettings
  } catch {
    return {}
  }
}

/// 读取并合并写入，返回合并后的完整设置。
export function saveSettings(patch: Partial<AppSettings>): AppSettings {
  const merged = { ...loadSettings(), ...patch }
  try {
    localStorage.setItem(SETTINGS_KEY, JSON.stringify(merged))
  } catch { /* localStorage 不可用时忽略 */ }
  return merged
}
