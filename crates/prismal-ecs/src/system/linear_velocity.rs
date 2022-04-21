use std::ops::{Deref, DerefMut};

use crate::component::linear_velocity::CptLinearVelocity;
use crate::component::position::CptPosition;

use prismal_ecs_core::prelude::*;

#[derive(SystemData)]
pub struct SysLinearVelocityData<'a> {
    positions: WriteStorage<'a, CptPosition>,
    velocities: ReadStorage<'a, CptLinearVelocity>,
}

pub struct SysLinearVelocity;

impl SysLinearVelocity {
    pub const NAME: &'static str = "prismal_sys_linear_velocity";
}

impl<'a> System<'a> for SysLinearVelocity {
    type SystemData = SysLinearVelocityData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        // TODO: Use actual frame delta time
        let frame_delta: f32 = 0.01667;
        for (pos, vel) in (&mut data.positions, &data.velocities).join() {
            *pos.deref_mut() += (*vel.deref()) * frame_delta;
        }
    }
}
