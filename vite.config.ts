import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],

  // Tauri expects a fixed port
  server: {
    port: 5173,
    strictPort: true,
    // 添加这部分的 watch 配置，解决文件锁定冲突
    watch: {
      ignored: ['**/src-tauri/target/**']
    }
  },

  // Env variables starting with TAURI_ are exposed to the client
  envPrefix: ['VITE_', 'TAURI_'],

  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target: ['es2021', 'chrome100', 'safari13'],
    // Don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // Produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },

  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
  },
})
