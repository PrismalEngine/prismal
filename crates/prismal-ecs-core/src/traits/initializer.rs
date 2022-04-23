use specs::DispatcherBuilder;
use specs::World;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EcsInitializeOrder {
    #[doc(hidden)]
    InternalUseOnlyAssetsEcsInitializer,
    User(i16),
    #[doc(hidden)]
    InternalUseOnlyDefaultEcsInitializer,
}

impl PartialOrd for EcsInitializeOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            EcsInitializeOrder::InternalUseOnlyAssetsEcsInitializer => match other {
                EcsInitializeOrder::InternalUseOnlyAssetsEcsInitializer => {
                    Some(std::cmp::Ordering::Equal)
                }
                EcsInitializeOrder::User(_) => Some(std::cmp::Ordering::Less),
                EcsInitializeOrder::InternalUseOnlyDefaultEcsInitializer => {
                    Some(std::cmp::Ordering::Less)
                }
            },
            EcsInitializeOrder::User(rhs) => match other {
                EcsInitializeOrder::InternalUseOnlyAssetsEcsInitializer => {
                    Some(std::cmp::Ordering::Greater)
                }
                EcsInitializeOrder::User(lhs) => rhs.partial_cmp(lhs),
                EcsInitializeOrder::InternalUseOnlyDefaultEcsInitializer => {
                    Some(std::cmp::Ordering::Less)
                }
            },
            EcsInitializeOrder::InternalUseOnlyDefaultEcsInitializer => match other {
                EcsInitializeOrder::InternalUseOnlyAssetsEcsInitializer => {
                    Some(std::cmp::Ordering::Less)
                }
                EcsInitializeOrder::User(_) => Some(std::cmp::Ordering::Greater),
                EcsInitializeOrder::InternalUseOnlyDefaultEcsInitializer => {
                    Some(std::cmp::Ordering::Equal)
                }
            },
        }
    }
}
impl Eq for EcsInitializeOrder {}
impl Ord for EcsInitializeOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Trait used to initialize the ECS
pub trait EcsInitializer {
    /// Return an enum to use as a key to sort initializers before use.
    fn get_order(&self) -> EcsInitializeOrder {
        EcsInitializeOrder::User(0)
    }

    /// Setup the tick dispatcher using a [`DispatcherBuilder`].
    /// Return the modified builder.
    ///
    /// Used to register systems that will run every frame.
    fn setup_tick_dispatcher<'a, 'b>(
        &self,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> DispatcherBuilder<'a, 'b> {
        builder
    }

    /// Setup the *early* tick dispatcher using a [`DispatcherBuilder`].
    /// Return the modified builder.
    ///
    /// Used to register systems that will run every frame, but
    /// before the normal tick dispatcher.
    fn setup_early_tick_dispatcher<'a, 'b>(
        &self,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> DispatcherBuilder<'a, 'b> {
        builder
    }

    /// Setup the ECS world (insert resources, register components, etc.)
    #[allow(unused_variables)]
    fn setup_world(&self, world: &mut World) {}
}
