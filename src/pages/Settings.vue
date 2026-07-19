<template>
  <div class="settings-page">
    <div class="settings-sidebar">
      <nav class="settings-nav">
        <a
          v-for="item in menuItems"
          :key="item.id"
          :href="'#' + item.id"
          class="settings-nav-item"
          :class="{ 'active': activeSection === item.id }"
          @click="scrollToSection(item.id)"
        >
          {{ item.label }}
        </a>
      </nav>
    </div>

    <div class="settings-content" ref="contentRef">
      <div class="settings-section" id="basic">
        <h2 class="section-title">基本设置</h2>
        <el-form :model="basicSettings" label-width="140px">
          <el-form-item label="联系人匹配模式">
            <el-radio-group v-model="basicSettings.matchMode">
              <el-radio value="exact">精确匹配</el-radio>
              <el-radio value="fuzzy">模糊匹配</el-radio>
            </el-radio-group>
          </el-form-item>

          <el-form-item label="语言">
            <el-select v-model="basicSettings.language" style="width: 200px;">
              <el-option label="简体中文" value="zh-CN" />
              <el-option label="繁體中文" value="zh-TW" />
              <el-option label="English" value="en" />
            </el-select>
          </el-form-item>

          <el-form-item label="网络时间同步">
            <el-switch v-model="basicSettings.syncNetworkTime" />
          </el-form-item>

          <el-form-item label="关闭到托盘">
            <el-switch v-model="basicSettings.minimizeToTray" />
          </el-form-item>

          <el-form-item label="自动更新">
            <el-switch v-model="basicSettings.autoUpdate" />
          </el-form-item>

          <el-form-item label="启动时自动运行">
            <el-switch v-model="basicSettings.autoStart" />
          </el-form-item>

          <el-form-item>
            <el-button type="primary" @click="handleSaveBasicSettings">保存设置</el-button>
          </el-form-item>
        </el-form>
      </div>

      <div class="divider"></div>

      <div class="settings-section" id="wechat">
        <h2 class="section-title">微信监控</h2>
        <el-form :model="wechatSettings" label-width="140px">
          <el-form-item label="掉线检测间隔">
            <el-input-number v-model="wechatSettings.checkInterval" :min="1" :max="60" />
            <span style="margin-left: 8px;">分钟</span>
          </el-form-item>

          <el-form-item label="掉线后行为">
            <el-checkbox-group v-model="wechatSettings.offlineActions">
              <el-checkbox value="sound">播放提示音</el-checkbox>
              <el-checkbox value="email">邮件提醒</el-checkbox>
              <el-checkbox value="notification">系统通知</el-checkbox>
            </el-checkbox-group>
          </el-form-item>

          <el-form-item v-if="wechatSettings.offlineActions.includes('email')" label="通知邮箱">
            <el-input v-model="wechatSettings.notifyEmail" placeholder="请输入邮箱地址" style="width: 300px;" />
          </el-form-item>

          <el-form-item label="掉线后自动重连">
            <el-switch v-model="wechatSettings.autoReconnect" />
          </el-form-item>

          <el-form-item label="重连尝试次数">
            <el-input-number v-model="wechatSettings.reconnectTimes" :min="1" :max="10" />
            <span style="margin-left: 8px;">次</span>
          </el-form-item>

          <el-form-item>
            <el-button type="primary" @click="handleSaveWechatSettings">保存设置</el-button>
          </el-form-item>
        </el-form>
      </div>

      <div class="divider"></div>

      <div class="settings-section" id="task">
        <h2 class="section-title">任务执行</h2>
        <el-form :model="taskSettings" label-width="140px">
          <el-form-item label="失败重试次数">
            <el-input-number v-model="taskSettings.retryTimes" :min="0" :max="10" />
            <span style="margin-left: 8px;">次</span>
          </el-form-item>

          <el-form-item label="重试间隔">
            <el-input-number v-model="taskSettings.retryInterval" :min="1" :max="60" />
            <span style="margin-left: 8px;">秒</span>
          </el-form-item>

          <el-form-item label="并发任务数">
            <el-input-number v-model="taskSettings.concurrentTasks" :min="1" :max="10" />
            <span style="margin-left: 8px;">个</span>
          </el-form-item>

          <el-form-item label="任务执行超时">
            <el-input-number v-model="taskSettings.timeout" :min="10" :max="300" />
            <span style="margin-left: 8px;">秒</span>
          </el-form-item>

          <el-form-item label="智能拆句">
            <el-switch v-model="taskSettings.autoSplit" />
          </el-form-item>

          <el-form-item v-if="taskSettings.autoSplit" label="拆句长度">
            <el-input-number v-model="taskSettings.splitLength" :min="100" :max="1000" />
            <span style="margin-left: 8px;">字符</span>
          </el-form-item>

          <el-form-item label="执行失败通知">
            <el-checkbox-group v-model="taskSettings.failureNotify">
              <el-checkbox value="sound">播放提示音</el-checkbox>
              <el-checkbox value="email">邮件提醒</el-checkbox>
              <el-checkbox value="notification">系统通知</el-checkbox>
            </el-checkbox-group>
          </el-form-item>

          <el-form-item>
            <el-button type="primary" @click="handleSaveTaskSettings">保存设置</el-button>
          </el-form-item>
        </el-form>
      </div>

      <div class="divider"></div>

      <div class="settings-section" id="system">
        <h2 class="section-title">系统优化</h2>
        <el-form :model="systemSettings" label-width="140px">
          <el-form-item label="防电脑休眠">
            <el-switch v-model="systemSettings.preventSleep" />
          </el-form-item>

          <el-form-item label="低性能模式">
            <el-switch v-model="systemSettings.lowPerformanceMode" />
          </el-form-item>

          <el-form-item label="日志保留天数">
            <el-input-number v-model="systemSettings.logRetentionDays" :min="1" :max="30" />
            <span style="margin-left: 8px;">天</span>
          </el-form-item>

          <el-form-item label="自动清理已完成任务">
            <el-switch v-model="systemSettings.autoCleanTasks" />
          </el-form-item>

          <el-form-item v-if="systemSettings.autoCleanTasks" label="清理时间">
            <el-time-picker v-model="systemSettings.cleanTime" placeholder="选择清理时间" />
          </el-form-item>

          <el-form-item label="数据备份">
            <el-button type="primary" @click="handleBackupData">备份数据</el-button>
            <el-button @click="handleRestoreData">恢复数据</el-button>
          </el-form-item>

          <el-form-item>
            <el-button type="primary" @click="handleSaveSystemSettings">保存设置</el-button>
          </el-form-item>
        </el-form>
      </div>

      <div class="divider"></div>

      <div class="settings-section" id="license">
        <h2 class="section-title">授权管理</h2>
        <div class="license-card">
          <div class="license-header">
            <div>
              <div class="license-name">LeafMaster - 轻羽大师版</div>
              <div class="license-status">{{ licenseInfo.status }}</div>
            </div>
            <span class="license-tag">{{ licenseInfo.label }}</span>
          </div>

          <el-form label-width="100px">
            <el-form-item label="授权状态">
              <span>{{ licenseInfo.status }}</span>
            </el-form-item>

            <el-form-item v-if="licenseInfo.trialDays" label="剩余试用">
              <span>{{ licenseInfo.trialDays }} 天</span>
            </el-form-item>

            <el-form-item label="激活卡密">
              <el-input v-model="activationCode" placeholder="请输入激活卡密" style="width: 300px;" />
            </el-form-item>

            <el-form-item>
              <el-button type="primary" @click="handleActivate">激活授权</el-button>
            </el-form-item>
          </el-form>

          <div class="pricing-info">
            <div class="pricing-item">试用版:免费试用 7 天,功能无限制</div>
            <div class="pricing-item">正式版:¥399 永久授权,一次购买终身使用</div>
            <div class="pricing-item">增值服务:售后 ¥29.9 起,维护 ¥49.9 起,培训 ¥69.9 起</div>
          </div>
        </div>
      </div>

      <div class="divider"></div>

      <div class="settings-section" id="about">
        <h2 class="section-title">关于</h2>
        <div class="about-card">
          <div class="about-title">LeafMaster - 轻羽大师版</div>
          <div class="about-subtitle">专业级微信自动化系统</div>

          <div class="about-info">
            <div class="info-row">
              <span class="info-label">应用版本</span>
              <span class="info-value">v1.0.0</span>
            </div>
            <div class="info-row">
              <span class="info-label">更新时间</span>
              <span class="info-value">2025-07-19</span>
            </div>
            <div class="info-row">
              <span class="info-label">官方网站</span>
              <a href="https://gitee.com/Yangshengzhou/LeafAuto" target="_blank" class="link-text">Gitee 主页</a>
            </div>
            <div class="info-row">
              <span class="info-label">技术支持</span>
              <span class="info-value">yangsz03@foxmail.com</span>
            </div>
          </div>

          <div class="about-actions">
            <el-button type="primary" @click="handleCheckUpdate">检查更新</el-button>
            <el-button @click="handleOpenWebsite">访问官网</el-button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { ElMessage } from 'element-plus'

