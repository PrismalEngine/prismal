use std::hash::Hash;

#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct AssetId(pub(crate) u64);

impl PartialEq for AssetId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for AssetId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}
