use std::sync::atomic::{AtomicBool, Ordering};

/// 全局操作取消标志
pub static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

/// 检查操作是否被取消
pub fn is_cancelled() -> bool {
    CANCEL_FLAG.load(Ordering::SeqCst)
}

/// 设置取消标志
pub fn set_cancelled() {
    CANCEL_FLAG.store(true, Ordering::SeqCst);
}

/// 重置取消标志
pub fn reset() {
    CANCEL_FLAG.store(false, Ordering::SeqCst);
}
