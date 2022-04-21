use crate::id::AssetId;
use crate::type_id::AssetTypeId;

use downcast::AnySync;

pub trait LoadedAsset: AnySync {
    fn asset_id(&self) -> AssetId;
    fn asset_type_id(&self) -> AssetTypeId;
}
downcast::downcast_sync!(dyn LoadedAsset);
