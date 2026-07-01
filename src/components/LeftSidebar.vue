<template>
  <div class="left-sidebar">
    <div class="menu-list">
      <div class="menu-item" :class="{ active: activePanel === 'home' }" @click="selectFunction('home')">
        <el-icon>
          <HomeFilled />
        </el-icon>
        <span>首页</span>
      </div>

      <div class="menu-group-title">通用文件工具</div>
      <div class="menu-item" :class="{ active: activePanel === 'file-organize' }"
        @click="selectFunction('file-organize')">
        <el-icon>
          <FolderOpened />
        </el-icon>
        <span>文件目录整理</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'batch-rename' }"
        @click="selectFunction('batch-rename')">
        <el-icon>
          <EditPen />
        </el-icon>
        <span>批量重命名</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'duplicate-clean' }"
        @click="selectFunction('duplicate-clean')">
        <el-icon>
          <DeleteFilled />
        </el-icon>
        <span>重复文件清理</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'cleanup' }" @click="selectFunction('cleanup')">
        <el-icon>
          <Delete />
        </el-icon>
        <span>附属文件清理</span>
      </div>

      <div class="menu-group-title">照片与智能整理</div>
      <div class="menu-item" :class="{ active: activePanel === 'fix-date' }" @click="selectFunction('fix-date')">
        <el-icon>
          <Calendar />
        </el-icon>
        <span>修复拍摄时间</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'exif-clean' }" @click="selectFunction('exif-clean')">
        <el-icon>
          <Hide />
        </el-icon>
        <span>照片脱敏</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'write-gps' }" @click="selectFunction('write-gps')">
        <el-icon>
          <Location />
        </el-icon>
        <span>写入地理信息</span>
      </div>
      <div class="menu-item" :class="{ active: activePanel === 'ai-classify' }" @click="selectFunction('ai-classify')">
        <el-icon>
          <MagicStick />
        </el-icon>
        <span>AI 智能分类</span>
      </div>

      <div class="menu-item" :class="{ active: activePanel === 'log-view' }" @click="selectFunction('log-view')">
        <el-icon>
          <Document />
        </el-icon>
        <span>日志</span>
      </div>
    </div>

    <div class="menu-footer">
      <div class="menu-item" :class="{ active: activePanel === 'settings' }" @click="selectFunction('settings')">
        <el-icon>
          <Setting />
        </el-icon>
        <span>设置</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
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
  color: #C8D0DC;
  font-size: 13px;
  transition: background 0.15s;
  border-radius: 6px;
}

.menu-item:hover {
  background: #2A2B30;
  color: #E0E6ED;
}

.menu-item.active {
  background: #353639;
  color: #FFFFFF;
}

.menu-item .el-icon {
  font-size: 16px;
}

.menu-group-title {
  padding: 16px 12px 8px;
  font-size: 11px;
  color: #8A94A6;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.menu-footer {
  padding: 12px;
  border-top: 1px solid #2A2B30;
}
</style>