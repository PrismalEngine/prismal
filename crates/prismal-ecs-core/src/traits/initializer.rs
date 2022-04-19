use specs::DispatcherBuilder;
use specs::World;

pub trait EcsInitializer {
    fn setup_tick_dispatcher<'a, 'b>(
        &self,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> DispatcherBuilder<'a, 'b>;
    fn setup_world(&self, world: &mut World);
}
