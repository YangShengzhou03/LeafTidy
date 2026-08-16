<template>
  <div class="home-page">
    <div class="home-content">
      <div ref="workDirRef" class="directory-section">
        <div class="section-header">
          <span class="section-title">待处理目录</span>
          <el-button size="small" type="primary" @click="selectWorkDirs">添加</el-button>
        </div>
        <div class="directory-list">
          <div v-for="dir in workDirs" :key="dir.path" class="directory-item">
            <el-icon>
              <FolderOpened />
            </el-icon>
            <span class="dir-name">{{ dir.name }}</span>
            <span class="dir-path">{{ dir.path }}</span>
            <el-button text class="remove-btn" @click="removeWorkDir(dir.path)">
              <el-icon>
                <Close />
              </el-icon>
            </el-button>
          </div>
          <div v-if="!workDirs.length" class="empty-area" @click="selectWorkDirs">
            <el-icon>
              <Plus />
            </el-icon>
            <span>点击添加待处理目录</span>
          </div>
        </div>
      </div>

      <div ref="outputDirRef" class="directory-section">
        <div class="section-header">
          <span class="section-title">输出目录</span>
          <el-button size="small" type="primary" @click="selectOutputDir">添加</el-button>
        </div>
        <div class="directory-list">
          <div v-if="outputDir" class="directory-item">
            <el-icon>
              <FolderOpened />
            </el-icon>
            <span class="dir-path">{{ outputDir }}</span>
            <el-button text class="remove-btn" @click="clearOutputDir">
              <el-icon>
                <Close />
              </el-icon>
            </el-button>
          </div>
          <div v-else class="empty-area" @click="selectOutputDir">
            <el-icon>
              <Plus />
            </el-icon>
            <span>点击选择输出目录</span>
          </div>
        </div>
      </div>

      <div ref="quickActionsRef" class="quick-actions">
        <div class="section-title">快速操作</div>
        <div class="action-grid">
          <div class="action-card" @click="goTo('file-organize')">
            <el-icon>
              <FolderOpened />
            </el-icon>
            <span>文件整理</span>
            <p class="desc">按类型、日期、大小自动整理文件</p>
          </div>
          <div class="action-card" @click="goTo('batch-rename')">
            <el-icon>
              <EditPen />
            </el-icon>
            <span>批量重命名</span>
            <p class="desc">用模板批量修改文件名</p>
          </div>
          <div class="action-card" @click="goTo('duplicate-clean')">
            <el-icon>
              <DeleteFilled />
            </el-icon>
            <span>重复清理</span>
            <p class="desc">识别并清理重复文件，释放空间</p>
          </div>
          <div class="action-card" @click="goTo('ai-classify')">
            <el-icon>
              <MagicStick />
            </el-icon>
            <span>AI 智能分类</span>
            <p class="desc">AI 自动识别照片内容并分类</p>
          </div>
          <div class="action-card" @click="goTo('fix-date')">
            <el-icon>
              <Calendar />
            </el-icon>
            <span>修复拍摄时间</span>
            <p class="desc">修正照片错误的拍摄日期</p>
          </div>
        </div>
      </div>
    </div>

    <el-tour v-model="homeTourOpen" type="primary" :mask="true">
      <el-tour-step
        :target="workDirRef"
        title="待处理目录"
        description="在此添加需要整理的文件夹，可以添加多个目录。添加后右侧会显示文件统计信息。"
      />
      <el-tour-step
        :target="outputDirRef"
        title="输出目录"
        description="指定文件整理后的输出位置。建议选择一个空文件夹作为输出目录。"
      />
      <el-tour-step
        :target="quickActionsRef"
        title="快速操作"
        description="点击卡片快速进入对应功能。包括文件整理、批量重命名、重复文件清理和AI智能分类等强大功能。"
      />
    </el-tour>
  </div>
</template>

