use educe::Educe;

use prismal_ecs_core::prelude::*;
use prismal_math::vector::*;

/// 3D linear velocity ECS component
#[derive(Debug, Component, Educe)]
#[educe(Deref, DerefMut)]
pub struct CptLinearVelocity {
    pub enabled: bool,

    #[educe(Deref, DerefMut)]
    pub meters_per_second: Vec3,
}

impl CptLinearVelocity {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            enabled: true,
            meters_per_second: Vec3::ZERO,
        }
    }
    pub fn new_disabled() -> Self {
        Self {
            enabled: false,
            meters_per_second: Vec3::ZERO,
        }
    }
}
