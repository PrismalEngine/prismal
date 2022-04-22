use prismal_app_core::traits::{AppCore, AppEcs, AppFactory};
use prismal_assets::prelude::Assets;
use prismal_gfx::state::GfxState;
use prismal_platform_init::init::initialize_platform;
use prismal_utils::interior_mut::InteriorMut;
use prismal_window::prelude::*;

use crate::ecs::*;
use crate::event_handler::handle_event;

/// Prismal Engine entry point
#[allow(clippy::await_holding_refcell_ref)]
pub async fn entry<A: AppCore + AppFactory + AppEcs + 'static>() {
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

    let mut world = create_world::<A>();
    let mut tick_dispatcher = create_tick_dispatcher::<A>();
    tick_dispatcher.setup(&mut world);

    {
        let app = app.borrow_int_mut().unwrap();
        let mut assets = world.fetch_mut::<Assets>();
        for p in app.preload_asset_paths() {
            assets.load_bytes(p).await;
        }
    }
    {
        let mut app = app.borrow_int_mut().unwrap();
        app.resources_mut().insert(world);
    }

    {
        let mut app = app.borrow_int_mut().unwrap();
        app.start();
    }

    event_loop.run(move |event, _, flow| {
        handle_event(app.clone(), &mut tick_dispatcher, event, flow);
    });
}
