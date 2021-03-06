pub type SyncRc<T> = std::sync::Arc<T>;
pub type SyncWeak<T> = std::sync::Weak<T>;
pub type SyncRefMut<T> = crate::sync::Mutex<T>;
