use itertools::Itertools;

use prismal_ecs_core::prelude::{Dispatcher, DispatcherBuilder, World};
use prismal_ecs_core::{
    prelude::WorldExt,
    traits::{EcsInitializeOrder, EcsInitializer},
};

use crate::resource::time::Time;

use crate::component::position::CptPosition;
use crate::component::rotation::CptRotation;
use crate::component::scale::CptScale;

use crate::component::linear_acceleration::CptLinearAcceleration;
use crate::component::linear_velocity::CptLinearVelocity;

use crate::system::linear_acceleration::SysLinearAcceleration;
use crate::system::linear_velocity::SysLinearVelocity;
use crate::system::update_time::SysUpdateTime;

pub(crate) struct DefaultEcsInitializer;

impl EcsInitializer for DefaultEcsInitializer {
    fn setup_tick_dispatcher<'a, 'b>(
        &self,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> DispatcherBuilder<'a, 'b> {
        builder
            .with(SysLinearAcceleration, SysLinearAcceleration::NAME, &[])
            .with(
                SysLinearVelocity,
                SysLinearVelocity::NAME,
                &[SysLinearAcceleration::NAME],
            )
            .with_thread_local(SysUpdateTime)
    }

    fn setup_world(&self, world: &mut prismal_ecs_core::prelude::World) {
        world.insert(Time::new());

        world.register::<CptPosition>();
        world.register::<CptRotation>();
        world.register::<CptScale>();
        world.register::<CptLinearAcceleration>();
        world.register::<CptLinearVelocity>();
    }

    fn get_order(&self) -> EcsInitializeOrder {
        EcsInitializeOrder::InternalUseOnlyDefaultEcsInitializer
    }
}

fn default_initializers() -> Vec<Box<dyn EcsInitializer>> {
    vec![Box::new(DefaultEcsInitializer) as Box<dyn EcsInitializer>]
}

#[doc(hidden)]
pub fn create_tick_dispatcher<'a, 'b>(
    initializers: &[Box<dyn EcsInitializer>],
) -> Dispatcher<'a, 'b> {
    let mut builder = DispatcherBuilder::new();
    let default_initializers = default_initializers();
    let initializers = initializers
        .iter()
        .chain(default_initializers.iter())
        .sorted_by(|a, b| a.get_order().cmp(&b.get_order()));
    for i in initializers {
        builder = i.setup_tick_dispatcher(builder);
    }
    builder.build()
}

#[doc(hidden)]
pub fn create_world(initializers: &[Box<dyn EcsInitializer>]) -> World {
    let mut world = World::new();
    let default_initializers = default_initializers();
    let initializers = initializers
        .iter()
        .chain(default_initializers.iter())
        .sorted_by(|a, b| a.get_order().cmp(&b.get_order()));
    for i in initializers {
        i.setup_world(&mut world);
    }
    world
}
