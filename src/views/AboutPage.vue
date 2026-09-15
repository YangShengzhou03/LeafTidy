<template>
  <div class="about-page">
    <div class="about-content">
      <h1 class="page-title">{{ t('about.title') }}</h1>
      <p class="page-meta">{{ t('about.meta') }}</p>

      <div class="about-section">
        <div class="section-title">{{ t('about.s1.title') }}</div>
        <div class="section-content">
          <p>{{ t('about.s1p1') }}</p>
        </div>
      </div>

      <div class="about-section">
        <div class="section-title">{{ t('about.s2.title') }}</div>
        <div class="section-content">
          <p><strong>{{ t('about.s2l1a') }}</strong>{{ t('about.s2l1b') }}</p>
          <p><strong>{{ t('about.s2l2a') }}</strong>{{ t('about.s2l2b') }}</p>
          <p><strong>{{ t('about.s2l3a') }}</strong>{{ t('about.s2l3b') }}</p>
          <p><strong>{{ t('about.s2l4a') }}</strong>{{ t('about.s2l4b') }}</p>
          <p><strong>{{ t('about.s2l5a') }}</strong>{{ t('about.s2l5b') }}</p>
          <p><strong>{{ t('about.s2l6a') }}</strong>{{ t('about.s2l6b') }}</p>
        </div>
      </div>

      <div class="about-section">
        <div class="section-title">{{ t('about.s3.title') }}</div>
        <div class="section-content">
          <p>{{ t('about.s3l1') }}</p>
          <p>{{ t('about.s3l2') }}</p>
          <p>{{ t('about.s3l3') }}</p>
        </div>
      </div>

      <div class="about-section">
        <div class="section-title">{{ t('about.s4.title') }}</div>
        <div class="section-content">
          <p>{{ t('about.s4p1') }}</p>
          <p>{{ t('about.s4p2') }}</p>
          <p>{{ t('about.s4p3') }}</p>
          <p>{{ t('about.s4l1') }}YangSZ03@foxmail.com</p>
          <p>GitHub：{{ GITHUB_URL }}</p>
          <p>Gitee：{{ GITEE_URL }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { t } from '@/i18n'
import { useConfetti } from '@/composables/useConfetti'

useConfetti()

const GITHUB_URL = 'https://github.com/YangShengzhou03'
const GITEE_URL = 'https://gitee.com/Yangshengzhou/yang-shengzhou'
</script>

<style scoped>
.about-page {
  height: 100%;
  background: var(--bg);
  overflow-y: auto;
}

.about-content {
  max-width: 800px;
  margin: 0 auto;
  padding: 24px;
}

.page-title {
  font-size: 20px;
  font-weight: 500;
  color: var(--text);
  margin: 0 0 8px 0;
  text-align: center;
}

.page-meta {
  font-size: 12px;
  color: var(--text-muted);
  text-align: center;
  margin-bottom: 28px;
}

.about-section {
  margin-bottom: 24px;
}

.section-title {
  font-size: 15px;
  font-weight: 500;
  color: var(--text);
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--panel-2);
}

.section-content {
  color: var(--text-secondary);
  font-size: 13px;
  line-height: 1.8;
}

.section-content p {
  margin-bottom: 12px;
}

.section-content strong {
  color: var(--text);
  font-weight: 600;
}
</style>
