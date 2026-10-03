import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import zhTw from 'element-plus/dist/locale/zh-tw.mjs'
import en from 'element-plus/dist/locale/en.mjs'
import App from './App.vue'
import 'element-plus/dist/index.css'
import './styles/theme.css'
import './styles/components.css'

// 渲染前应用主题与语言，避免首帧闪烁；默认暗色、简体中文
let saved = {}
try {
  saved = JSON.parse(localStorage.getItem('leaf-tidy-settings') || '{}')
} catch (e) {
  saved = {}
}
document.documentElement.dataset.theme = saved.theme === 'light' ? 'light' : 'dark'

const epLocales = { 'zh-CN': zhCn, 'zh-TW': zhTw, 'en': en }
const lang = saved.language in epLocales ? saved.language : 'zh-CN'
document.documentElement.lang = lang

const app = createApp(App)

// 全局错误处理 —— 防止未捕获异常导致白屏
app.config.errorHandler = (err, instance, info) => {
  console.error(`[Vue Error] ${info}:`, err)
}

// 全局未处理 Promise 拒绝监听
window.addEventListener('unhandledrejection', (event) => {
  console.error('[Unhandled Promise Rejection]:', event.reason)
})

app.use(ElementPlus, { locale: epLocales[lang] })
app.mount('#app')
