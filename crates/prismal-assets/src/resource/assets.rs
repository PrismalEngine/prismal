use std::collections::HashMap;

use prismal_assets_core::prelude::*;
use prismal_utils::hash::int::IntHasherBuilder;
use prismal_utils::shared::*;

use crate::loaded::bytes::LoadedBytesAsset;

pub struct Assets {
    loaded: HashMap<AssetKey, SyncRc<dyn LoadedAsset>, IntHasherBuilder>,
}

impl Assets {
    pub(crate) fn new() -> Self {
        Self {
            loaded: HashMap::default(),
        }
    }
    pub async fn load_bytes(&mut self, path: String) {
        let key = path.asset_key();
        if let std::collections::hash_map::Entry::Vacant(e) = self.loaded.entry(key) {
            let bytes = read_asset_file_bytes(path).await.unwrap();
            e.insert(SyncRc::new(LoadedBytesAsset { key, bytes }));
        }
    }
    pub fn get_loaded<T: LoadedAsset>(&self, key: AssetKey) -> Option<SyncRc<T>> {
        if let Some(l) = self.loaded.get(&key) {
            l.clone().downcast_arc().ok()
        } else {
            None
        }
    }
    pub fn unload(&mut self, key: AssetKey) {
        self.loaded.remove(&key);
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}
