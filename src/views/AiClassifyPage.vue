<template>
  <div class="page-container">
    <div class="page-header">
      <h2>AI 智能分类</h2>
      <p class="desc">使用本地AI模型自动识别照片内容并分类</p>
    </div>
    <div class="page-content">
      <div class="coming-soon">
        <el-icon class="soon-icon"><Clock /></el-icon>
        <span class="soon-text">功能开发中，敬请期待...</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Clock } from '@element-plus/icons-vue'
</script>

<style scoped>
.page-container {
  height: 100%;
  background: #18191C;
  padding: 24px;
  overflow-y: auto;
}

.page-header {
  margin-bottom: 24px;
}

.page-header h2 {
  font-size: 16px;
  font-weight: 500;
  color: #E0E6ED;
  margin-bottom: 6px;
}

.page-header .desc {
  font-size: 13px;
  color: #8A94A6;
}

.page-content {
  display: flex;
  justify-content: center;
  align-items: center;
  height: calc(100% - 80px);
}

.coming-soon {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  padding: 40px;
  background: #1F2023;
  border-radius: 12px;
}

.soon-icon {
  font-size: 48px;
  color: #3A86FF;
}

.soon-text {
  font-size: 14px;
  color: #8A94A6;
}
</style>