<template>
  <div class="home-page">
    <div class="home-content">
      <div ref="workDirRef" class="directory-section">
        <div class="section-header">
          <span class="section-title">{{ t('home.workDirs.title') }}</span>
          <div class="header-actions">
            <el-button @click="openSmbDialog">
              {{ t('home.smb.connectBtn') }}
            </el-button>
            <el-button type="primary" @click="selectWorkDirs">{{ t('home.add') }}</el-button>
          </div>
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
            <span>{{ t('home.workDirs.empty') }}</span>
          </div>
        </div>
      </div>

      <div ref="outputDirRef" class="directory-section">
        <div class="section-header">
          <span class="section-title">{{ t('home.outputDir.title') }}</span>
          <el-button type="primary" @click="selectOutputDir">{{ t('home.add') }}</el-button>
        </div>
        <div class="directory-list">
          <div v-if="outputDir" class="directory-item">
            <el-icon>
              <FolderOpened />
            </el-icon>
            <span class="dir-name">{{ outputDirName }}</span>
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
            <span>{{ t('home.outputDir.empty') }}</span>
          </div>
        </div>
      </div>

      <div ref="quickActionsRef" class="quick-actions">
        <div class="section-title">{{ t('home.quickActions.title') }}</div>
        <div class="action-grid">
          <div class="action-card" @click="goTo('file-organize')">
            <el-icon>
              <FolderOpened />
            </el-icon>
            <span>{{ t('home.action.organize') }}</span>
            <p class="desc">{{ t('home.action.organizeDesc') }}</p>
          </div>
          <div class="action-card" @click="goTo('batch-rename')">
            <el-icon>
              <EditPen />
            </el-icon>
            <span>{{ t('home.action.rename') }}</span>
            <p class="desc">{{ t('home.action.renameDesc') }}</p>
          </div>
          <div class="action-card" @click="goTo('duplicate-clean')">
            <el-icon>
              <DeleteFilled />
            </el-icon>
            <span>{{ t('home.action.duplicates') }}</span>
            <p class="desc">{{ t('home.action.duplicatesDesc') }}</p>
          </div>
          <div class="action-card" @click="goTo('ai-classify')">
            <el-icon>
              <MagicStick />
            </el-icon>
            <span>{{ t('home.action.aiClassify') }}</span>
            <p class="desc">{{ t('home.action.aiClassifyDesc') }}</p>
          </div>
          <div class="action-card" @click="goTo('fix-date')">
            <el-icon>
              <Calendar />
            </el-icon>
            <span>{{ t('home.action.fixDate') }}</span>
            <p class="desc">{{ t('home.action.fixDateDesc') }}</p>
          </div>
        </div>
      </div>
    </div>

    <el-tour v-model="homeTourOpen" type="primary" :mask="true">
      <el-tour-step
        :target="workDirRef"
        :title="t('home.workDirs.title')"
        :description="t('home.tour.workDirs')"
      />
      <el-tour-step
        :target="outputDirRef"
        :title="t('home.outputDir.title')"
        :description="t('home.tour.outputDir')"
      />
      <el-tour-step
        :target="quickActionsRef"
        :title="t('home.quickActions.title')"
        :description="t('home.tour.quickActions')"
      />
    </el-tour>

    <!-- SMB 连接对话框 -->
    <el-dialog
      v-model="smbDialogVisible"
      :title="t('home.smb.dialogTitle')"
      width="480px"
      :close-on-click-modal="!smbConnecting"
      :show-close="!smbConnecting"
      :append-to-body="true"
    >
      <el-form :model="smbForm" label-width="100px" @submit.prevent>
        <el-form-item :label="t('home.smb.server')" required>
          <el-input
            v-model="smbForm.server"
            :placeholder="t('home.smb.serverPlaceholder')"
            :disabled="smbConnecting"
          />
        </el-form-item>
        <el-form-item :label="t('home.smb.share')" required>
          <el-input
            v-model="smbForm.share"
            :placeholder="t('home.smb.sharePlaceholder')"
            :disabled="smbConnecting"
          />
        </el-form-item>
        <el-form-item :label="t('home.smb.username')">
          <el-input
            v-model="smbForm.username"
            :disabled="smbConnecting"
            autocomplete="off"
          />
        </el-form-item>
        <el-form-item :label="t('home.smb.password')">
          <el-input
            v-model="smbForm.password"
            type="password"
            :disabled="smbConnecting"
            show-password
            autocomplete="off"
          />
        </el-form-item>
        <el-form-item>
          <el-checkbox v-model="smbForm.persistent" :disabled="smbConnecting">
            {{ t('home.smb.persistent') }}
          </el-checkbox>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="smbDialogVisible = false" :disabled="smbConnecting">
          {{ t('common.cancel') }}
        </el-button>
        <el-button type="primary" @click="connectSmb" :loading="smbConnecting">
          {{ smbConnecting ? t('home.smb.connecting') : t('home.smb.connectBtn') }}
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { inject, watch, ref, computed, onMounted, type Ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { FolderOpened, EditPen, DeleteFilled, MagicStick, Calendar, Plus, Close } from '@element-plus/icons-vue'
import { ElNotification, ElMessage } from 'element-plus'
import { useFileOps } from '@/composables/useFileOps'
import { useLog } from '@/composables/useLog'
import { debounce } from '@/utils/debounce'
import { t } from '@/i18n'
import type { WorkDirectory, FunctionPanel, DirectoryStats, SMBConnection, SMBConnectionResult } from '@/types'

const activePanel = inject<Ref<FunctionPanel>>('activePanel')!
const workDirs = inject<Ref<WorkDirectory[]>>('workDirs')!
const outputDir = inject<Ref<string>>('outputDir')!
const dirStats = inject<Ref<DirectoryStats | null>>('dirStats')!

const outputDirName = computed(() => outputDir.value.split(/[/\\]/).pop() || outputDir.value)

const workDirRef = ref<HTMLElement>()
const outputDirRef = ref<HTMLElement>()
const quickActionsRef = ref<HTMLElement>()
const homeTourOpen = ref(false)

const { getDirectoryStats } = useFileOps()

// ===== SMB 连接 =====
const smbDialogVisible = ref(false)
const smbConnecting = ref(false)
const smbForm = ref<SMBConnection>({
  server: '',
  share: '',
  username: '',
  password: '',
  persistent: true,
})

async function openSmbDialog() {
  smbForm.value = { server: '', share: '', username: '', password: '', persistent: true }
  smbDialogVisible.value = true
}

async function connectSmb() {
  if (!smbForm.value.server.trim()) {
    ElMessage.warning(t('home.smb.invalidServer'))
    return
  }
  if (!smbForm.value.share.trim()) {
    ElMessage.warning(t('home.smb.invalidShare'))
    return
  }

  smbConnecting.value = true
  try {
    const result = await invoke<SMBConnectionResult>('smb_connect', {
      server: smbForm.value.server.trim(),
      share: smbForm.value.share.trim(),
      username: smbForm.value.username?.trim() || undefined,
      password: smbForm.value.password || undefined,
      persistent: smbForm.value.persistent,
    })

    // 自动添加到工作目录
    if (!workDirs.value.some((d) => d.path === result.unc_path)) {
      const shareName = smbForm.value.share.trim()
      workDirs.value.push({ path: result.unc_path, name: shareName })
      useLog().logEvent(t('home.log.smbConnect', { path: result.unc_path }))
    }

    ElNotification({
      type: 'success',
      title: t('home.smb.connectSuccess', { path: result.unc_path }),
      duration: 3000,
    })
    smbDialogVisible.value = false
  } catch (e: any) {
    ElNotification({
      type: 'error',
      title: t('home.smb.connectFailed', { msg: String(e) }),
      duration: 5000,
    })
  } finally {
    smbConnecting.value = false
  }
}

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
      title: t('home.dialog.selectWorkDirs'),
    })
    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected]
      for (const path of paths) {
        const name = path.split(/[/\\]/).pop() || path
        if (!workDirs.value.some((d) => d.path === path)) {
          workDirs.value.push({ path, name })
          useLog().logEvent(t('home.log.addDir', { name }))
        }
      }
    }
  } catch (e) {
    ElNotification({ type: 'error', message: t('home.error.selectDir') })
  }
}

