use std::ops::Deref;

use prismal_utils::hash::fast::fast_hash_64;

use super::AssetKey;

#[inline(always)]
pub fn asset_key<S: Deref<Target = str>>(s: S) -> AssetKey {
    let s = s.trim().to_lowercase().replace(&['\\'], "/");
    AssetKey::from(fast_hash_64(s.as_bytes()))
}
