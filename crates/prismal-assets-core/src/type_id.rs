use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
#[serde(from = "u16", into = "u16")]
pub enum AssetTypeId {
    Bytes,
    Group,
    Scene,
    Material,
    Texture,
    StaticMesh,
    SkeletalMesh,
    Other(u16),
}

impl From<u16> for AssetTypeId {
    fn from(i: u16) -> Self {
        match i {
            0 => Self::Bytes,
            1 => Self::Group,
            2 => Self::Scene,
            3 => Self::Material,
            4 => Self::Texture,
            5 => Self::StaticMesh,
            6 => Self::SkeletalMesh,
            _ => Self::Other(i),
        }
    }
}

impl From<AssetTypeId> for u16 {
    fn from(id: AssetTypeId) -> Self {
        match id {
            AssetTypeId::Bytes => 0,
            AssetTypeId::Group => 1,
            AssetTypeId::Scene => 2,
            AssetTypeId::Material => 3,
            AssetTypeId::Texture => 4,
            AssetTypeId::StaticMesh => 5,
            AssetTypeId::SkeletalMesh => 6,
            AssetTypeId::Other(i) => i,
        }
    }
}
