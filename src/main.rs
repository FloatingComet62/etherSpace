use ether_space::{
    info,
    modules::log::Log,
    registry::Registry,
    world::World,
};
/// Note that using Mutex introduces potential for contention (threads waiting for the lock) which
/// might affect performance. There are other synchronization primitives in Rust, like RwLock,
/// Atomic* types, and channels (mpsc) that might be suitable depending on the specific use case.
/// use std::sync::{Arc, Mutex};

#[allow(unreachable_code)]
fn main() {
    info!("Initializing");

    let mut registry = Registry::new();
    // TODO: FIGURE OUT SHIT HERE
    let window = EtherSpaceEngine::new(&mut registry);
    // window.world.create_object(&mut registry);
    // window.world.create_object(&mut registry);
    println!("{}", serde_yaml::to_string(&window.world).unwrap());
    println!("{}", serde_yaml::to_string(&registry).unwrap());

    info!("Exiting");
}

#[allow(dead_code)]
struct EtherSpaceEngine<'a> {
    pub world: World<'a>,
}
#[allow(dead_code)]
impl<'a> EtherSpaceEngine<'a> {
    pub fn new(registry: &'a mut Registry) -> Self {
        Self {
            world: World::new(0, registry),
        }
    }
}
