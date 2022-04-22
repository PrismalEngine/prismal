use prismal_window::prelude::winit;
use winit::dpi::PhysicalPosition;

use prismal_math::vector::IVec2;

/// Event data for a [`WindowMove`] event
///
/// [`WindowMove`]: crate::event::base_event::Event::WindowMove
#[derive(Debug, Clone)]
pub struct WindowMoveEventData {
    /// The new position of the window
    pub position: IVec2,
}

impl WindowMoveEventData {
    pub fn new(position: PhysicalPosition<i32>) -> Self {
        Self {
            position: IVec2::new(position.x, position.y),
        }
    }
}
