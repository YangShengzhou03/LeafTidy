<template>
  <el-container style="height: 100vh;">
    <el-aside width="200px" style="background: #545c64;">
      <div style="height: 60px; display: flex; align-items: center; justify-content: center; color: #fff; font-size: 18px; font-weight: bold;">
        轻羽大师版
      </div>
      <el-menu
        :default-active="activeMenu"
        router
        style="border-right: none; background: #545c64;"
        text-color="#fff"
        active-text-color="#ffd04b"
      >
        <el-menu-item index="/">
          <el-icon><HomeFilled /></el-icon>
          <span>首页</span>
        </el-menu-item>
        <el-menu-item index="/auto-message">
          <el-icon><ChatDotRound /></el-icon>
          <span>自动信息</span>
        </el-menu-item>
        <el-menu-item index="/settings">
          <el-icon><Setting /></el-icon>
          <span>设置</span>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <el-main style="background: #f5f5f5; overflow-y: auto; padding: 20px;">
      <router-view />
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { HomeFilled, ChatDotRound, Setting } from '@element-plus/icons-vue'

const route = useRoute()
const activeMenu = computed(() => route.path)
</script>