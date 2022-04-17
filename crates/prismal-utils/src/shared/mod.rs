mod sync;
mod unsync;

pub use sync::*;
pub use unsync::*;

pub type Rc<T> = SyncRc<T>;
pub type Weak<T> = SyncWeak<T>;
pub type RefMut<T> = SyncRefMut<T>;

pub type RcMut<T> = Rc<RefMut<T>>;
pub type WeakMut<T> = Weak<RefMut<T>>;

pub fn rc_mut<T>(value: T) -> RcMut<T> {
    Rc::new(RefMut::new(value))
}
