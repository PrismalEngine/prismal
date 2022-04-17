use prismal_app_core::traits::AppCore;
use prismal_gfx::prelude::*;
use prismal_utils::interior_mut::InteriorMut;
use prismal_utils::shared::UnsyncRcMut;
use prismal_window::prelude::*;

pub fn handle_event<A: AppCore + 'static>(
    app: UnsyncRcMut<A>,
    event: WinitEvent<()>,
    flow: &mut ControlFlow,
) {
    let app = app.borrow_int_mut().unwrap();

    fn resize_gfx<A: AppCore + 'static>(
        app: impl std::ops::Deref<Target = A>,
        width: u32,
        height: u32,
    ) {
        let gfx = app.resources().get::<GfxState>();
        if let Some(gfx) = gfx {
            gfx.resize(width, height);
        }
    }
    fn resize_gfx_current<A: AppCore + 'static>(app: impl std::ops::Deref<Target = A>) {
        let gfx = app.resources().get::<GfxState>();
        if let Some(gfx) = gfx {
            let config = gfx.surface_config.borrow_int_mut().unwrap();
            gfx.resize(config.width, config.height);
        }
    }
    fn resize_window<A: AppCore + 'static>(
        app: impl std::ops::Deref<Target = A>,
        new_inner_size: WinitPhysicalSize<u32>,
    ) {
        resize_gfx(app, new_inner_size.width, new_inner_size.height);
    }

    match event {
        WinitEvent::WindowEvent { event, .. } => match event {
            WinitWindowEvent::CloseRequested => {
                *flow = ControlFlow::Exit;
            }
            WinitWindowEvent::Resized(new_inner_size) => {
                resize_window(app, new_inner_size);
            }
            WinitWindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                resize_window(app, *new_inner_size);
            }
            _ => {}
        },
        WinitEvent::MainEventsCleared => {
            let gfx = app.resources().get::<GfxState>().unwrap();
            if let Err(err) = gfx.render() {
                match err {
                    SurfaceError::Outdated | SurfaceError::Lost => resize_gfx_current(app),
                    SurfaceError::OutOfMemory => {
                        log::error!("GPU out of memory!");
                        panic!("{:?}", err);
                    }
                    _ => {
                        log::error!("Surface Error: {:?}", err);
                        panic!("{:?}", err);
                    }
                }
            }
        }
        _ => {}
    }
}
