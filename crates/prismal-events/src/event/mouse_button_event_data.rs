use crate::input::input_state::*;
use crate::input::mouse_button::*;

/// Event data for a [`Keyboard`] event
///
/// [`Keyboard`]: crate::event::base_event::Event::Keyboard
#[derive(Debug, Clone)]
pub struct MouseButtonEventData {
    /// The new state of the mouse button.
    ///
    /// May be either [`Pressed`] or [`Released`].
    ///
    /// [`Pressed`]: crate::input::input_state::BinaryInputState::Pressed
    /// [`Released`]: crate::input::input_state::BinaryInputState::Released
    pub state: BinaryInputState,

    /// Which mouse button the event is for
    pub button: MouseButton,
}

impl MouseButtonEventData {
    pub fn new(state: WinitInputState, button: WinitMouseButton) -> Self {
        Self {
            state: state.into(),
            button: button.into(),
        }
    }
}
