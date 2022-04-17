// Original implementation: https://github.com/kjetilkjeka/interior-mut/blob/master/src/lib.rs

use std::cell::RefCell;
use std::ops::DerefMut;

pub trait InteriorMut<'a, T> {
    type RefMut: DerefMut<Target = T> + 'a;
    type Error;
    fn borrow_int_mut(&'a self) -> Result<Self::RefMut, Self::Error>;
}

impl<'a, T: 'a> InteriorMut<'a, T> for RefCell<T> {
    type RefMut = std::cell::RefMut<'a, T>;
    type Error = std::cell::BorrowMutError;
    fn borrow_int_mut(&'a self) -> Result<Self::RefMut, Self::Error> {
        RefCell::try_borrow_mut(self)
    }
}
impl<'a, T: 'a> InteriorMut<'a, T> for crate::sync::Mutex<T> {
    type RefMut = crate::sync::MutexGuard<'a, T>;
    type Error = ();
    fn borrow_int_mut(&'a self) -> Result<Self::RefMut, Self::Error> {
        self.try_lock().ok_or(())
    }
}
impl<'a, T: 'a> InteriorMut<'a, T> for crate::sync::RwLock<T> {
    type RefMut = crate::sync::RwLockWriteGuard<'a, T>;
    type Error = ();
    fn borrow_int_mut(&'a self) -> Result<Self::RefMut, Self::Error> {
        self.try_write().ok_or(())
    }
}
