#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum AssetTypeId {
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
            0 => Self::Group,
            1 => Self::Scene,
            2 => Self::Material,
            3 => Self::Texture,
            4 => Self::StaticMesh,
            5 => Self::SkeletalMesh,
            _ => Self::Other(i),
        }
    }
}

impl From<AssetTypeId> for u16 {
    fn from(id: AssetTypeId) -> Self {
        match id {
            AssetTypeId::Group => 0,
            AssetTypeId::Scene => 1,
            AssetTypeId::Material => 2,
            AssetTypeId::Texture => 3,
            AssetTypeId::StaticMesh => 4,
            AssetTypeId::SkeletalMesh => 5,
            AssetTypeId::Other(i) => i,
        }
    }
}
