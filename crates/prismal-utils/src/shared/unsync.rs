pub type UnsyncRc<T> = std::rc::Rc<T>;
pub type UnsyncWeak<T> = std::rc::Weak<T>;
pub type UnsyncRefMut<T> = std::cell::RefCell<T>;

pub type UnsyncRcMut<T> = UnsyncRc<UnsyncRefMut<T>>;
pub type UnsyncWeakMut<T> = UnsyncWeak<UnsyncRefMut<T>>;

pub fn unsync_rc_mut<T>(value: T) -> UnsyncRcMut<T> {
    UnsyncRc::new(UnsyncRefMut::new(value))
}
