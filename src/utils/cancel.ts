/// 取消操作的错误标记前缀（与 Rust 端 cancel::CANCELLED_PREFIX 保持一致）
const CANCELLED_PREFIX = '[CANCELLED]'

/// 检查错误消息是否为用户主动取消操作。
/// 替代脆弱的中文字符串模糊匹配，使用结构化前缀检测。
export function isCancelledError(e: unknown): boolean {
  const msg = e instanceof Error ? e.message : String(e)
  return msg.includes(CANCELLED_PREFIX)
}
