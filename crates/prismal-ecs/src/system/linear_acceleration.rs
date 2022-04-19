use std::ops::{Deref, DerefMut};

use crate::component::linear_acceleration::CptLinearAcceleration;
use crate::component::linear_velocity::CptLinearVelocity;

use prismal_ecs_core::prelude::*;

#[derive(SystemData)]
pub struct SysLinearAccelerationData<'a> {
    velocities: WriteStorage<'a, CptLinearVelocity>,
    accelerations: ReadStorage<'a, CptLinearAcceleration>,
}

pub struct SysLinearAcceleration;

impl<'a> System<'a> for SysLinearAcceleration {
    type SystemData = SysLinearAccelerationData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        // TODO: Use actual frame delta time
        let frame_delta: f32 = 0.01667;
        for (vel, accel) in (&mut data.velocities, &data.accelerations).join() {
            *vel.deref_mut() += (*accel.deref()) * frame_delta;
        }
    }
}
