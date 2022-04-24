use std::collections::HashSet;

use prismal_utils::hash::int::IntHasherBuilder;

use crate::callback::EventCallback;
use crate::event::base_event::Event;

pub struct EventManager {
    pressed_keys: HashSet<u32, IntHasherBuilder>,
    queue: Vec<Event>,
    callbacks: Vec<EventCallback>,
}

impl EventManager {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self {
            pressed_keys: HashSet::default(),
            queue: Vec::with_capacity(8),
            callbacks: Vec::new(),
        }
    }
    pub fn push(&mut self, event: Event) {
        if let Some(data) = event.as_keyboard() {
            let scancode = data.scancode;
            if data.state.is_pressed() {
                if !self.pressed_keys.contains(&scancode) {
                    self.pressed_keys.insert(scancode);
                } else {
                    return;
                }
            } else {
                self.pressed_keys.remove(&scancode);
            }
        }
        self.queue.push(event);
    }
    pub fn add_callback(&mut self, callback: EventCallback) {
        self.callbacks.push(callback)
    }
    pub fn process_events(&mut self) {
        for event in self.queue.iter().rev() {
            for cb in &self.callbacks {
                let mut cb = cb.callback.lock();
                (cb)(event);
            }
        }
        self.queue.clear();
        self.queue.shrink_to(8);
    }
}
