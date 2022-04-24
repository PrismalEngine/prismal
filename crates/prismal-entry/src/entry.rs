use prismal_app_core::traits::{AppCore, AppFactory};
// use prismal_assets::prelude::Assets;
use prismal_gfx::state::GfxState;
use prismal_platform_init::init::initialize_platform;
#[allow(unused_imports)]
use prismal_utils::futures::futures::future::join_all;
use prismal_utils::interior_mut::InteriorMut;
use prismal_window::prelude::*;

use crate::event_handler::handle_event;

/// Prismal Engine entry point
#[allow(clippy::await_holding_refcell_ref)]
pub async fn entry<A: AppCore + AppFactory + 'static>() {
    initialize_platform();
    let app = A::make_app();

    let event_loop = EventLoop::new();
    let window = initialize_window(app.clone(), &event_loop);

    {
        let mut app = app.borrow_int_mut().unwrap();
        app.resources_mut().insert_unsync(window);
    }

    {
        let gfx = GfxState::new(app.clone()).await;
        let mut app = app.borrow_int_mut().unwrap();
        app.resources_mut().insert(gfx);
    }

    {
        let mut app = app.borrow_int_mut().unwrap();
        app.start();
    }

    event_loop.run(move |event, _, flow| {
        handle_event(app.clone(), event, flow);
    });
}
