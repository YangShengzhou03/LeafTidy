import { createApp } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import ElementPlus from 'element-plus'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import zhTw from 'element-plus/dist/locale/zh-tw.mjs'
import en from 'element-plus/dist/locale/en.mjs'
import App from './App.vue'
import { loadSettings, resolveTheme } from './utils/settings'
import 'element-plus/dist/index.css'
import './styles/theme.css'
import './styles/components.css'

// 渲染前应用主题与语言，避免首帧闪烁；默认浅色、简体中文
let saved = {}
try {
  saved = JSON.parse(localStorage.getItem('leaf-tidy-settings') || '{}')
} catch (e) {
  saved = {}
}
document.documentElement.dataset.theme = resolveTheme(saved.theme)

// 跟随系统主题：操作系统切换深浅色时实时联动（仅在 system 模式下生效）
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
  if (loadSettings().theme === 'system') {
    document.documentElement.dataset.theme = resolveTheme('system')
  }
})

const epLocales = { 'zh-CN': zhCn, 'zh-TW': zhTw, 'en': en }
const lang = saved.language in epLocales ? saved.language : 'zh-CN'
document.documentElement.lang = lang

const app = createApp(App)

// 前端异常写入审计日志，生产构建 drop_console 后仍可追溯
function reportFrontendError(kind, err) {
  const detail = (err && (err.stack || err.message)) || String(err)
  invoke('write_event', { action: `${kind}: ${detail}`.slice(0, 500), size: 0 }).catch(() => {})
}

// 全局错误处理 —— 防止未捕获异常导致白屏
app.config.errorHandler = (err, instance, info) => {
  console.error(`[Vue Error] ${info}:`, err)
  reportFrontendError(`前端错误(${info})`, err)
}

// 全局未处理 Promise 拒绝监听
window.addEventListener('unhandledrejection', (event) => {
  console.error('[Unhandled Promise Rejection]:', event.reason)
  reportFrontendError('未处理Promise拒绝', event.reason)
})

app.use(ElementPlus, { locale: epLocales[lang] })
app.mount('#app')

// 兜底：窗口配置为 visible:false，正常由 App.vue onMounted 显示；
// 若前端初始化在挂载前异常，窗口将永不显示，8s 后强制显示
setTimeout(() => {
  import('@tauri-apps/api/window')
    .then(({ getCurrentWindow }) => getCurrentWindow().show())
    .catch(() => {})
}, 8000)
