declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module 'element-plus/dist/locale/*.mjs' {
  import type { Language } from 'element-plus/es/locale'
  const locale: Language
  export default locale
}
