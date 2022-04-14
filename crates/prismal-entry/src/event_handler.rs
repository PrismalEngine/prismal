use prismal_app_core::traits::AppCore;
use prismal_window::prelude::*;

pub fn handle_event<A: AppCore + 'static>(
    _app: &mut A,
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
        WinitEvent::MainEventsCleared => {}
        _ => {}
    }
}
