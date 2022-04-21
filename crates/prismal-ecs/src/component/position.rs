use educe::Educe;
use serde::{Deserialize, Serialize};

use prismal_ecs_core::prelude::*;
use prismal_math::vector::*;

/// 3D position ECS component
#[derive(Debug, Component, Educe)]
#[derive(Deserialize, Serialize)]
#[storage(VecStorage)]
#[educe(Deref, DerefMut)]
#[serde(transparent)]
pub struct CptPosition(
    /// Position value in meters
    pub Vec3,
);

impl CptPosition {
    /// [`CptPosition`] with the coordinates, `(0, 0, 0)`
    pub const ORIGIN: Self = CptPosition(Vec3::ZERO);
}

impl Default for CptPosition {
    fn default() -> Self {
        Self::ORIGIN
    }
}
