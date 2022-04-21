use educe::Educe;
use serde::{Deserialize, Serialize};

use prismal_ecs_core::prelude::*;
use prismal_math::quat::*;

/// 3D rotation ECS component
#[derive(Debug, Component, Educe)]
#[derive(Deserialize, Serialize)]
#[storage(VecStorage)]
#[educe(Deref, DerefMut)]
#[serde(transparent)]
pub struct CptRotation(pub Quat);

impl CptRotation {
    /// [`CptRotation`] corresponding to no rotation
    pub const IDENTITY: Self = CptRotation(Quat::IDENTITY);
}

impl Default for CptRotation {
    fn default() -> Self {
        Self::IDENTITY
    }
}
