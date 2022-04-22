/// Event data for a [`WindowFocus`] event
///
/// [`WindowFocus`]: crate::event::base_event::Event::WindowFocus
#[derive(Debug, Clone)]
pub struct WindowFocusEventData {
    /// `true` if the window gained focus. `false` if the window lost focus.
    pub gained: bool,
}

impl WindowFocusEventData {
    pub fn new(gained: bool) -> Self {
        Self { gained }
    }
}
