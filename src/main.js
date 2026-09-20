import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import zhTw from 'element-plus/dist/locale/zh-tw.mjs'
import en from 'element-plus/dist/locale/en.mjs'
import App from './App.vue'
import 'element-plus/dist/index.css'

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
app.use(ElementPlus, { locale: epLocales[lang] })
app.mount('#app')
