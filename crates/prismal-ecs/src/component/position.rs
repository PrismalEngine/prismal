use educe::Educe;

use prismal_ecs_core::prelude::*;
use prismal_math::vector::*;

/// 3D position ECS component
#[derive(Debug, Component, Educe)]
#[storage(VecStorage)]
#[educe(Deref, DerefMut)]
pub struct CptPosition(pub Vec3);

impl CptPosition {
    /// [`CptPosition`] with the coordinates, `(0, 0, 0)`
    pub const ORIGIN: Self = CptPosition(Vec3::ZERO);
}

impl Default for CptPosition {
    fn default() -> Self {
        Self::ORIGIN
    }
}
