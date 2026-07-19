<template>
  <div>
    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="6">
        <el-card shadow="hover" style="text-align: center;">
          <el-statistic title="总任务数" :value="statistics.totalTasks">
            <template #suffix>
              <el-icon><Document /></el-icon>
            </template>
          </el-statistic>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" style="text-align: center;">
          <el-statistic title="今日执行" :value="statistics.todayExecuted">
            <template #suffix>
              <el-icon><CircleCheck /></el-icon>
            </template>
          </el-statistic>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" style="text-align: center;">
          <el-statistic title="成功率" :value="statistics.successRate" :precision="1" suffix="%">
            <template #prefix>
              <el-icon><TrendCharts /></el-icon>
            </template>
          </el-statistic>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" style="text-align: center;">
          <el-statistic title="微信状态" :value="wechatStatus.online ? '在线' : '离线'">
            <template #suffix>
              <el-icon :style="{ color: wechatStatus.online ? '#67C23A' : '#F56C6C' }">
                <component :is="wechatStatus.online ? 'CircleCheck' : 'CircleClose'" />
              </el-icon>
            </template>
          </el-statistic>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20">
      <el-col :span="16">
        <el-card>
          <template #header>
            <div style="display: flex; justify-content: space-between; align-items: center;">
              <span>最近任务</span>
              <el-button type="primary" text @click="$router.push('/auto-message')">
                查看全部
              </el-button>
            </div>
          </template>

          <el-table :data="recentTasks" style="width: 100%">
            <el-table-column prop="name" label="任务名称" />
            <el-table-column prop="recipient" label="接收者" width="120" />
            <el-table-column prop="type" label="类型" width="100">
              <template #default="{ row }">
                <el-tag :type="row.type === '定时' ? 'primary' : 'success'" size="small">
                  {{ row.type }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="nextExecute" label="下次执行" width="160" />
            <el-table-column prop="status" label="状态" width="80">
              <template #default="{ row }">
                <el-switch v-model="row.enabled" size="small" />
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>

      <el-col :span="8">
        <el-card>
          <template #header>
            <span>快速操作</span>
          </template>

          <div style="display: flex; flex-direction: column; gap: 12px;">
            <el-button type="primary" size="large" @click="$router.push('/auto-message')">
              <el-icon style="margin-right: 8px;"><Plus /></el-icon>
              创建定时任务
            </el-button>

            <el-button type="success" size="large" @click="handleQuickSend">
              <el-icon style="margin-right: 8px;"><ChatDotRound /></el-icon>
              快速发送消息
            </el-button>

            <el-button type="warning" size="large" @click="handleImportExcel">
              <el-icon style="margin-right: 8px;"><Upload /></el-icon>
              Excel 导入任务
            </el-button>

            <el-button type="info" size="large" @click="handleDownloadTemplate">
              <el-icon style="margin-right: 8px;"><Download /></el-icon>
              下载任务模板
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-dialog v-model="quickSendVisible" title="快速发送消息" width="500px">
      <el-form :model="quickSendForm" label-width="80px">
        <el-form-item label="接收者">
          <el-input v-model="quickSendForm.recipient" placeholder="微信好友或群聊备注名称" />
        </el-form-item>

        <el-form-item label="消息内容">
          <el-input v-model="quickSendForm.message" type="textarea" :rows="4" placeholder="消息内容" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="quickSendVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSendQuickMessage">发送</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { Document, CircleCheck, TrendCharts, Plus, ChatDotRound, Upload, Download } from '@element-plus/icons-vue'

const statistics = ref({
  totalTasks: 15,
  todayExecuted: 128,
  successRate: 98.5
})

const wechatStatus = ref({
  online: true,
  username: '张三'
})

const recentTasks = ref([
  { name: '早会提醒', recipient: '技术部群', type: '定时', nextExecute: '2025-07-20 09:00', enabled: true },
  { name: '日报发送', recipient: '工作群', type: '间隔', nextExecute: '2025-07-19 18:00', enabled: true },
  { name: '下午茶提醒', recipient: '下午茶群', type: '定时', nextExecute: '2025-07-20 15:00', enabled: false }
])

const quickSendVisible = ref(false)
const quickSendForm = ref({
  recipient: '',
  message: ''
})

const handleQuickSend = () => {
  quickSendVisible.value = true
}

const handleImportExcel = () => {
  ElMessage.info('Excel 导入功能开发中')
}

const handleDownloadTemplate = () => {
  ElMessage.info('模板下载功能开发中')
}

const handleSendQuickMessage = () => {
  if (!quickSendForm.value.recipient || !quickSendForm.value.message) {
    ElMessage.warning('请填写完整信息')
    return
  }

  ElMessage.success('消息发送成功')
  quickSendVisible.value = false
  quickSendForm.value = { recipient: '', message: '' }
}
</script>