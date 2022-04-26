use prismal_utils::string::key::KString;

pub trait ComponentKey: Send + Sync {
    fn key(&self) -> KString;
}
