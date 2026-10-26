import { computed, type Ref } from 'vue'
import type { OperationResult } from '@/types'

/**
 * 操作结果统计 —— 替代各页面重复的 successCount/failCount/successResults/failResults
 */
export function useOperationResults<T extends OperationResult>(results: Ref<T[]>) {
  const stats = computed(() => {
    let success = 0
    let fail = 0
    const successList: T[] = []
    const failList: T[] = []

    for (const r of results.value) {
      if (r.success) {
        success++
        successList.push(r)
      } else {
        fail++
        failList.push(r)
      }
    }

    return { successCount: success, failCount: fail, successResults: successList, failResults: failList }
  })

  return {
    successCount: computed(() => stats.value.successCount),
    failCount: computed(() => stats.value.failCount),
    successResults: computed(() => stats.value.successResults),
    failResults: computed(() => stats.value.failResults),
  }
}
