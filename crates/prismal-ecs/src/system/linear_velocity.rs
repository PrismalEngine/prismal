use std::ops::{Deref, DerefMut};

use crate::component::linear_velocity::CptLinearVelocity;
use crate::component::position::CptPosition;
use crate::resource::time::Time;

use prismal_ecs_core::prelude::*;

pub struct SysLinearVelocity;

impl SysLinearVelocity {
    pub const NAME: &'static str = "prismal_sys_linear_velocity";
}

#[derive(SystemData)]
pub struct SysLinearVelocityData<'a> {
    time: Read<'a, Time>,
    positions: WriteStorage<'a, CptPosition>,
    velocities: ReadStorage<'a, CptLinearVelocity>,
}

impl<'a> System<'a> for SysLinearVelocity {
    type SystemData = SysLinearVelocityData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        let frame_delta = data.time.frame_delta().as_secs_f32();
        for (pos, vel) in (&mut data.positions, &data.velocities).join() {
            *pos.deref_mut() += (*vel.deref()) * frame_delta;
        }
    }
}
