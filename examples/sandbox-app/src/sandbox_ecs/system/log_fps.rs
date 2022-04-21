use prismal::ecs::prelude::*;

pub struct SysPrintFps;

#[derive(SystemData)]
pub struct SysPrintFpsData<'a> {
    time: ReadExpect<'a, Time>,
}

impl<'a> System<'a> for SysPrintFps {
    type SystemData = SysPrintFpsData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let delta = data.time.frame_delta();
        log::info!("FPS: {}", 1.0 / delta.as_secs_f64());
    }
}
