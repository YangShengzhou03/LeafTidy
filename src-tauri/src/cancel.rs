use std::sync::atomic::{AtomicBool, Ordering};

pub static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

pub fn is_cancelled() -> bool {
    CANCEL_FLAG.load(Ordering::SeqCst)
}

pub fn set_cancelled() {
    CANCEL_FLAG.store(true, Ordering::SeqCst);
}

pub fn reset() {
    CANCEL_FLAG.store(false, Ordering::SeqCst);
}
