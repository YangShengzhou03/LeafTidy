import { invoke } from '@tauri-apps/api/core'

// 全局缩略图批处理队列：把短时间内的多张缩略图请求合并为一次 IPC + 一次 spawn_blocking，
// 避免每张图单独穿越 IPC 边界（LazyThumb 每张图原本各发一次 invoke）。
//
// 调度策略：请求入队后等一个 microtask 聚合窗口；期间同一路径去重；
// 窗口到期后一次 invoke 拿回所有结果，分发给各等待者。
// 若上一轮尚未返回而又有新请求入队，追加到下一轮（pending 标记）。

type Pending = { resolve: (v: string) => void; reject: (e: unknown) => void }

// LRU 上限：缩略图 base64 会长期驻留内存，超过上限时淘汰最久未使用的条目
const MAX_CACHE = 300
const cache = new Map<string, string>()
// 同一路径可能被多个组件同时请求（重复图片/多面板），必须保留所有等待者
const queue = new Map<string, Pending[]>()
let timer: ReturnType<typeof setTimeout> | null = null
let inflight = false

const WAIT_MS = 30

function flush() {
  timer = null
  if (inflight) return // 上一轮还在飞，下一轮调度时再试
  const batch = new Map(queue)
  queue.clear()
  if (batch.size === 0) return

  const paths = [...batch.keys()]
  inflight = true

  invoke<Record<string, string>>('get_thumbnails_batch', { paths })
    .then((result) => {
      for (const [path, waiters] of batch) {
        const thumb = result[path]
        if (thumb) {
          cache.set(path, thumb)
          if (cache.size > MAX_CACHE) {
            const oldest = cache.keys().next().value
            if (oldest !== undefined) cache.delete(oldest)
          }
          waiters.forEach(p => p.resolve(thumb))
        } else {
          waiters.forEach(p => p.reject(new Error('缩略图生成失败')))
        }
      }
    })
    .catch((e) => {
      for (const [, waiters] of batch) waiters.forEach(p => p.reject(e))
    })
    .finally(() => {
      inflight = false
      if (queue.size > 0) schedule() // 执行期间又有新请求
    })
}

function schedule() {
  if (timer) return
  timer = setTimeout(flush, WAIT_MS)
}

export function loadThumbnail(path: string): Promise<string> {
  const hit = cache.get(path)
  if (hit) {
    // 命中后重新插入，维持 LRU 顺序
    cache.delete(path)
    cache.set(path, hit)
    return Promise.resolve(hit)
  }
  return new Promise((resolve, reject) => {
    const waiters = queue.get(path)
    if (waiters) waiters.push({ resolve, reject })
    else queue.set(path, [{ resolve, reject }])
    schedule()
  })
}