<script setup lang="ts">
import { inject, watch, ref, onMounted, type Ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { FolderOpened, EditPen, DeleteFilled, MagicStick, Calendar, Plus, Close } from '@element-plus/icons-vue'
import { useFileOps } from '@/composables/useFileOps'
import type { WorkDirectory, FunctionPanel, DirectoryStats } from '@/types'

const activePanel = inject<Ref<FunctionPanel>>('activePanel')!
const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!
const outputDir = inject<Ref<string>>('outputDir')!
const dirStats = inject<Ref<DirectoryStats | null>>('dirStats')!

const workDirRef = ref<HTMLElement>()
const outputDirRef = ref<HTMLElement>()
const quickActionsRef = ref<HTMLElement>()
const homeTourOpen = ref(false)

const { getDirectoryStats } = useFileOps()

onMounted(() => {
  const hasSeenTour = localStorage.getItem('leaftidy_home_tour_seen')
  if (!hasSeenTour) {
    setTimeout(() => {
      homeTourOpen.value = true
    }, 500)
  }
})

watch(homeTourOpen, (open) => {
  if (!open) {
    localStorage.setItem('leaftidy_home_tour_seen', 'true')
  }
})

async function selectWorkDirs() {
  try {
    const selected = await open({
      directory: true,
      multiple: true,
      title: '选择待处理目录',
    })
    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected]
      for (const path of paths) {
        const name = path.split(/[/\\]/).pop() || path
        if (!workDirs.value.some((d) => d.path === path)) {
          workDirs.value.push({ path, name })
        }
      }
    }
  } catch (e) {
    console.error('选择目录失败:', e)
  }
}

function removeWorkDir(path: string) {
  const index = workDirs.value.findIndex((d) => d.path === path)
  if (index >= 0) {
    workDirs.value.splice(index, 1)
  }
}

async function selectOutputDir() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择输出目录',
    })
    if (selected) {
      outputDir.value = selected as string
    }
  } catch (e) {
    console.error('选择目录失败:', e)
  }
}

function clearOutputDir() {
  outputDir.value = ''
}

function goTo(panel: FunctionPanel) {
  activePanel.value = panel
}

watch(workDirs, async (dirs) => {
  if (dirs && dirs.length > 0) {
    try {
      const paths = dirs.map((d) => d.path)
      dirStats.value = await getDirectoryStats(paths)
    } catch (e) {
      console.error('获取统计失败:', e)
      dirStats.value = null
    }
  } else {
    dirStats.value = null
  }
}, { deep: true, immediate: true })
</script>

<style scoped>
.home-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 24px;
  background: #18191C;
  overflow-y: auto;
}

.home-content {
  max-width: 900px;
  margin: 0 auto;
  width: 100%;
}

.directory-section {
  margin-bottom: 20px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.section-title {
  font-size: 13px;
  font-weight: 500;
  color: #C8D0DC;
}

.directory-list {
  background: #1F2023;
  border-radius: 8px;
  padding: 12px;
  min-height: 80px;
  display: flex;
  flex-direction: column;
}

.directory-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: #2A2B30;
  border-radius: 6px;
  margin-bottom: 8px;
}

.directory-item:last-child {
  margin-bottom: 0;
}

.directory-item .el-icon {
  color: #3A86FF;
  font-size: 16px;
}

.dir-name {
  font-size: 13px;
  color: #E0E6ED;
  font-weight: 500;
}

.dir-path {
  font-size: 12px;
  color: #8A94A6;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.remove-btn {
  color: #8A94A6;
  padding: 8px !important;
}

.remove-btn:hover {
  color: #E81123 !important;
}

.empty-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  border: 2px dashed #3A3B40;
  border-radius: 6px;
  color: #8A94A6;
  cursor: pointer;
  transition: all 0.15s;
  padding: 20px;
}

.empty-area:hover {
  border-color: #3A86FF;
  color: #3A86FF;
}

.empty-area .el-icon {
  font-size: 24px;
}

.empty-area span {
  font-size: 13px;
}

.quick-actions {
  margin-top: 32px;
}

.action-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-top: 12px;
}

.action-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px 16px;
  background: #1F2023;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.15s;
}

.action-card:hover {
  background: #2A2B30;
}

.action-card .el-icon {
  font-size: 32px;
  color: #3A86FF;
  margin-bottom: 12px;
}

.action-card span {
  font-size: 13px;
  color: #E0E6ED;
  font-weight: 500;
  margin-bottom: 6px;
}

.action-card .desc {
  font-size: 12px;
  color: #8A94A6;
  text-align: center;
  line-height: 1.5;
}
</style>