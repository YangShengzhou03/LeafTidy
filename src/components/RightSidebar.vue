<template>
  <div class="right-sidebar">
    <div v-if="!dirStats" class="empty-state">
      <el-icon :size="32">
        <InfoFilled />
      </el-icon>
      <p>请选择待处理目录</p>
    </div>
    <div v-else class="detail-content">
      <div class="stats-section">
        <div class="stats-row">
          <span class="stats-label">文件总数</span>
          <span class="stats-value">{{ dirStats.total_files }}</span>
        </div>
        <div class="stats-row">
          <span class="stats-label">目录总数</span>
          <span class="stats-value">{{ dirStats.total_dirs }}</span>
        </div>
        <div class="stats-row">
          <span class="stats-label">总大小</span>
          <span class="stats-value">{{ formatSize(dirStats.total_size) }}</span>
        </div>
        <div class="stats-row">
          <span class="stats-label">最旧文件</span>
          <span class="stats-value">{{ dirStats.oldest_file || '无' }}</span>
        </div>
        <div class="stats-row">
          <span class="stats-label">最新文件</span>
          <span class="stats-value">{{ dirStats.newest_file || '无' }}</span>
        </div>
        <div v-if="Object.keys(dirStats.file_types).length > 0" class="file-types">
          <div class="section-subtitle">文件类型分布</div>
          <div ref="chartContainer" class="chart-container"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, watch, ref, onMounted, nextTick, type Ref } from 'vue'
import { InfoFilled } from '@element-plus/icons-vue'
import * as echarts from 'echarts'
import type { DirectoryStats } from '@/types'

const dirStats = inject<Ref<DirectoryStats | null>>('dirStats')!
const chartContainer = ref<HTMLElement | null>(null)
let chartInstance: echarts.ECharts | null = null

function formatSize(size: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  let s = size
  let i = 0
  while (s >= 1024 && i < units.length - 1) {
    s /= 1024
    i++
  }
  return i === 0 ? s + ' ' + units[i] : s.toFixed(2) + ' ' + units[i]
}

function initChart() {
  if (!chartContainer.value || !dirStats.value) return

  if (chartInstance) {
    chartInstance.dispose()
  }

  chartInstance = echarts.init(chartContainer.value, 'dark')

  const fileTypes = dirStats.value.file_types
  const data = Object.entries(fileTypes).map(([type, count]) => ({
    name: type.toUpperCase(),
    value: count,
  }))

  const option = {
    backgroundColor: 'transparent',
    tooltip: {
      trigger: 'item',
      formatter: '{b}: {c} 个 ({d}%)',
    },
    legend: {
      orient: 'horizontal',
      bottom: 0,
      left: 'center',
      textStyle: {
        color: '#C8D0DC',
        fontSize: 11,
      },
      itemWidth: 12,
      itemHeight: 12,
      itemGap: 12,
    },
    series: [
      {
        type: 'pie',
        radius: ['40%', '70%'],
        center: ['50%', '40%'],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 4,
          borderColor: '#1F2023',
          borderWidth: 2,
        },
        label: {
          show: false,
        },
        emphasis: {
          label: {
            show: true,
            fontSize: 12,
            fontWeight: 'bold',
            color: '#E0E6ED',
          },
        },
        labelLine: {
          show: false,
        },
        data: data,
      },
    ],
    color: ['#3A86FF', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899', '#06B6D4', '#84CC16'],
  }

  chartInstance.setOption(option)
}

onMounted(() => {
  if (dirStats.value && Object.keys(dirStats.value.file_types).length > 0) {
    nextTick(() => {
      initChart()
    })
  }
})

watch(
  () => dirStats.value,
  (newStats) => {
    if (newStats && Object.keys(newStats.file_types).length > 0) {
      nextTick(() => {
        initChart()
      })
    }
  },
  { deep: true }
)
</script>

<style scoped>
.right-sidebar {
  height: 100%;
  background: #1F2023;
  overflow-y: auto;
  padding: 16px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #8A94A6;
  gap: 8px;
}

.empty-state p {
  font-size: 13px;
}

.stats-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.stats-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid #2A2B30;
  gap: 12px;
}

.stats-row:last-child {
  border-bottom: none;
}

.stats-label {
  flex-shrink: 0;
  font-size: 12px;
  color: #8A94A6;
}

.stats-value {
  font-size: 12px;
  color: #E0E6ED;
  font-weight: 500;
  text-align: right;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 180px;
}

.file-types {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #2A2B30;
}

.chart-container {
  width: 100%;
  height: 200px;
  margin-top: 8px;
}

.section-subtitle {
  font-size: 12px;
  color: #8A94A6;
  font-weight: 500;
  margin-bottom: 8px;
}
</style>
