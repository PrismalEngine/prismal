use prismal_app_core::traits::AppCore;
use prismal_gfx::prelude::*;
use prismal_window::prelude::*;

pub fn handle_event<A: AppCore + 'static>(
    app: &mut A,
    event: WinitEvent<()>,
    flow: &mut ControlFlow,
) {
    match event {
        WinitEvent::WindowEvent { event, .. } => match event {
            WinitWindowEvent::CloseRequested => {
                *flow = ControlFlow::Exit;
            }
            _ => {}
        },
        WinitEvent::MainEventsCleared => {
            let gfx = app.resources().get::<GfxState>().unwrap();
            match gfx.render() {
                Err(err) => match err {
                    SurfaceError::Timeout => todo!(),
                    SurfaceError::Outdated => todo!(),
                    SurfaceError::Lost => todo!(),
                    SurfaceError::OutOfMemory => todo!(),
                },
                _ => {}
            }
        }
        _ => {}
    }
}
