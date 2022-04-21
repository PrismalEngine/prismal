use prismal_ecs_core::prelude::*;

use crate::resource::Assets;

pub struct AssetsEcsInitializer;
impl EcsInitializer for AssetsEcsInitializer {
    fn get_order(&self) -> EcsInitializeOrder {
        EcsInitializeOrder::InternalUseOnlyAssetsEcsInitializer
    }

    fn setup_tick_dispatcher<'a, 'b>(
        &self,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> DispatcherBuilder<'a, 'b> {
        builder
    }

    fn setup_world(&self, world: &mut World) {
        world.insert(Assets::new());
    }
}
