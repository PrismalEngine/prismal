use crate::id::AssetId;
use downcast::AnySync;

pub trait LoadedAsset: AnySync {
    fn id(&self) -> AssetId;
}
downcast::downcast_sync!(dyn LoadedAsset);
