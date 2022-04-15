use prismal_utils::shared::*;

use std::any::TypeId;
use std::collections::HashMap;

#[allow(unused_imports)]
use downcast::{Any, AnySync};

#[cfg(target_arch = "wasm32")]
pub trait SyncAppResource: Any {}

#[cfg(not(target_arch = "wasm32"))]
pub trait SyncAppResource: AnySync {}

#[cfg(target_arch = "wasm32")]
impl<T: Any> SyncAppResource for T {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: AnySync> SyncAppResource for T {}

#[cfg(target_arch = "wasm32")]
downcast::downcast!(dyn SyncAppResource);

#[cfg(not(target_arch = "wasm32"))]
downcast::downcast_sync!(dyn SyncAppResource);

pub struct AppResources {
    sync_resources: HashMap<TypeId, Rc<dyn SyncAppResource>>,
    unsync_resources: HashMap<TypeId, UnsyncRc<dyn Any + 'static>>,
}

impl AppResources {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            sync_resources: HashMap::new(),
            unsync_resources: HashMap::new(),
        })
    }

    pub fn insert<T: SyncAppResource>(&mut self, resource: T) {
        let tid = TypeId::of::<T>();
        self.sync_resources.insert(tid, Rc::new(resource));
    }
    pub fn insert_unsync<T: Any>(&mut self, resource: T) {
        let tid = TypeId::of::<T>();
        self.unsync_resources.insert(tid, UnsyncRc::new(resource));
    }

    pub fn get<T: SyncAppResource>(&self) -> Option<&T> {
        let tid = TypeId::of::<T>();
        if let Some(r) = self.sync_resources.get(&tid) {
            r.downcast_ref().ok()
        } else {
            None
        }
    }
    pub fn get_unsync<T: Any + 'static>(&self) -> Option<&T> {
        let tid = TypeId::of::<T>();
        if let Some(r) = self.unsync_resources.get(&tid) {
            r.downcast_ref().ok()
        } else {
            None
        }
    }
}
