use prismal_math::vector::Vec2;

/// Event data for a [`MouseMotion`] event
///
/// [`MouseMotion`]: crate::event::base_event::Event::MouseMotion
#[derive(Debug, Clone)]
pub struct MouseMotionEventData {
    /// The change in the mouse position.
    pub delta: Vec2,
}

impl MouseMotionEventData {
    pub fn new(delta: (f64, f64)) -> Self {
        Self {
            delta: Vec2::new(delta.0 as f32, delta.1 as f32),
        }
    }
}