function removeWorkDir(path: string) {
  const index = workDirs.value.findIndex((d) => d.path === path)
  if (index >= 0) {
    const name = workDirs.value[index].name
    workDirs.value.splice(index, 1)
    useLog().logEvent(t('home.log.removeDir', { name }))
  }
}

async function selectOutputDir() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: t('home.dialog.selectOutputDir'),
    })
    if (selected) {
      outputDir.value = selected as string
    }
  } catch (e) {
    ElNotification({ type: 'error', message: t('home.error.selectDir') })
  }
}

function clearOutputDir() {
  outputDir.value = ''
}

function goTo(panel: FunctionPanel) {
  activePanel.value = panel
}

let statsSeq = 0
const updateStats = debounce(async (dirs: WorkDirectory[] | undefined) => {
  const seq = ++statsSeq
  if (dirs && dirs.length > 0) {
    try {
      const paths = dirs.map((d) => d.path)
      const stats = await getDirectoryStats(paths)
      if (seq === statsSeq) dirStats.value = stats
    } catch (e) {
      if (seq === statsSeq) {
        ElNotification({ type: 'error', message: t('home.error.stats') })
        dirStats.value = null
      }
    }
  } else {
    dirStats.value = null
  }
}, 200)
watch(workDirs, updateStats, { deep: true, immediate: true })
</script>

<style scoped>
.home-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 24px;
  background: var(--bg);
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

.header-actions {
  display: flex;
  gap: 8px;
}

.section-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.directory-list {
  background: var(--panel);
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
  background: var(--panel-2);
  border-radius: 6px;
  margin-bottom: 8px;
}

.directory-item:last-child {
  margin-bottom: 0;
}

.directory-item .el-icon {
  color: var(--primary);
  font-size: 16px;
}

.dir-name {
  font-size: 13px;
  color: var(--text);
  font-weight: 500;
}

.dir-path {
  font-size: 12px;
  color: var(--text-muted);
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.remove-btn {
  color: var(--text-muted);
  padding: 8px;
}

.remove-btn:hover {
  color: var(--danger);
}

.empty-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  border: 2px dashed var(--border);
  border-radius: 6px;
  color: var(--text-muted);
  cursor: pointer;
  transition: all 0.15s;
  padding: 20px;
}

.empty-area:hover {
  border-color: var(--primary);
  color: var(--primary);
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
  background: var(--panel);
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.15s;
}

.action-card:hover {
  background: var(--panel-2);
}

.action-card .el-icon {
  font-size: 32px;
  color: var(--primary);
  margin-bottom: 12px;
}

.action-card span {
  font-size: 13px;
  color: var(--text);
  font-weight: 500;
  margin-bottom: 6px;
}

.action-card .desc {
  font-size: 12px;
  color: var(--text-muted);
  text-align: center;
  line-height: 1.5;
}
</style>