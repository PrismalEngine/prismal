use std::hash::Hash;
use std::hash::Hasher;

use prismal_window::prelude::winit;
pub use winit::event::ElementState as WinitInputState;

use serde::{Deserialize, Serialize};

/// Describes the input state of a binary input.
///
/// A binary input can be either *activated* or *not activated*. Examples include:
/// - Keyboard keys
/// - Mouse buttons
/// - Gamepad face buttons
///     - For an XBox controller these are the *X*, *Y*, *A* and *B* buttons
///     - For an PlayStation controller these are the
///       *Square*, *Triangle*, *X* and *Circle* buttons
#[derive(Debug, Ord, PartialOrd, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[serde(from = "bool", into = "bool")]
#[repr(u8)]
pub enum BinaryInputState {
    /// Key or button is *not activated*
    Released = 0,

    /// Key or button is *activated*
    Pressed = 1,
}

impl PartialEq for BinaryInputState {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Hash for BinaryInputState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8((*self) as u8)
    }
}

impl From<bool> for BinaryInputState {
    fn from(b: bool) -> Self {
        if b {
            Self::Pressed
        } else {
            Self::Released
        }
    }
}
impl From<BinaryInputState> for bool {
    fn from(state: BinaryInputState) -> Self {
        state.is_pressed()
    }
}

impl BinaryInputState {
    /// Returns `true` if the input state is [`Pressed`].
    ///
    /// [`Pressed`]: BinaryInputState::Pressed
    #[must_use]
    pub fn is_pressed(&self) -> bool {
        matches!(self, Self::Pressed)
    }

    /// Returns `true` if the input state is [`Released`].
    ///
    /// [`Released`]: BinaryInputState::Released
    #[must_use]
    pub fn is_released(&self) -> bool {
        matches!(self, Self::Released)
    }
}

impl From<WinitInputState> for BinaryInputState {
    fn from(src: WinitInputState) -> Self {
        match src {
            WinitInputState::Pressed => Self::Pressed,
            WinitInputState::Released => Self::Released,
        }
    }
}
