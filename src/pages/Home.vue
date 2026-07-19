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
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const statistics = ref({
  totalTasks: 15,
  todayExecuted: 128,
  successRate: 98
})

const tasks = ref([
  { id: 1, name: '早会提醒', recipient: '技术部群', type: '定时', nextExecute: '2025-07-20 09:00', enabled: true },
  { id: 2, name: '日报发送', recipient: '工作群', type: '间隔', nextExecute: '2025-07-19 18:00', enabled: true },
  { id: 3, name: '下午茶提醒', recipient: '下午茶群', type: '定时', nextExecute: '2025-07-20 15:00', enabled: false }
])

const nextTask = computed(() => {
  return tasks.value.find(task => task.enabled) || tasks.value[0]
})
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
</style>