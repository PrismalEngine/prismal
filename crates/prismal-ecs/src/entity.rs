use std::hash::Hash;
use std::hash::Hasher;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialOrd, Eq, Ord)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Entity(u64);

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for Entity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}

impl Entity {
    pub fn from_index(index: u64) -> Self {
        Self(index)
    }
    pub fn index(&self) -> u64 {
        self.0
    }
}
