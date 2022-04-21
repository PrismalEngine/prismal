use crate::key::AssetKey;
use crate::type_id::AssetTypeId;

use downcast::AnySync;

pub trait LoadedAsset: AnySync {
    fn asset_key(&self) -> AssetKey;
    fn asset_type_id(&self) -> AssetTypeId;
}
downcast::downcast_sync!(dyn LoadedAsset);