const activationCode = ref('')
const activeSection = ref('basic')
const contentRef = ref<HTMLElement | null>(null)

const menuItems = [
  { id: 'basic', label: '基本设置' },
  { id: 'wechat', label: '微信监控' },
  { id: 'task', label: '任务执行' },
  { id: 'system', label: '系统优化' },
  { id: 'license', label: '授权管理' },
  { id: 'about', label: '关于' }
]

const licenseInfo = ref({
  status: '试用版(剩余 7 天)',
  type: 'warning',
  label: '试用中',
  trialDays: 7
})

const basicSettings = ref({
  matchMode: 'exact',
  language: 'zh-CN',
  syncNetworkTime: true,
  minimizeToTray: true,
  autoUpdate: true,
  autoStart: false
})

const wechatSettings = ref({
  checkInterval: 2,
  offlineActions: ['sound'],
  notifyEmail: '',
  autoReconnect: true,
  reconnectTimes: 3
})

const taskSettings = ref({
  retryTimes: 3,
  retryInterval: 5,
  concurrentTasks: 3,
  timeout: 60,
  autoSplit: false,
  splitLength: 500,
  failureNotify: ['sound']
})

const systemSettings = ref({
  preventSleep: false,
  lowPerformanceMode: false,
  logRetentionDays: 7,
  autoCleanTasks: false,
  cleanTime: new Date(2025, 0, 1, 3, 0, 0)
})

