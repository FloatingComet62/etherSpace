use ether_space::{
    info,
    modules::{log::Log, serializer::Serialize},
    registry::Registry,
    world::World,
};
/// Note that using Mutex introduces potential for contention (threads waiting for the lock) which
/// might affect performance. There are other synchronization primitives in Rust, like RwLock,
/// Atomic* types, and channels (mpsc) that might be suitable depending on the specific use case.
use std::sync::{Arc, Mutex};

#[allow(unreachable_code)]
fn main() {
    info!("Initializing");

    let registry = Arc::new(Mutex::new(Registry::new()));
    let mut window = EtherSpaceEngine::new(registry);
    window.world.create_object();
    window.world.create_object();
    println!("{}", window.world.serialize());

    info!("Exiting");
}

struct EtherSpaceEngine {
    pub world: World,
}
impl EtherSpaceEngine {
    pub fn new(registry: Arc<Mutex<Registry>>) -> Self {
        Self {
            world: World::new(0, registry),
        }
    }
}
