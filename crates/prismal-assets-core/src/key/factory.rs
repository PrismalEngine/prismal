use std::ops::Deref;

use prismal_utils::hash::fast::fast_hash_64;

use super::AssetKey;

pub trait AssetKeyFactoryExt {
    fn asset_key(&self) -> AssetKey;
}

impl<T: Deref<Target = str>> AssetKeyFactoryExt for T {
    #[inline(always)]
    fn asset_key(&self) -> AssetKey {
        let s = self.trim().to_lowercase().replace(&['\\'], "/");
        AssetKey::from(fast_hash_64(s.as_bytes()))
    }
}
