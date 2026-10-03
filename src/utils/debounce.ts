/// 简易 debounce：连续调用只执行最后一次。
export function debounce<A extends unknown[]>(fn: (...args: A) => void, ms: number) {
  let timer: ReturnType<typeof setTimeout> | undefined
  return (...args: A) => {
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => { fn(...args); timer = undefined }, ms)
  }
}