const scrollToSection = (sectionId: string) => {
  const element = document.getElementById(sectionId)
  if (element) {
    element.scrollIntoView({ behavior: 'smooth', block: 'start' })
  }
}

const handleScroll = () => {
  if (!contentRef.value) return

  const sections = menuItems.map(item => ({
    id: item.id,
    element: document.getElementById(item.id)
  }))

  for (let i = sections.length - 1; i >= 0; i--) {
    const section = sections[i]
    if (section.element) {
      const rect = section.element.getBoundingClientRect()
      if (rect.top <= 100) {
        activeSection.value = section.id
        break
      }
    }
  }
}

onMounted(() => {
  if (contentRef.value) {
    contentRef.value.addEventListener('scroll', handleScroll)
  }
})

onUnmounted(() => {
  if (contentRef.value) {
    contentRef.value.removeEventListener('scroll', handleScroll)
  }
})

const handleSaveBasicSettings = () => {
  ElMessage.success('基本设置已保存')
}

const handleSaveWechatSettings = () => {
  ElMessage.success('微信监控设置已保存')
}

const handleSaveTaskSettings = () => {
  ElMessage.success('任务执行设置已保存')
}

const handleSaveSystemSettings = () => {
  ElMessage.success('系统优化设置已保存')
}

const handleBackupData = () => {
  ElMessage.info('数据备份功能开发中')
}

