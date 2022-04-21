use prismal_ecs_core::traits::EcsInitializer;

pub trait AppEcs {
    fn ecs_initializers() -> Vec<Box<dyn EcsInitializer>> {
        Vec::new()
    }
}
