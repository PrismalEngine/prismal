use std::collections::HashSet;

use prismal_utils::hash::int::IntHasherBuilder;

use crate::event::base_event::Event;
use crate::input::key_code::KeyCode;

pub struct EventManager {
    pressed_keys: HashSet<u32, IntHasherBuilder>,
    queue: Vec<Event>,
}

impl EventManager {
    pub(crate) fn new() -> Self {
        Self {
            pressed_keys: HashSet::default(),
            queue: Vec::with_capacity(8),
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
    pub fn process_events(&mut self) {
        for event in self.queue.iter().rev() {
            log::info!("{:?}", event);
        }
        self.queue.clear();
        self.queue.shrink_to(8);
    }
}
