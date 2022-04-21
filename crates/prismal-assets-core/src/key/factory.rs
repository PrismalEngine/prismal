use std::ops::Deref;

use prismal_utils::hash::fast::fast_hash_with_seed_64;

use super::AssetKey;

pub trait AssetKeyFactoryExt {
    fn asset_key(&self) -> AssetKey;
}

impl<T: Deref<Target = str>> AssetKeyFactoryExt for T {
    #[inline(always)]
    fn asset_key(&self) -> AssetKey {
        let s = self.trim().to_lowercase().replace(&['\\'], "/");
        let s = s
            .split('/')
            .filter_map(|x| (!x.is_empty()).then(|| x))
            .fold(String::new(), |acc, x| acc + "/" + x);
        let s = s.strip_suffix(".asset").unwrap_or(&s);
        let seed = s.len() as u64;
        AssetKey::from(fast_hash_with_seed_64(s.as_bytes(), seed))
    }
}
