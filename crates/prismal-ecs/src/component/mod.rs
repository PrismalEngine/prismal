pub mod storage;

use downcast::AnySync;

use prismal_utils::string::key::KString;

use storage::component_storage::ComponentStorage;

pub trait Component: Sized + Send + Sync {
    type Storage: ComponentStorage<Self>;
    fn key(&self) -> KString;
}

pub trait AnyComponent: AnySync {
    fn key(&self) -> KString;
}
impl<T: ComponentStorage<Self>, C: Component<Storage = T> + 'static> AnyComponent for C {
    fn key(&self) -> KString {
        self.key()
    }
}
downcast::downcast_sync!(dyn AnyComponent);
