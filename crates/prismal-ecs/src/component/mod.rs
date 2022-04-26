pub mod key;
pub mod storage;

use downcast::AnySync;

use prismal_utils::string::key::KString;

pub use key::ComponentKey;
use storage::component_storage::ComponentStorage;

pub trait Component: ComponentKey + Sized + Send + Sync {
    type Storage: ComponentStorage<Stored = Self>;
}

pub trait AnyComponent: AnySync {
    fn key(&self) -> KString;
}
impl<T: ComponentStorage<Stored = Self>, C: Component<Storage = T> + 'static> AnyComponent for C {
    fn key(&self) -> KString {
        self.component_key()
    }
}
downcast::downcast_sync!(dyn AnyComponent);
