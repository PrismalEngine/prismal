pub trait AppFactory {
    fn make_app() -> Box<Self>;
}
