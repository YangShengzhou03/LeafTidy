<template>
  <div class="home-page">
    <div class="page-header">
      <h1 class="page-title">欢迎回来</h1>
      <p class="page-subtitle">管理您的自动化任务</p>
    </div>

    <div class="divider"></div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-number">{{ statistics.totalTasks }}</div>
        <div class="stat-label">总任务数</div>
      </div>

      <div class="stat-card">
        <div class="stat-number">{{ statistics.todayExecuted }}</div>
        <div class="stat-label">今日执行</div>
      </div>

      <div class="stat-card">
        <div class="stat-number">{{ statistics.successRate }}%</div>
        <div class="stat-label">成功率</div>
      </div>
    </div>

    <div class="divider"></div>

    <div class="section">
      <div class="section-header">
        <h2 class="section-title">下一个任务</h2>
      </div>
      <div class="task-list" v-if="nextTask">
        <div class="task-item">
          <div class="task-info">
            <div class="task-name">{{ nextTask.name }}</div>
            <div class="task-meta">
              <span class="task-recipient">{{ nextTask.recipient }}</span>
              <span class="task-type" :class="nextTask.type">{{ nextTask.type }}</span>
            </div>
          </div>
          <div class="task-schedule">{{ nextTask.nextExecute }}</div>
          <button class="task-toggle" :class="{ 'active': nextTask.enabled }">
            <span class="toggle-dot"></span>
          </button>
        </div>
      </div>
      <div class="no-task-hint" v-else>
        <p>暂无待执行任务</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useTaskStore } from '../stores/task'

const taskStore = useTaskStore()

// 统计数据从 store 获取
const statistics = computed(() => ({
  totalTasks: taskStore.statistics.total,
  todayExecuted: taskStore.tasks.reduce((sum, t) => sum + t.executeCount, 0),
  successRate: taskStore.statistics.total > 0
    ? Math.round((taskStore.statistics.completed / taskStore.statistics.total) * 100)
    : 0
}))

// 获取下一个要执行的任务
const nextTask = computed(() => {
  const now = new Date()
  const pendingTasks = taskStore.tasks
    .filter(t => t.enabled && new Date(t.nextExecute) > now)
    .sort((a, b) => new Date(a.nextExecute).getTime() - new Date(b.nextExecute).getTime())

  const task = pendingTasks[0]
  if (!task) return null

  return {
    name: task.content.substring(0, 20) + (task.content.length > 20 ? '...' : ''),
    recipient: task.recipient,
    type: task.type,
    nextExecute: formatTime(task.nextExecute),
    enabled: task.enabled
  }
})

// 格式化时间
const formatTime = (time: string) => {
  if (!time) return '-'
  try {
    const date = new Date(time)
    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0')
    const day = String(date.getDate()).padStart(2, '0')
    const hours = String(date.getHours()).padStart(2, '0')
    const minutes = String(date.getMinutes()).padStart(2, '0')
    return `${year}-${month}-${day} ${hours}:${minutes}`
  } catch {
    return '-'
  }
}
</script>

<style scoped>
.home-page {
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
  grid-template-columns: repeat(3, 1fr);
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

.task-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.task-item {
  display: flex;
  align-items: center;
  gap: 24px;
  padding: 32px;
  background: #ffffff;
  border: 1px solid #e5edf5;
  border-radius: 4px;
}

.task-info {
  flex: 1;
}

.task-name {
  font-size: 16px;
  font-weight: 400;
  color: #061b31;
  margin-bottom: 8px;
  letter-spacing: -0.01em;
}

.task-meta {
  display: flex;
  align-items: center;
  gap: 12px;
}

.task-recipient {
  font-size: 14px;
  color: #64748d;
  font-weight: 300;
}

.task-type {
  padding: 4px 12px;
  font-size: 12px;
  font-weight: 400;
  border-radius: 9999px;
  letter-spacing: 0;
}

.task-type.定时 {
  background: #e8e9ff;
  color: #533afd;
}

.task-type.间隔 {
  background: #e8e9ff;
  color: #533afd;
}

.task-schedule {
  font-size: 14px;
  color: #50617a;
  font-weight: 300;
}

.task-toggle {
  width: 44px;
  height: 24px;
  display: flex;
  align-items: center;
  background: #e5edf5;
  border: none;
  border-radius: 9999px;
  cursor: pointer;
  padding: 2px;
  transition: background 0.2s;
}

.task-toggle.active {
  background: #533afd;
}

.toggle-dot {
  width: 20px;
  height: 20px;
  background: #ffffff;
  border-radius: 50%;
  transition: transform 0.2s;
}

.task-toggle.active .toggle-dot {
  transform: translateX(20px);
}

.no-task-hint {
  padding: 48px 32px;
  background: #ffffff;
  border: 1px solid #e5edf5;
  border-radius: 4px;
  text-align: center;
}

.no-task-hint p {
  margin: 0;
  font-size: 16px;
  color: #64748d;
  font-weight: 300;
}
</style>