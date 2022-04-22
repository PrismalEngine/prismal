use prismal_math::vector::Vec2;

use prismal_window::prelude::winit;
pub use winit::event::MouseScrollDelta as WinitMouseScrollDelta;

use serde::{Deserialize, Serialize};

/// Describes a difference in the mouse scroll wheel state.
#[derive(Debug, Copy, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MouseScrollDelta {
    /// Amount in lines or rows to scroll in the horizontal and vertical directions.
    ///
    /// Positive values indicate movement forward (away from the user) or rightwards.
    LineDelta(Vec2),

    /// Amount in pixels to scroll in the horizontal and vertical direction.
    ///
    /// Scroll events are expressed as a PixelDelta if supported by the
    /// device (e.g. a touchpad) and platform.
    PixelDelta(Vec2),
}

impl From<WinitMouseScrollDelta> for MouseScrollDelta {
    fn from(src: WinitMouseScrollDelta) -> Self {
        match src {
            WinitMouseScrollDelta::LineDelta(x, y) => Self::LineDelta(Vec2::new(x, y)),
            WinitMouseScrollDelta::PixelDelta(d) => {
                Self::PixelDelta(Vec2::new(d.x as f32, d.y as f32))
            }
        }
    }
}
