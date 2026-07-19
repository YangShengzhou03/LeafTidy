<template>
  <div style="padding: 20px;">
    <el-card style="margin-bottom: 20px;">
      <template #header>
        <span>微信状态</span>
      </template>

      <div style="display: flex; justify-content: space-between; align-items: center;">
        <div style="display: flex; align-items: center; gap: 12px;">
          <el-avatar :size="48">
            <el-icon :size="24"><User /></el-icon>
          </el-avatar>
          <div style="display: flex; flex-direction: column; gap: 6px;">
            <div>{{ status.username || '未登录' }}</div>
            <el-tag :type="status.online ? 'success' : 'danger'" size="small">
              {{ status.online ? '在线' : '离线' }}
            </el-tag>
          </div>
        </div>
        <el-button type="primary" :loading="loading" @click="handleCheckStatus">刷新</el-button>
      </div>
      <div v-if="status.login_time" style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
        <el-icon><Clock /></el-icon>
        <span>{{ status.login_time }}</span>
      </div>
      <el-alert v-if="!status.online" title="请先登录微信客户端" type="warning" :closable="false" show-icon style="margin-top: 12px;" />
    </el-card>

    <el-card>
      <template #header>
        <span>发送消息</span>
      </template>

      <el-form :model="form" label-width="90px">
        <el-form-item label="接收者">
          <el-input v-model="form.recipient" placeholder="微信好友名称" />
        </el-form-item>

        <el-form-item label="消息内容">
          <el-input v-model="form.message" type="textarea" :rows="3" placeholder="消息内容" />
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="handleSend">立即发送</el-button>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useWeChatStore } from '../stores/wechat'
import { storeToRefs } from 'pinia'
import { ElMessage } from 'element-plus'
import { User, Clock } from '@element-plus/icons-vue'

const wechatStore = useWeChatStore()
const { status, loading } = storeToRefs(wechatStore)

const form = ref({
  recipient: '',
  message: ''
})

const handleCheckStatus = async () => {
  await wechatStore.checkStatus()
  if (status.value.online) {
    ElMessage.success('微信已在线')
  } else {
    ElMessage.warning('微信未登录')
  }
}

const handleSend = async () => {
  if (!form.value.recipient || !form.value.message) {
    ElMessage.warning('请填写完整信息')
    return
  }

  if (!status.value.online) {
    ElMessage.error('微信未在线，请先登录')
    return
  }

  try {
    const result = await wechatStore.sendMessage(form.value.recipient, form.value.message)
    if (result.success) {
      ElMessage.success('消息发送成功')
      form.value.recipient = ''
      form.value.message = ''
    } else {
      ElMessage.error(result.message || '发送失败')
    }
  } catch (error) {
    ElMessage.error('发送失败')
  }
}

onMounted(() => {
  wechatStore.checkStatus()
})
</script>