const handleRestoreData = () => {
  ElMessage.info('数据恢复功能开发中')
}

const handleActivate = () => {
  if (!activationCode.value) {
    ElMessage.warning('请输入激活卡密')
    return
  }

  ElMessage.success('授权激活成功')
  licenseInfo.value = {
    status: '正式版',
    type: 'success',
    label: '已激活',
    trialDays: 0
  }
  activationCode.value = ''
}

const handleCheckUpdate = () => {
  ElMessage.info('已是最新版本')
}

const handleOpenWebsite = () => {
  window.open('https://gitee.com/Yangshengzhou/LeafAuto', '_blank')
}
</script>

<style scoped>
.settings-page {
  display: flex;
  height: 100%;
  background: #ffffff;
}

.settings-sidebar {
  width: 200px;
  border-right: 1px solid #e5edf5;
  padding: 32px 0;
  background: #ffffff;
  flex-shrink: 0;
}

.settings-nav {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 0 12px;
}

.settings-nav-item {
  display: block;
  padding: 12px 16px;
  color: #061b31;
  text-decoration: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 400;
  letter-spacing: -0.01em;
  transition: background 0.2s;
}

.settings-nav-item:hover {
  background: #f8fafd;
}

.settings-nav-item.active {
  background: #533afd;
  color: #ffffff;
}

.settings-content {
  flex: 1;
  overflow-y: auto;
  padding: 32px;
}

.settings-section {
  margin-bottom: 0;
}

.section-title {
  font-size: 32px;
  font-weight: 300;
  color: #061b31;
  margin: 0 0 32px 0;
  letter-spacing: -0.64px;
}

.divider {
  height: 1px;
  background: #e5edf5;
  margin: 52px 0;
}

.license-card {
  padding: 32px;
  background: #ffffff;
  border: 1px solid #e5edf5;
  border-radius: 4px;
  max-width: 500px;
}

.license-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.license-name {
  font-size: 18px;
  font-weight: 300;
  color: #061b31;
  margin-bottom: 4px;
}

.license-status {
  font-size: 14px;
  color: #64748d;
  font-weight: 300;
}

.license-tag {
  display: inline-block;
  padding: 6px 12px;
  font-size: 14px;
  font-weight: 400;
  border-radius: 4px;
  background: #e8e9ff;
  color: #533afd;
}

.pricing-info {
  margin-top: 24px;
  padding: 16px;
  background: #f8fafd;
  border-radius: 4px;
}

.pricing-item {
  font-size: 14px;
  color: #50617a;
  font-weight: 300;
  margin-bottom: 8px;
}

.pricing-item:last-child {
  margin-bottom: 0;
}

.about-card {
  padding: 32px;
  background: #ffffff;
  border: 1px solid #e5edf5;
  border-radius: 4px;
  max-width: 600px;
  text-align: center;
}

.about-title {
  font-size: 24px;
  font-weight: 300;
  color: #061b31;
  margin-bottom: 8px;
}

.about-subtitle {
  font-size: 14px;
  color: #64748d;
  font-weight: 300;
  margin-bottom: 32px;
}

.about-info {
  text-align: left;
  margin-bottom: 32px;
}

.info-row {
  display: flex;
  justify-content: space-between;
  padding: 12px 0;
  border-bottom: 1px solid #e5edf5;
}

.info-row:last-child {
  border-bottom: none;
}

.info-label {
  font-size: 14px;
  color: #64748d;
  font-weight: 300;
}

.info-value {
  font-size: 14px;
  color: #061b31;
  font-weight: 400;
}

.link-text {
  font-size: 14px;
  font-weight: 400;
  color: #533afd;
  text-decoration: none;
}

.link-text:hover {
  color: #7389ff;
}

.about-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
}
</style>