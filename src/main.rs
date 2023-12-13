use ether_space::{
    components::transform::Transform, log::Log, objects::Object, serializer::Serialize,
    world::World,
};

fn main() {
    Log::info("Initializing");
    let mut window = EtherSpaceEngine::new();
    let mut object1 = Object::new();
    if let Err(_) = object1.add_component(Box::new(Transform::new())) {
        Log::critical("Failed to add component to object1");
        return;
    }

    let mut object2 = Object::new();
    if let Err(_) = object2.add_component(Box::new(Transform::new())) {
        Log::critical("Failed to add component to object2");
        return;
    }

    window.world.add_object(object1);
    window.world.add_object(object2);
    println!("{}", window.world.serialize());

    Log::info("Exiting");
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
