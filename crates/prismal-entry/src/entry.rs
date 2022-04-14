use prismal_app_core::traits::{AppCore, AppFactory};
use prismal_gfx::state::GfxState;
use prismal_platform_init::init::initialize_platform;
use prismal_window::prelude::*;

use crate::event_handler::handle_event;

pub async fn entry<A: AppCore + AppFactory + 'static>() {
    initialize_platform();
    let mut app = A::make_app();

    let event_loop = EventLoop::new();
    let window = initialize_window(app.as_ref(), &event_loop);
    app.resources_mut().insert_unsync(window);

    let gfx = GfxState::new(app.as_ref()).await;
    app.resources_mut().insert(gfx);

    app.start();

    event_loop.run(move |event, _, flow| {
        handle_event(app.as_mut(), event, flow);
    });
}
