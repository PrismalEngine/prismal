use educe::Educe;
use serde::{Deserialize, Serialize};

use prismal_ecs_core::prelude::*;
use prismal_math::vector::*;

/// 3D linear acceleration ECS component
#[derive(Debug, Component, Educe)]
#[derive(Deserialize, Serialize)]
#[educe(Deref, DerefMut)]
#[serde(rename_all = "kebab-case")]
pub struct CptLinearAcceleration {
    /// Is the component enabled?
    ///
    /// If `false` the corresponding system ([`SysLinearAcceleration`][SysLinearAcceleration])
    /// will ignore this component.
    ///
    /// [SysLinearAcceleration]: crate::system::linear_acceleration::SysLinearAcceleration
    pub enabled: bool,

    /// Amount to add to the velocity, on each axis, every second.
    /// Measured in meters-per-second.
    #[educe(Deref, DerefMut)]
    pub meters_per_second_sq: Vec3,
}

impl CptLinearAcceleration {
    /// Return a new [`CptLinearAcceleration`] that is enabled and has a
    /// value of `(0, 0, 0)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use prismal_ecs::component::linear_acceleration::CptLinearAcceleration;
    /// use prismal_math::vector::Vec3;
    ///
    /// let accel = CptLinearAcceleration::new();
    ///
    /// assert_eq!(accel.enabled, true);
    /// assert_eq!(accel.meters_per_second_sq, Vec3::ZERO);
    /// ```
    pub fn new() -> Self {
        Self {
            enabled: true,
            meters_per_second_sq: Vec3::ZERO,
        }
    }

    /// Return a new [`CptLinearAcceleration`] that is disabled and has a
    /// value of `(0, 0, 0)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use prismal_ecs::component::linear_acceleration::CptLinearAcceleration;
    /// use prismal_math::vector::Vec3;
    ///
    /// let accel = CptLinearAcceleration::new_disabled();
    ///
    /// assert_eq!(accel.enabled, false);
    /// assert_eq!(accel.meters_per_second_sq, Vec3::ZERO);
    /// ```
    pub fn new_disabled() -> Self {
        Self {
            enabled: false,
            meters_per_second_sq: Vec3::ZERO,
        }
    }
}

impl Default for CptLinearAcceleration {
    fn default() -> Self {
        Self::new()
    }
}
