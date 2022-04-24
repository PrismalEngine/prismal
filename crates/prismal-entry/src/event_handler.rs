use prismal_app_core::traits::AppCore;

use prismal_gfx::prelude::*;
use prismal_utils::interior_mut::InteriorMut;
use prismal_utils::shared::UnsyncRcMut;
use prismal_window::prelude::*;

use winit::event::ElementState;
use winit::event::MouseButton;

#[allow(unused_imports)]
use prismal_events::event::base_event::Event;
#[allow(unused_imports)]
use prismal_events::manager::EventManager;

/// Handle a `winit` event
pub fn handle_event<'a, 'b, A: AppCore + 'static>(
    app: UnsyncRcMut<A>,
    event: WinitEvent<()>,
    flow: &mut ControlFlow,
) {
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

    {
        // let event = Event::try_from_winit(&event);
        // if let Ok(event) = event {}
    }
    match &event {
        WinitEvent::WindowEvent { event, .. } => match event {
            WinitWindowEvent::CloseRequested => {
                *flow = ControlFlow::Exit;
            }
            WinitWindowEvent::Resized(new_inner_size) => {
                let app_ref = app.borrow_int_mut().unwrap();
                resize_window(app_ref, *new_inner_size);
            }
            WinitWindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                let app_ref = app.borrow_int_mut().unwrap();
                let window = app_ref.resources().get_unsync::<Window>().unwrap();
                if let Err(err) = window.set_cursor_grab(true) {
                    log::error!("{:?}", err);
                }
            }
            WinitWindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                let app_ref = app.borrow_int_mut().unwrap();
                resize_window(app_ref, **new_inner_size);
            }
            _ => {}
        },
        WinitEvent::MainEventsCleared => {
            let app_ref = app.borrow_int_mut().unwrap();
            let gfx = app_ref.resources().get::<GfxState>().unwrap();
            if let Err(err) = gfx.render() {
                match err {
                    SurfaceError::Outdated | SurfaceError::Lost => resize_gfx_current(app_ref),
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
