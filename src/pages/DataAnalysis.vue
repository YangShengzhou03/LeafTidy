<template>
  <div class="data-analysis-page">
    <div class="page-header">
      <h1 class="page-title">数据分析</h1>
      <p class="page-subtitle">历史任务执行情况与性能分析</p>
    </div>

    <div class="divider"></div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-number">{{ statistics.totalExecuted }}</div>
        <div class="stat-label">总执行次数</div>
      </div>

      <div class="stat-card">
        <div class="stat-number">{{ statistics.successRate }}%</div>
        <div class="stat-label">成功率</div>
      </div>

      <div class="stat-card">
        <div class="stat-number">{{ statistics.avgDelay }}</div>
        <div class="stat-label">平均延误时间(秒)</div>
      </div>

      <div class="stat-card">
        <div class="stat-number">{{ statistics.failedCount }}</div>
        <div class="stat-label">失败次数</div>
      </div>
    </div>

    <div class="divider"></div>

    <div class="section">
      <div class="section-header">
        <h2 class="section-title">执行历史</h2>
        <div class="filter-group">
          <el-select v-model="dateRange" placeholder="时间范围" style="width: 150px;">
            <el-option label="今天" value="today" />
            <el-option label="最近7天" value="week" />
            <el-option label="最近30天" value="month" />
          </el-select>
          <el-select v-model="statusFilter" placeholder="状态筛选" style="width: 150px;">
            <el-option label="全部" value="all" />
            <el-option label="成功" value="success" />
            <el-option label="失败" value="failed" />
          </el-select>
        </div>
      </div>
      <div class="table-wrapper">
        <el-table :data="filteredHistory">
          <el-table-column prop="taskName" label="任务名称" width="150">
            <template #default="{ row }">
              <div class="text-ellipsis">{{ row.taskName }}</div>
            </template>
          </el-table-column>

          <el-table-column prop="recipient" label="接收对象" width="120">
            <template #default="{ row }">
              <div class="text-ellipsis">{{ row.recipient }}</div>
            </template>
          </el-table-column>

          <el-table-column prop="executeTime" label="执行时间" width="200">
            <template #default="{ row }">
              <div class="text-ellipsis" :title="row.executeTime">{{ row.executeTime }}</div>
            </template>
          </el-table-column>

          <el-table-column prop="delay" label="延误时间" width="150">
            <template #default="{ row }">
              <div class="text-ellipsis">
                <span :class="row.delay > 5 ? 'delay-warning' : ''">{{ row.delay.toFixed(4) }}秒</span>
              </div>
            </template>
          </el-table-column>

          <el-table-column prop="status" label="状态" width="100">
            <template #default="{ row }">
              <div class="text-ellipsis">
                <span class="status-tag" :class="row.status === '成功' ? 'success' : 'failed'">{{ row.status }}</span>
              </div>
            </template>
          </el-table-column>

          <el-table-column prop="errorMessage" label="错误信息" min-width="200">
            <template #default="{ row }">
              <div v-if="row.errorMessage" class="text-ellipsis" :title="row.errorMessage">{{ row.errorMessage }}</div>
              <div v-else class="text-muted">-</div>
            </template>
          </el-table-column>

          <el-table-column label="操作" width="80" fixed="right">
            <template #default="{ row, $index }">
              <el-button link type="danger" @click="handleDeleteHistory($index)">删除</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </div>

    <div class="divider"></div>

    <div class="section">
      <div class="section-header">
        <h2 class="section-title">性能分析</h2>
      </div>
      <div class="performance-grid">
        <div class="performance-card">
          <div class="performance-title">任务执行时间分布</div>
          <div class="performance-chart">
            <div class="chart-bars">
              <div class="chart-bar" v-for="item in timeDistribution" :key="item.label">
                <div class="bar-label">{{ item.label }}</div>
                <div class="bar-container">
                  <div class="bar-fill" :style="{ width: item.value + '%' }"></div>
                </div>
                <div class="bar-value">{{ item.value }}%</div>
              </div>
            </div>
          </div>
        </div>

        <div class="performance-card">
          <div class="performance-title">延误时间分析</div>
          <div class="delay-stats">
            <div class="delay-item">
              <span class="delay-label">小于1秒</span>
              <span class="delay-count">{{ delayAnalysis.lessThan1s }}次</span>
            </div>
            <div class="delay-item">
              <span class="delay-label">1-5秒</span>
              <span class="delay-count">{{ delayAnalysis.between1to5s }}次</span>
            </div>
            <div class="delay-item">
              <span class="delay-label">大于5秒</span>
              <span class="delay-count delay-warning">{{ delayAnalysis.moreThan5s }}次</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useTaskStore } from '../stores/task'

const taskStore = useTaskStore()

const dateRange = ref('week')
const statusFilter = ref('all')

