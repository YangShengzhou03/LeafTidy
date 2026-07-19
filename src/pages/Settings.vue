<template>
  <div>
    <el-tabs v-model="activeTab" type="border-card">
      <el-tab-pane label="基本设置" name="basic">
        <el-form :model="basicSettings" label-width="120px">
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

      <el-tab-pane label="高级设置" name="advanced">
        <el-alert title="高级设置仅专业版可用" type="info" :closable="false" show-icon style="margin-bottom: 20px;" />

        <el-form :model="advancedSettings" label-width="120px" :disabled="!isProVersion">
          <el-form-item label="错误提示音">
            <el-switch v-model="advancedSettings.errorSound" :disabled="!isProVersion" />
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              任务执行失败时播放提示音
            </div>
          </el-form-item>

          <el-form-item label="邮箱提醒">
            <el-switch v-model="advancedSettings.emailNotify" :disabled="!isProVersion" />
          </el-form-item>

          <el-form-item v-if="advancedSettings.emailNotify" label="邮箱地址">
            <el-input v-model="advancedSettings.emailAddress" :disabled="!isProVersion" style="width: 300px;" />
          </el-form-item>

          <el-form-item label="防电脑休眠">
            <el-switch v-model="advancedSettings.preventSleep" :disabled="!isProVersion" />
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              保持系统唤醒状态，防止任务执行中断
            </div>
          </el-form-item>

          <el-form-item label="自动锁屏保护">
            <el-switch v-model="advancedSettings.autoLock" :disabled="!isProVersion" />
            <div style="font-size: 12px; color: #909399; margin-top: 4px;">
              服务器断连时自动锁屏，保护隐私安全
            </div>
          </el-form-item>

          <el-form-item label="失败重试次数">
            <el-input-number v-model="advancedSettings.retryTimes" :min="1" :max="10" :disabled="!isProVersion" />
            <span style="margin-left: 8px;">次</span>
          </el-form-item>

          <el-form-item label="并发任务数">
            <el-input-number v-model="advancedSettings.concurrentTasks" :min="1" :max="10" :disabled="!isProVersion" />
            <span style="margin-left: 8px;">个</span>
          </el-form-item>

          <el-form-item>
            <el-button type="primary" :disabled="!isProVersion" @click="handleSaveAdvancedSettings">
              保存设置
            </el-button>
          </el-form-item>
        </el-form>
      </el-tab-pane>

      <el-tab-pane label="授权管理" name="license">
        <el-card style="max-width: 500px;">
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
            <div>
              <div style="font-size: 18px; font-weight: bold;">当前版本</div>
              <div style="color: #909399; font-size: 14px;">免费版</div>
            </div>
            <el-tag size="large">已激活</el-tag>
          </div>

          <el-form label-width="100px">
            <el-form-item label="授权状态">
              <span>已激活（免费版）</span>
            </el-form-item>

            <el-form-item label="任务限制">
              <span>最多 5 个任务</span>
            </el-form-item>

            <el-form-item label="每日限制">
              <span>最多 10 个任务/天</span>
            </el-form-item>

            <el-form-item label="激活卡密">
              <el-input v-model="activationCode" placeholder="请输入激活卡密" style="width: 300px;" />
            </el-form-item>

            <el-form-item>
              <el-button type="primary" @click="handleActivate">激活授权</el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-tab-pane>

      <el-tab-pane label="关于" name="about">
        <el-card style="max-width: 600px;">
          <div style="text-align: center; padding: 20px 0;">
            <div style="font-size: 24px; font-weight: bold; margin-bottom: 12px;">轻羽大师版</div>
            <div style="color: #909399; margin-bottom: 24px;">专业级微信自动化系统</div>

            <el-descriptions :column="1" border>
              <el-descriptions-item label="应用版本">v1.0.0</el-descriptions-item>
              <el-descriptions-item label="技术栈">ElementPlus + Rust (Tauri)</el-descriptions-item>
              <el-descriptions-item label="更新时间">2025-07-19</el-descriptions-item>
              <el-descriptions-item label="开发团队">Yangshengzhou</el-descriptions-item>
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
const isProVersion = ref(false)
const activationCode = ref('')

const basicSettings = ref({
  language: 'zh-CN',
  syncNetworkTime: true,
  minimizeToTray: true,
  autoUpdate: true,
  autoStart: false
})

const advancedSettings = ref({
  errorSound: false,
  emailNotify: false,
  emailAddress: '',
  preventSleep: false,
  autoLock: false,
  retryTimes: 3,
  concurrentTasks: 3
})

const handleSaveBasicSettings = () => {
  ElMessage.success('基本设置已保存')
}

const handleSaveAdvancedSettings = () => {
  if (!isProVersion.value) {
    ElMessage.warning('高级设置仅专业版可用')
    return
  }
  ElMessage.success('高级设置已保存')
}

const handleActivate = () => {
  if (!activationCode.value) {
    ElMessage.warning('请输入激活卡密')
    return
  }

  ElMessage.success('授权激活成功')
  isProVersion.value = true
  activationCode.value = ''
}

const handleCheckUpdate = () => {
  ElMessage.info('已是最新版本')
}

const handleOpenWebsite = () => {
  window.open('https://gitee.com/Yangshengzhou/LeafAuto', '_blank')
}
</script>