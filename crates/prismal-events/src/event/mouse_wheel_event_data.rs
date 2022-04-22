use crate::input::mouse_scroll_delta::*;

/// Event data for a [`MouseWheel`] event
///
/// [`MouseWheel`]: crate::event::base_event::Event::MouseWheel
#[derive(Debug, Clone)]
pub struct MouseWheelEventData {
    /// The change in the mouse scroll wheel rotation
    pub delta: MouseScrollDelta,
}

impl MouseWheelEventData {
    pub fn new(delta: WinitMouseScrollDelta) -> Self {
        Self {
            delta: delta.into(),
        }
    }
}
