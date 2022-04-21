use educe::Educe;
use serde::{Deserialize, Serialize};

use prismal_ecs_core::prelude::*;
use prismal_math::vector::*;

/// 3D scale ECS component
#[derive(Debug, Component, Educe)]
#[derive(Deserialize, Serialize)]
#[storage(VecStorage)]
#[educe(Deref, DerefMut)]
#[serde(transparent)]
pub struct CptScale(
    /// Scale value
    pub Vec3,
);

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
