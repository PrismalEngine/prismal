use educe::Educe;

use prismal_ecs_core::prelude::*;
use prismal_math::vector::*;

/// 3D scale ECS component
#[derive(Debug, Component, Educe)]
#[storage(VecStorage)]
#[educe(Deref, DerefMut)]
pub struct CptScale(pub Vec3);

impl CptScale {
    /// [`CptScale`] with the components, `(1, 1, 1)`
    pub const ONE: Self = CptScale(Vec3::ONE);

    /// [`CptScale`] with the components, `(0, 0, 0)`
    pub const ZERO: Self = CptScale(Vec3::ZERO);
}

impl Default for CptScale {
    fn default() -> Self {
        Self::ONE
    }
}
