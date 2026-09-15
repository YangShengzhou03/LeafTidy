<template>
  <div class="page-container">
    <div class="page-header">
      <h2>{{ t('ai.title') }}</h2>
      <p class="desc">{{ t('ai.desc') }}</p>
    </div>
    <div class="page-content">
      <div class="coming-soon">
        <el-icon class="soon-icon">
          <Clock />
        </el-icon>
        <span class="soon-text">{{ t('ai.comingSoon') }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Clock } from '@element-plus/icons-vue'
import { t } from '@/i18n'
</script>

<style scoped>
.page-container {
  height: 100%;
  background: var(--bg);
  padding: 24px;
  overflow-y: auto;
}

.page-header {
  margin-bottom: 24px;
}

.page-header h2 {
  font-size: 16px;
  font-weight: 500;
  color: var(--text);
  margin-bottom: 6px;
}

.page-header .desc {
  font-size: 13px;
  color: var(--text-muted);
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
  background: var(--panel);
  border-radius: 12px;
}

.soon-icon {
  font-size: 48px;
  color: var(--primary);
}

.soon-text {
  font-size: 14px;
  color: var(--text-muted);
}
</style>