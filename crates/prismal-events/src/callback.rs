use prismal_utils::shared::{rc_mut, RcMut};

use crate::event::base_event::Event;

pub type RcEventCallback = RcMut<dyn FnMut(&Event) + Send + Sync>;

pub struct EventCallback {
    pub callback: RcEventCallback,
}

impl EventCallback {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut(&Event) + Send + Sync + 'static,
    {
        Self {
            callback: rc_mut(f),
        }
    }
}
