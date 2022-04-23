use prismal_utils::prelude::once_cell::sync::OnceCell;
use specs::World;

static WORLD: OnceCell<World> = OnceCell::new();

pub fn get_world() -> &'static World {
    WORLD.get().expect("WORLD is not set yet!")
}
pub fn set_world(world: World) {
    match WORLD.set(world) {
        Err(_) => panic!("WORLD was already set!"),
        _ => {}
    }
}
