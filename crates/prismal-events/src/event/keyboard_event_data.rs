use crate::input::input_state::BinaryInputState;
use crate::input::key_code::KeyCode;

use prismal_window::prelude::winit;
pub use winit::event::KeyboardInput as WinitKeyboardInput;

/// Event data for a [`Keyboard`] event
///
/// [`Keyboard`]: crate::event::base_event::Event::Keyboard
#[derive(Debug, Clone)]
pub struct KeyboardEventData {
    /// The new state of the keyboard key.
    ///
    /// May be either [`Pressed`] or [`Released`].
    ///
    /// [`Pressed`]: crate::input::input_state::BinaryInputState::Pressed
    /// [`Released`]: crate::input::input_state::BinaryInputState::Released
    pub state: BinaryInputState,

    /// Identifies the physical key
    pub scancode: u32,

    /// Identifies the semantic meaning of the key
    pub keycode: Option<KeyCode>,
}

impl KeyboardEventData {
    pub fn new(src: WinitKeyboardInput) -> Self {
        Self {
            state: src.state.into(),
            scancode: src.scancode,
            keycode: src.virtual_keycode.map(From::from),
        }
    }
}
