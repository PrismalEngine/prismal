use prismal_ecs_core::prelude::*;
use prismal_ecs_core::traits::EcsInitializer;

use crate::manager::EventManager;
use crate::system::update_events::SysUpdateEvents;

pub struct EventsEcsInitializer;

impl EcsInitializer for EventsEcsInitializer {
    fn setup_world(&self, world: &mut World) {
        world.insert(EventManager::new());
    }
    fn setup_tick_dispatcher<'a, 'b>(
        &self,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> DispatcherBuilder<'a, 'b> {
        builder.with(SysUpdateEvents, SysUpdateEvents::NAME, &[])
    }
}