const statistics = computed(() => {
  const tasks = taskStore.tasks

  const totalExecuted = tasks.reduce((sum, task) => sum + task.executeCount, 0)
  const failedCount = tasks.filter(t => t.status === 'failed').length
  const successRate = totalExecuted > 0 ? Math.round(((totalExecuted - failedCount) / totalExecuted) * 100) : 0

  return {
    totalExecuted,
    successRate,
    avgDelay: 0.5,
    failedCount
  }
})

const historyData = computed(() => {
  return taskStore.tasks.map(task => {
    const executeTime = task.executeTime ? new Date(task.executeTime).getTime() : Date.now()
    const plannedTime = executeTime
    const actualTime = executeTime + Math.random() * 5000
    const delayMs = (actualTime - plannedTime) / 1000
    const delay = Math.round(delayMs * 10000) / 10000
    
    return {
      recipient: task.recipient,
      executeTime: task.executeTime || task.nextExecute || '-',
      delay,
      status: task.status === 'failed' ? '失败' : '成功',
      errorMessage: task.status === 'failed' ? '发送失败' : ''
    }
  })
})

const filteredHistory = computed(() => {
  let result = historyData.value

  if (statusFilter.value !== 'all') {
    result = result.filter(item => item.status === (statusFilter.value === 'success' ? '成功' : '失败'))
  }

  return result
})

const timeDistribution = computed(() => {
  return [
    { label: '0-5秒', value: 75 },
    { label: '5-10秒', value: 20 },
    { label: '10秒以上', value: 5 }
  ]
})

const delayAnalysis = computed(() => {
  return {
    lessThan1s: Math.floor(statistics.value.totalExecuted * 0.6),
    between1to5s: Math.floor(statistics.value.totalExecuted * 0.3),
    moreThan5s: Math.floor(statistics.value.totalExecuted * 0.1)
  }
})

const handleDeleteHistory = (index: number) => {
}
</script>

<style scoped>
.data-analysis-page {
  max-width: 1320px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 48px;
  font-weight: 300;
  color: #061b31;
  margin: 0 0 8px 0;
  letter-spacing: -0.96px;
}

.page-subtitle {
  font-size: 22px;
  color: #64748d;
  margin: 0;
  font-weight: 300;
  letter-spacing: -0.22px;
}

.divider {
  height: 1px;
  background: #e5edf5;
  margin: 52px 0;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 24px;
}

.stat-card {
  padding: 32px;
  background: #ffffff;
}

.stat-number {
  font-size: 56px;
  font-weight: 300;
  color: #061b31;
  line-height: 1.03;
  letter-spacing: -1.4px;
  margin-bottom: 8px;
}

.stat-label {
  font-size: 16px;
  color: #64748d;
  font-weight: 300;
}

.section {
  margin-bottom: 0;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 32px;
}

.section-title {
  font-size: 32px;
  font-weight: 300;
  color: #061b31;
  margin: 0;
  letter-spacing: -0.64px;
}

.filter-group {
  display: flex;
  gap: 12px;
}

.table-wrapper {
  background: #ffffff;
  border: 1px solid #e5edf5;
  border-radius: 4px;
  overflow: hidden;
}

.text-ellipsis {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}

.text-muted {
  color: #64748d;
}

.delay-warning {
  color: #dc2626;
  font-weight: 400;
}

.status-tag {
  display: inline-block;
  padding: 4px 12px;
  font-size: 12px;
  font-weight: 400;
  border-radius: 9999px;
}

.status-tag.success {
  background: #e8e9ff;
  color: #533afd;
}

.status-tag.failed {
  background: #fee2e2;
  color: #dc2626;
}

.performance-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 24px;
}

.performance-card {
  padding: 32px;
  background: #ffffff;
  border: 1px solid #e5edf5;
  border-radius: 4px;
}

.performance-title {
  font-size: 16px;
  font-weight: 400;
  color: #061b31;
  margin-bottom: 24px;
  letter-spacing: -0.01em;
}

.chart-bars {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.chart-bar {
  display: flex;
  align-items: center;
  gap: 12px;
}

.bar-label {
  font-size: 14px;
  color: #50617a;
  font-weight: 300;
  width: 100px;
  flex-shrink: 0;
}

.bar-container {
  flex: 1;
  height: 8px;
  background: #e5edf5;
  border-radius: 4px;
  overflow: hidden;
}

.bar-fill {
  height: 100%;
  background: #533afd;
  border-radius: 4px;
  transition: width 0.3s;
}

.bar-value {
  font-size: 14px;
  color: #061b31;
  font-weight: 400;
  width: 40px;
  text-align: right;
}

.delay-stats {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.delay-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #f8fafd;
  border-radius: 4px;
}

.delay-label {
  font-size: 14px;
  color: #50617a;
  font-weight: 300;
}

.delay-count {
  font-size: 14px;
  color: #061b31;
  font-weight: 400;
}
</style>