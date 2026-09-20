<template>
  <div class="left-sidebar">
    <div class="menu-list">
      <div class="menu-item" :class="{ active: activePanel === 'home' }" @click="selectFunction('home')">
        <el-icon>
          <HomeFilled />
        </el-icon>
        <span>{{ t('sidebar.left.home') }}</span>
      </div>

      <div class="menu-group-title">{{ t('sidebar.left.groupCommon') }}</div>
      <div class="menu-item" :class="{ active: activePanel === 'file-organize' }"
        @click="selectFunction('file-organize')">
        <el-icon>
          <FolderOpened />
        </el-icon>
        <span>{{ t('sidebar.left.fileOrganize') }}</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'batch-rename' }"
        @click="selectFunction('batch-rename')">
        <el-icon>
          <EditPen />
        </el-icon>
        <span>{{ t('sidebar.left.batchRename') }}</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'duplicate-clean' }"
        @click="selectFunction('duplicate-clean')">
        <el-icon>
          <DeleteFilled />
        </el-icon>
        <span>{{ t('sidebar.left.duplicateClean') }}</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'cleanup' }" @click="selectFunction('cleanup')">
        <el-icon>
          <Delete />
        </el-icon>
        <span>{{ t('sidebar.left.associatedCleanup') }}</span>
      </div>

      <div class="menu-group-title">{{ t('sidebar.left.groupPhoto') }}</div>
      <div class="menu-item" :class="{ active: activePanel === 'fix-date' }" @click="selectFunction('fix-date')">
        <el-icon>
          <Calendar />
        </el-icon>
        <span>{{ t('sidebar.left.fixDate') }}</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'exif-clean' }" @click="selectFunction('exif-clean')">
        <el-icon>
          <Hide />
        </el-icon>
        <span>{{ t('sidebar.left.exifClean') }}</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'write-gps' }" @click="selectFunction('write-gps')">
        <el-icon>
          <Location />
        </el-icon>
        <span>{{ t('sidebar.left.writeGps') }}</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'ai-classify' }" @click="selectFunction('ai-classify')">
        <el-icon>
          <MagicStick />
        </el-icon>
        <span>{{ t('sidebar.left.aiClassify') }}</span>
      </div>

      <div class="menu-item" :class="{ active: activePanel === 'log-view' }" @click="selectFunction('log-view')">
        <el-icon>
          <Document />
        </el-icon>
        <span>{{ t('sidebar.left.logView') }}</span>
      </div>
    </div>

    <div class="menu-footer">
      <div class="menu-item" :class="{ active: activePanel === 'settings' }" @click="selectFunction('settings')">
        <el-icon>
          <Setting />
        </el-icon>
        <span>{{ t('sidebar.left.settings') }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
import { t } from '@/i18n'
import type { FunctionPanel } from '@/types'
import { HomeFilled, FolderOpened, EditPen, DeleteFilled, Delete, Calendar, Hide, Location, MagicStick, Document, Setting } from '@element-plus/icons-vue'

const activePanel = inject<Ref<FunctionPanel>>('activePanel')!

function selectFunction(id: FunctionPanel) {
  activePanel.value = id
}
</script>

<style scoped>
.left-sidebar {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.menu-list {
  flex: 1;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 12px;
  cursor: pointer;
  color: var(--text-secondary);
  font-size: 13px;
  transition: background 0.15s;
  border-radius: 6px;
}

.menu-item:hover {
  background: var(--panel-2);
  color: var(--text);
}

.menu-item.active {
  background: var(--panel-2-hover);
  color: var(--white);
}

:root[data-theme='light'] .menu-item.active {
  background: var(--text);
  color: var(--white);
}

.menu-item .el-icon {
  font-size: 16px;
}

.menu-group-title {
  padding: 16px 12px 8px;
  font-size: 11px;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.menu-footer {
  padding: 12px;
  border-top: 1px solid var(--panel-2);
}
</style>