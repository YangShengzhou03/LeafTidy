import { onMounted, onUnmounted } from 'vue'
import confetti from 'canvas-confetti'

const COLORS = ['#f8a5c2', '#fbc2eb', '#a6c1ee', '#ffd3b6', '#ffffff']
const DURATION = 3500

/** 页面打开时持续 3.5 秒的礼花绽放效果（左右两侧交替发射） */
export function useConfetti() {
  let timer: number | undefined
  let end = 0

  function burst() {
    confetti({
      particleCount: 18,
      angle: 60,
      spread: 70,
      origin: { x: 0, y: 0.7 },
      colors: COLORS,
    })
    confetti({
      particleCount: 18,
      angle: 120,
      spread: 70,
      origin: { x: 1, y: 0.7 },
      colors: COLORS,
    })
    if (Date.now() < end) {
      timer = window.setTimeout(burst, 180)
    }
  }

  onMounted(() => {
    end = Date.now() + DURATION
    burst()
  })

  onUnmounted(() => {
    if (timer !== undefined) {
      clearTimeout(timer)
    }
    confetti.reset()
  })
}
