use prismal_window::prelude::winit;
pub use winit::event::MouseButton as WinitMouseButton;

use serde::{Deserialize, Serialize};

/// Describes a button on a mouse input device
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MouseButton {
    /// Left or primary mouse button
    Left,
    /// Right or alternate mouse button
    Right,
    /// Middle mouse button
    Middle,
    /// Unknown mouse button
    Other(
        /// ID of the unknown mouse button
        u16,
    ),
}

impl MouseButton {
    /// Returns `true` if the mouse button is [`Left`].
    ///
    /// [`Left`]: MouseButton::Left
    #[must_use]
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left)
    }

    /// Returns `true` if the mouse button is [`Right`].
    ///
    /// [`Right`]: MouseButton::Right
    #[must_use]
    pub fn is_right(&self) -> bool {
        matches!(self, Self::Right)
    }

    /// Returns `true` if the mouse button is [`Middle`].
    ///
    /// [`Middle`]: MouseButton::Middle
    #[must_use]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }

    /// Returns the unknown button id if the mouse button is [`Other`].
    /// Otherwise, returns `None`.
    ///
    /// [`Other`]: MouseButton::Other
    #[must_use]
    pub fn as_other(&self) -> Option<u16> {
        if let Self::Other(id) = self {
            Some(*id)
        } else {
            None
        }
    }
}

impl From<WinitMouseButton> for MouseButton {
    fn from(src: WinitMouseButton) -> Self {
        match src {
            WinitMouseButton::Left => Self::Left,
            WinitMouseButton::Right => Self::Right,
            WinitMouseButton::Middle => Self::Middle,
            WinitMouseButton::Other(i) => Self::Other(i),
        }
    }
}
