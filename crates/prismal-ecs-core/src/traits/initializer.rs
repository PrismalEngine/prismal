use specs::DispatcherBuilder;
use specs::World;

/// Trait used to initialize the ECS
pub trait EcsInitializer {
    /// Setup the tick dispatcher using a [`DispatcherBuilder`].
    /// Return the modified builder.
    ///
    /// Used to register systems that will run every frame.
    fn setup_tick_dispatcher<'a, 'b>(
        &self,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> DispatcherBuilder<'a, 'b>;

    /// Setup the ECS world (insert resources, register components, etc.)
    fn setup_world(&self, world: &mut World);
}
