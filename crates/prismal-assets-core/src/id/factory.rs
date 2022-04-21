use std::ops::Deref;

use prismal_utils::hash::fast::fast_hash_64;

use super::AssetId;

#[inline(always)]
pub fn asset_id<S: Deref<Target = str>>(s: S) -> AssetId {
    let s = s.trim().to_lowercase().replace(&['\\'], "/");
    AssetId::from(fast_hash_64(s.as_bytes()))
}
