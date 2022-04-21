use educe::Educe;
use serde::{Deserialize, Serialize};

use prismal_ecs_core::prelude::*;
use prismal_math::vector::*;

/// 3D linear velocity ECS component
#[derive(Debug, Component, Educe)]
#[derive(Deserialize, Serialize)]
#[educe(Deref, DerefMut)]
#[serde(rename_all = "kebab-case")]
pub struct CptLinearVelocity {
    /// Is the component enabled?
    ///
    /// If `false` the corresponding system ([`SysLinearVelocity`][SysLinearVelocity])
    /// will ignore this component.
    ///
    /// [SysLinearVelocity]: crate::system::linear_velocity::SysLinearVelocity
    pub enabled: bool,

    /// Number of meters to move on each axis every second
    #[educe(Deref, DerefMut)]
    pub meters_per_second: Vec3,
}

impl CptLinearVelocity {
    /// Return a new [`CptLinearVelocity`] that is enabled and
    /// has a value equal to `(0, 0, 0)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use prismal_ecs::component::linear_velocity::CptLinearVelocity;
    /// use prismal_math::vector::Vec3;
    ///
    /// let vel = CptLinearVelocity::new();
    ///
    /// assert_eq!(vel.enabled, true);
    /// assert_eq!(vel.meters_per_second, Vec3::ZERO);
    /// ```
    pub fn new() -> Self {
        Self {
            enabled: true,
            meters_per_second: Vec3::ZERO,
        }
    }

    /// Return a new [`CptLinearVelocity`] that is disabled and
    /// has a value equal to `(0, 0, 0)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use prismal_ecs::component::linear_velocity::CptLinearVelocity;
    /// use prismal_math::vector::Vec3;
    ///
    /// let pos = CptLinearVelocity::new_disabled();
    ///
    /// assert_eq!(pos.enabled, false);
    /// assert_eq!(pos.meters_per_second, Vec3::ZERO);
    /// ```
    pub fn new_disabled() -> Self {
        Self {
            enabled: false,
            meters_per_second: Vec3::ZERO,
        }
    }
}

impl Default for CptLinearVelocity {
    fn default() -> Self {
        Self::new()
    }
}
