use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct AssetKey(u64);

impl From<u64> for AssetKey {
    fn from(i: u64) -> Self {
        Self(i)
    }
}

impl PartialEq for AssetKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for AssetKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}
