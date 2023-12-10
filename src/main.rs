use ether_space::{
    components::transform::Transform, log::Log, objects::Object, serializer::Serialize,
    world::World,
};

fn main() {
    Log::info("Initializing");
    let mut window = EtherSpaceEngine::new();
    let mut object1 = Object::new();
    object1.add_component(Box::new(Transform::new()));

    let mut object2 = Object::new();
    object2.add_component(Box::new(Transform::new()));

    window.world.add_object(object1);
    window.world.add_object(object2);
    println!("{}", window.world.serialize());
}

struct EtherSpaceEngine {
    pub world: World,
}
impl EtherSpaceEngine {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
}
