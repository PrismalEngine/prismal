use prismal_utils::shared::UnsyncRcMut;

pub trait AppFactory {
    fn make_app() -> UnsyncRcMut<Self>;
}
