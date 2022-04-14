mod sync;
mod unsync;

pub use sync::*;
pub use unsync::*;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub type Rc<T> = UnsyncRc<T>;
        pub type Weak<T> = UnsyncWeak<T>;
        pub type RefMut<T> = UnsyncRefMut<T>;
    } else {
        pub type Rc<T> = SyncRc<T>;
        pub type Weak<T> = SyncWeak<T>;
        pub type RefMut<T> = SyncRefMut<T>;
    }
}

pub type RcMut<T> = Rc<RefMut<T>>;
pub type WeakMut<T> = Weak<RefMut<T>>;

pub fn rc_mut<T>(value: T) -> RcMut<T> {
    Rc::new(RefMut::new(value))
}
