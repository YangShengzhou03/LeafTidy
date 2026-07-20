<template>
  <div class="log-viewer-page">
    <div class="log-content" v-if="logContent">
      <div
        v-for="(line, index) in paginatedLogLines"
        :key="index"
        class="log-line"
        :class="getLogLineClass(line)"
      >
        <span class="log-line-number">{{ (currentPage - 1) * pageSize + index + 1 }}</span>
        <span class="log-line-content">{{ line }}</span>
      </div>
      <div class="pagination" v-if="filteredLogLines.length > pageSize">
        <el-pagination
          v-model:current-page="currentPage"
          :page-size="pageSize"
          :total="filteredLogLines.length"
          layout="prev, pager, next"
        />
      </div>
    </div>
    <div v-else class="empty-state">
      <div class="empty-text">加载中...</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import { Search } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const route = useRoute()
const fileName = ref<string>('')
const logContent = ref<string>('')
const searchKeyword = ref('')
const logLevel = ref('')
const currentPage = ref(1)
const pageSize = ref(100)

const filteredLogLines = computed(() => {
  if (!logContent.value) return []

  let lines = logContent.value.split('\n').filter(line => line.trim())

  if (logLevel.value) {
    lines = lines.filter(line => line.includes(`[${logLevel.value}]`))
  }

  if (searchKeyword.value) {
    lines = lines.filter(line => line.toLowerCase().includes(searchKeyword.value.toLowerCase()))
  }

  return lines
})

const paginatedLogLines = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return filteredLogLines.value.slice(start, end)
})

const getLogLineClass = (line: string) => {
  return ''
}

const loadLogContent = async () => {
  if (!fileName.value) return

  try {
    const content = await invoke<string>('read_log_file', { fileName: fileName.value })
    logContent.value = content
  } catch (error) {
    console.error('读取日志文件失败:', error)
    ElMessage.error('读取日志文件失败')
  }
}

watch([searchKeyword, logLevel], () => {
  currentPage.value = 1
})

onMounted(() => {
  fileName.value = route.query.file as string || ''
  if (fileName.value) {
    loadLogContent()
  }
})
</script>

<style scoped>
.log-viewer-page {
  background: #ffffff;
}

.log-content {
  font-family: 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.6;
}

.log-line {
  display: flex;
  gap: 12px;
  padding: 4px 0;
  transition: background 0.15s;
}

.log-line:hover {
  background: #f8fafc;
}

.log-line-number {
  color: #94a3b8;
  font-size: 12px;
  min-width: 50px;
  text-align: right;
  user-select: none;
  padding-right: 8px;
  border-right: 1px solid #e5edf5;
}

.log-line-content {
  flex: 1;
  color: #061b31;
  word-break: break-all;
}

.pagination {
  margin-top: 24px;
  display: flex;
  justify-content: center;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 400px;
  text-align: center;
}

.empty-text {
  font-size: 18px;
  color: #64748b;
  font-weight: 300;
}
</style>