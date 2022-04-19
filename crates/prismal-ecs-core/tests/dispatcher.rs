use wasm_bindgen_test::*;

use prismal_ecs_core::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Component)]
struct CptInteger32(i32);

#[derive(SystemData)]
struct SysInteger32DoubleData<'a> {
    integer: WriteStorage<'a, CptInteger32>,
}

struct SysInteger32Double;
impl<'a> System<'a> for SysInteger32Double {
    type SystemData = SysInteger32DoubleData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        for integer in (&mut data.integer).join() {
            integer.0 <<= 1;
        }
    }
}

#[test]
#[wasm_bindgen_test]
fn test_dispatcher_double() {
    let mut world = World::new();
    world.register::<CptInteger32>();

    let ent0 = world.create_entity().with(CptInteger32(0)).build();
    let ent1 = world.create_entity().with(CptInteger32(1)).build();
    let ent2 = world.create_entity().with(CptInteger32(2)).build();
    let ent3 = world.create_entity().with(CptInteger32(3)).build();

    DispatcherBuilder::new()
        .with(SysInteger32Double, "sys_integer32_double", &[])
        .build()
        .dispatch(&world);

    let ints = world.read_component::<CptInteger32>();

    let int0 = ints.get(ent0);
    let int1 = ints.get(ent1);
    let int2 = ints.get(ent2);
    let int3 = ints.get(ent3);

    assert!(matches!(int0, Some(CptInteger32(0))));
    assert!(matches!(int1, Some(CptInteger32(2))));
    assert!(matches!(int2, Some(CptInteger32(4))));
    assert!(matches!(int3, Some(CptInteger32(6))));
}
