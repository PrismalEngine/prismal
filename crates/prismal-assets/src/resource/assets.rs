use std::collections::HashMap;

use prismal_assets_core::prelude::*;
use prismal_utils::hash::int::IntHasherBuilder;
use prismal_utils::shared::*;

pub struct Assets {
    loaded: HashMap<AssetKey, SyncRc<dyn LoadedAsset>, IntHasherBuilder>,
}

impl Assets {
    pub(crate) fn new() -> Self {
        Self {
            loaded: HashMap::default(),
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
