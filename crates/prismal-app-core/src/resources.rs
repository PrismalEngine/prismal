use prismal_utils::shared::*;

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct AppResources {
    sync_resources: HashMap<TypeId, Rc<dyn Any + Send + Sync + 'static>>,
    unsync_resources: HashMap<TypeId, UnsyncRc<dyn Any + 'static>>,
}

impl AppResources {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            sync_resources: HashMap::new(),
            unsync_resources: HashMap::new(),
        })
    }

    pub fn insert<T: Any + Send + Sync + 'static>(&mut self, resource: T) {
        let tid = TypeId::of::<T>();
        self.sync_resources.insert(tid, Rc::new(resource));
    }
    pub fn insert_unsync<T: Any + 'static>(&mut self, resource: T) {
        let tid = TypeId::of::<T>();
        self.unsync_resources.insert(tid, UnsyncRc::new(resource));
    }
    pub fn get<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        let tid = TypeId::of::<T>();
        if let Some(r) = self.sync_resources.get(&tid) {
            r.downcast_ref()
        } else {
            None
        }
    }
    pub fn get_unsync<T: Any + 'static>(&self) -> Option<&T> {
        let tid = TypeId::of::<T>();
        if let Some(r) = self.unsync_resources.get(&tid) {
            r.downcast_ref()
        } else {
            None
        }
    }
}
