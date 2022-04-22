use prismal_ecs_core::prelude::*;

use crate::manager::EventManager;

pub struct SysUpdateEvents;

impl SysUpdateEvents {
    pub const NAME: &'static str = "sys_update_events";
}

impl<'a> System<'a> for SysUpdateEvents {
    type SystemData = WriteExpect<'a, EventManager>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.process_events()
    }
}
