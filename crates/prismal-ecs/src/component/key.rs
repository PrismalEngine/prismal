use prismal_utils::string::key::KString;

pub trait ComponentKey: Send + Sync {
    fn component_key(&self) -> KString;
}
