use prismal_ecs_core::prelude::*;

use crate::resource::time::Time;

#[derive(SystemData)]
pub struct SysUpdateTimeData<'a> {
    time: Write<'a, Time>,
}

pub struct SysUpdateTime;

impl<'a> System<'a> for SysUpdateTime {
    type SystemData = SysUpdateTimeData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut time = data.time;
        time.after_frame();
    }
}
