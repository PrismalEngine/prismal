use prismal_app_core::traits::AppEcs;
use prismal_ecs::prelude::*;

fn collect_initializers<A: AppEcs>() -> Vec<Box<dyn EcsInitializer>> {
    let mut initializers = A::ecs_initializers();
    initializers.append(&mut vec![
        Box::new(prismal_assets::init::AssetsEcsInitializer),
        Box::new(prismal_events::init::EventsEcsInitializer),
    ]);
    initializers
}

/// Return a new ECS dispatcher for ticking every frame
pub fn create_tick_dispatcher<'a, 'b, A: AppEcs>() -> Dispatcher<'a, 'b> {
    let initializers = collect_initializers::<A>();
    prismal_ecs::init::create_tick_dispatcher(&initializers)
}

/// Return a new ECS dispatcher for ticking early every frame
pub fn create_early_tick_dispatcher<'a, 'b, A: AppEcs>() -> Dispatcher<'a, 'b> {
    let initializers = collect_initializers::<A>();
    prismal_ecs::init::create_early_tick_dispatcher(&initializers)
}

/// Return a new ECS world
pub fn create_world<A: AppEcs>() -> World {
    let initializers = collect_initializers::<A>();
    prismal_ecs::init::create_world(&initializers)
}
