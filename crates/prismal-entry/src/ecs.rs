use prismal_app_core::traits::AppEcs;
use prismal_ecs::prelude::*;

/// Return a new ECS dispatcher for ticking every frame
pub fn create_tick_dispatcher<'a, 'b, A: AppEcs>() -> Dispatcher<'a, 'b> {
    let initializers = A::ecs_initializers();
    prismal_ecs::init::create_tick_dispatcher(&initializers, &[])
}

/// Return a new ECS world
pub fn create_world<A: AppEcs>() -> World {
    let initializers = A::ecs_initializers();

    prismal_ecs::init::create_world(&initializers, &[])
}
