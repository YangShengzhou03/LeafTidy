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
import { loadThumbnail } from '@/utils/thumbBatcher'

defineOptions({ name: 'LazyThumb', inheritAttrs: false })

const props = defineProps<{ path: string; crop?: FaceCrop }>()

const src = ref<string>('')
const el = ref<HTMLElement | null>(null)
let observer: IntersectionObserver | null = null

async function load() {
  // 人脸裁剪缩略图走单独命令（带 crop 参数），普通缩略图走批量队列
  if (props.crop) {
    try {
      const thumb = await invoke<string>('get_face_thumbnail', { path: props.path, crop: props.crop })
      src.value = thumb
    } catch {
      // 生成失败时保持占位（由插槽内容兜底）
    }
    return
  }
  try {
    src.value = await loadThumbnail(props.path)
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
