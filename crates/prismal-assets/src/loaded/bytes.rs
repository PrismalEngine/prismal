use prismal_assets_core::prelude::*;

#[derive(Debug)]
pub struct LoadedBytesAsset {
    pub key: AssetKey,
    pub bytes: Vec<u8>,
}

impl LoadedAsset for LoadedBytesAsset {
    fn asset_key(&self) -> AssetKey {
        self.key
    }
    fn asset_type_id(&self) -> AssetTypeId {
        AssetTypeId::Bytes
    }
}
