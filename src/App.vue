<template>
  <div class="app-container">
    <div class="titlebar">
      <div class="titlebar-left">
        <span class="titlebar-logo">LeafMaster - 轻羽大师版</span>
      </div>
      <div class="titlebar-right">
        <div class="wechat-user" v-if="wechatStore.status.online" @click="handleRefreshStatus">
          <div class="user-avatar">{{ wechatStore.status.username ? wechatStore.status.username.charAt(0) : '微' }}</div>
          <span class="user-name">{{ wechatStore.status.username || '微信用户' }}</span>
          <span class="user-wechat-id" v-if="wechatStore.status.wechat_id">({{ wechatStore.status.wechat_id }})</span>
          <span class="user-status" :class="statusClass"></span>
        </div>
        <div class="wechat-user offline" v-else @click="handleRefreshStatus">
          <div class="user-avatar">微</div>
          <span class="user-name">未连接</span>
          <span class="user-status" :class="statusClass"></span>
        </div>
        <button class="titlebar-btn" @click="minimizeWindow">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
            <path d="M2 6H10" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </button>
        <button class="titlebar-btn" @click="maximizeWindow">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
            <rect x="2" y="2" width="8" height="8" stroke="currentColor" stroke-width="2" rx="1"/>
          </svg>
        </button>
        <button class="titlebar-btn titlebar-close" @click="closeWindow">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
            <path d="M2 2L10 10M2 10L10 2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </button>
      </div>
    </div>

    <div class="main-wrapper">
      <div class="sidebar">
        <nav class="sidebar-nav">
          <router-link to="/" class="nav-item" :class="{ 'active': $route.path === '/' }">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
              <polyline points="9 22 9 12 15 12 15 22"/>
            </svg>
            <span>首页</span>
          </router-link>
          <router-link to="/auto-message" class="nav-item" :class="{ 'active': $route.path === '/auto-message' }">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
            </svg>
            <span>自动信息</span>
          </router-link>
          <router-link to="/data-analysis" class="nav-item" :class="{ 'active': $route.path === '/data-analysis' }">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <line x1="18" y1="20" x2="18" y2="10"/>
              <line x1="12" y1="20" x2="12" y2="4"/>
              <line x1="6" y1="20" x2="6" y2="14"/>
            </svg>
            <span>数据分析</span>
          </router-link>
          <router-link to="/log-analysis" class="nav-item" :class="{ 'active': $route.path === '/log-analysis' }">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
              <polyline points="14 2 14 8 20 8"/>
              <line x1="16" y1="13" x2="8" y2="13"/>
              <line x1="16" y1="17" x2="8" y2="17"/>
              <polyline points="10 9 9 9 8 9"/>
            </svg>
            <span>日志分析</span>
          </router-link>
        </nav>
        <div class="sidebar-footer">
          <router-link to="/settings" class="nav-item" :class="{ 'active': $route.path === '/settings' }">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <circle cx="12" cy="12" r="3"/>
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
            </svg>
            <span>设置</span>
          </router-link>
        </div>
      </div>

      <main class="content">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useWeChatStore } from './stores/wechat'
import { ElMessage } from 'element-plus'

const appWindow = getCurrentWindow()
const wechatStore = useWeChatStore()

const statusClass = computed(() => {
  const { online, task_running } = wechatStore.status

  if (online && !task_running) {
    return 'online-no-task' // 绿色：登录成功无任务
  } else if (online && task_running) {
    return 'online-with-task' // 主题色：登录成功有任务
  } else if (!online && task_running) {
    return 'offline-with-task' // 红色：登录失败有任务
  } else {
    return '' // 灰色：默认
  }
})

const handleRefreshStatus = async () => {
  try {
    await wechatStore.checkStatus()
    if (wechatStore.status.online) {
      ElMessage.success('已刷新微信状态')
    } else {
      ElMessage.warning('未检测到微信窗口')
    }
  } catch (error) {
    ElMessage.error('刷新失败，请稍后重试')
  }
}

const minimizeWindow = async () => {
  try {
    await appWindow.minimize()
  } catch (error) {
    console.error('最小化窗口失败:', error)
  }
}

const maximizeWindow = async () => {
  try {
    await appWindow.toggleMaximize()
  } catch (error) {
    console.error('最大化窗口失败:', error)
  }
}

const closeWindow = async () => {
  try {
    await appWindow.close()
  } catch (error) {
    console.error('关闭窗口失败:', error)
  }
}

onMounted(async () => {
  // 初始化时检查微信状态
  await wechatStore.checkStatus()
})
</script>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #ffffff;
  font-family: 'Inter Tight', ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
}

.titlebar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 56px;
  padding: 0 24px;
  background: #ffffff;
  border-bottom: 1px solid #e5edf5;
  -webkit-app-region: drag;
}

.titlebar-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.titlebar-logo {
  font-size: 16px;
  font-weight: 300;
  color: #061b31;
  letter-spacing: -0.01em;
}

.titlebar-right {
  display: flex;
  align-items: center;
  gap: 16px;
  -webkit-app-region: no-drag;
}

.wechat-user {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  background: transparent;
  border-radius: 4px;
  -webkit-app-region: no-drag;
  cursor: pointer;
  transition: background 0.2s;
}

.wechat-user:hover {
  background: #f8fafd;
}

.wechat-user.offline {
  opacity: 0.6;
}

.user-avatar {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #533afd;
  color: #ffffff;
  border-radius: 50%;
  font-size: 16px;
  font-weight: 400;
}

.user-name {
  font-size: 14px;
  color: #061b31;
  font-weight: 400;
  letter-spacing: -0.01em;
}

.user-wechat-id {
  font-size: 12px;
  color: #64748d;
  font-weight: 300;
}

.user-status {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #64748d;
}

/* 登录成功无任务 - 绿色 */
.user-status.online-no-task {
  background: #10b981;
}

/* 登录成功有任务 - 主题色 */
.user-status.online-with-task {
  background: #533afd;
}

/* 登录失败有任务 - 红色 */
.user-status.offline-with-task {
  background: #ef4444;
}

.titlebar-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: #061b31;
  cursor: pointer;
  transition: background 0.2s;
}

.titlebar-btn:hover {
  background: #e5edf5;
}

.titlebar-close:hover {
  background: #ff5b5b;
  color: #ffffff;
}

.main-wrapper {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 200px;
  background: #ffffff;
  border-right: 1px solid #e5edf5;
  display: flex;
  flex-direction: column;
  padding: 24px 0;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 0 12px;
  flex: 1;
}

.sidebar-footer {
  padding: 0 12px;
  margin-top: auto;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  color: #061b31;
  text-decoration: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 400;
  letter-spacing: -0.01em;
  transition: background 0.2s;
}

.nav-item:hover {
  background: #f8fafd;
}

.nav-item.active {
  background: #533afd;
  color: #ffffff;
}

.content {
  flex: 1;
  padding: 32px;
  overflow-y: auto;
  background: #ffffff;
}
</style>