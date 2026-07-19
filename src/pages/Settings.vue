<template>
  <div>
    <el-tabs v-model="activeTab" type="border-card">
      <el-tab-pane label="基本设置" name="basic">
        <el-form :model="basicSettings" label-width="140px">
          <el-form-item label="联系人匹配模式">
            <el-radio-group v-model="basicSettings.matchMode">
              <el-radio value="exact">精确匹配</el-radio>
              <el-radio value="fuzzy">模糊匹配</el-radio>
            </el-radio-group>
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              精确匹配：联系人名必须完全一致；模糊匹配：相近包含即可
            </div>
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
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              启用后将自动同步网络时间，确保定时任务精准执行
            </div>
          </el-form-item>

          <el-form-item label="关闭到托盘">
            <el-switch v-model="basicSettings.minimizeToTray" />
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              关闭窗口时最小化到系统托盘，继续执行后台任务
            </div>
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
      </el-tab-pane>

      <el-tab-pane label="微信监控" name="wechat">
        <el-form :model="wechatSettings" label-width="140px">
          <el-form-item label="掉线检测间隔">
            <el-input-number v-model="wechatSettings.checkInterval" :min="1" :max="60" />
            <span style="margin-left: 8px;">分钟</span>
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              定期检测微信是否在线，建议设置 1-5 分钟
            </div>
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
      </el-tab-pane>

      <el-tab-pane label="任务执行" name="task">
        <el-form :model="taskSettings" label-width="140px">
          <el-form-item label="失败重试次数">
            <el-input-number v-model="taskSettings.retryTimes" :min="0" :max="10" />
            <span style="margin-left: 8px;">次</span>
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              任务执行失败时自动重试，设置为 0 表示不重试
            </div>
          </el-form-item>

          <el-form-item label="重试间隔">
            <el-input-number v-model="taskSettings.retryInterval" :min="1" :max="60" />
            <span style="margin-left: 8px;">秒</span>
          </el-form-item>

          <el-form-item label="并发任务数">
            <el-input-number v-model="taskSettings.concurrentTasks" :min="1" :max="10" />
            <span style="margin-left: 8px;">个</span>
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              同时执行的任务数量，建议不超过 5 个
            </div>
          </el-form-item>

          <el-form-item label="任务执行超时">
            <el-input-number v-model="taskSettings.timeout" :min="10" :max="300" />
            <span style="margin-left: 8px;">秒</span>
          </el-form-item>

          <el-form-item label="智能拆句">
            <el-switch v-model="taskSettings.autoSplit" />
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              自动拆分长消息，避免消息过长发送失败
            </div>
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
      </el-tab-pane>

      <el-tab-pane label="系统优化" name="system">
        <el-form :model="systemSettings" label-width="140px">
          <el-form-item label="防电脑休眠">
            <el-switch v-model="systemSettings.preventSleep" />
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              保持系统唤醒状态，防止任务执行中断
            </div>
          </el-form-item>

          <el-form-item label="低性能模式">
            <el-switch v-model="systemSettings.lowPerformanceMode" />
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              降低 CPU 和内存占用，适合老旧电脑
            </div>
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
      </el-tab-pane>

      <el-tab-pane label="授权管理" name="license">
        <el-card style="max-width: 500px;">
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
            <div>
              <div style="font-size: 18px; font-weight: bold;">轻羽大师版</div>
              <div style="color: #909399; font-size: 14px;">{{ licenseInfo.status }}</div>
            </div>
            <el-tag :type="licenseInfo.type" size="large">{{ licenseInfo.label }}</el-tag>
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

          <el-alert title="定价说明" type="info" :closable="false" style="margin-top: 20px;">
            <div>试用版：免费试用 7 天，功能无限制</div>
            <div>正式版：¥399 永久授权，一次购买终身使用</div>
            <div style="margin-top: 8px;">增值服务：售后 ¥29.9 起，维护 ¥49.9 起，培训 ¥69.9 起</div>
          </el-alert>
        </el-card>
      </el-tab-pane>

      <el-tab-pane label="关于" name="about">
        <el-card style="max-width: 600px;">
          <div style="text-align: center; padding: 20px 0;">
            <div style="font-size: 24px; font-weight: bold; margin-bottom: 12px;">轻羽大师版</div>
            <div style="color: #909399; margin-bottom: 24px;">专业级微信自动化系统</div>

            <el-descriptions :column="1" border>
              <el-descriptions-item label="应用版本">v1.0.0</el-descriptions-item>
              <el-descriptions-item label="更新时间">2025-07-19</el-descriptions-item>
              <el-descriptions-item label="官方网站">
                <a href="https://gitee.com/Yangshengzhou/LeafAuto" target="_blank">Gitee 主页</a>
              </el-descriptions-item>
              <el-descriptions-item label="技术支持">yangsz03@foxmail.com</el-descriptions-item>
            </el-descriptions>

            <div style="margin-top: 24px;">
              <el-button type="primary" @click="handleCheckUpdate">检查更新</el-button>
              <el-button @click="handleOpenWebsite">访问官网</el-button>
            </div>
          </div>
        </el-card>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'

const activeTab = ref('basic')
const activationCode = ref('')

const licenseInfo = ref({
  status: '试用版（剩余 7 天）',
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