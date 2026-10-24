<template>
  <span ref="el" class="lazy-thumb-anchor">
    <img v-if="src" class="lazy-thumb" :src="src" :class="$attrs.class" alt="" />
    <slot v-else />
  </span>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { FaceCrop } from '@/types'

defineOptions({ name: 'LazyThumb', inheritAttrs: false })

const props = defineProps<{ path: string; crop?: FaceCrop }>()

// 进程内缓存：同一路径（或同一人脸区域）只生成一次缩略图
const cache = new Map<string, string>()

const src = ref<string>('')
const el = ref<HTMLElement | null>(null)
let observer: IntersectionObserver | null = null

async function load() {
  const key = props.crop
    ? `${props.path}|${props.crop.x},${props.crop.y},${props.crop.w},${props.crop.h}`
    : props.path
  const cached = cache.get(key)
  if (cached) {
    src.value = cached
    return
  }
  try {
    const thumb = props.crop
      ? await invoke<string>('get_face_thumbnail', { path: props.path, crop: props.crop })
      : await invoke<string>('get_thumbnail', { path: props.path })
    cache.set(key, thumb)
    src.value = thumb
  } catch {
    // 生成失败时保持占位（由插槽内容兜底）
  }
}

onMounted(() => {
  // display:contents 的元素本身没有盒子，IntersectionObserver 观察不到，
  // 因此观察它的父容器（每条记录自己的卡片/行），滚到可见时才读盘
  const target = el.value?.parentElement
  if (!target) {
    load()
    return
  }
  observer = new IntersectionObserver((entries) => {
    if (entries.some(e => e.isIntersecting)) {
      observer?.disconnect()
      observer = null
      load()
    }
  })
  observer.observe(target)
})

onBeforeUnmount(() => {
  observer?.disconnect()
  observer = null
})
</script>

<style scoped>
.lazy-thumb-anchor {
  display: contents;
}
</style>
