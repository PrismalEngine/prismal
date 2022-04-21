use std::ops::{Deref, DerefMut};

use crate::component::linear_acceleration::CptLinearAcceleration;
use crate::component::linear_velocity::CptLinearVelocity;
use crate::resource::time::Time;

use prismal_ecs_core::prelude::*;

#[derive(SystemData)]
pub struct SysLinearAccelerationData<'a> {
    time: Read<'a, Time>,
    velocities: WriteStorage<'a, CptLinearVelocity>,
    accelerations: ReadStorage<'a, CptLinearAcceleration>,
}

pub struct SysLinearAcceleration;

impl SysLinearAcceleration {
    pub const NAME: &'static str = "prismal_sys_linear_acceleration";
}

impl<'a> System<'a> for SysLinearAcceleration {
    type SystemData = SysLinearAccelerationData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        let frame_delta = data.time.frame_delta().as_secs_f32();
        for (vel, accel) in (&mut data.velocities, &data.accelerations).join() {
            *vel.deref_mut() += (*accel.deref()) * frame_delta;
        }
    }
}
