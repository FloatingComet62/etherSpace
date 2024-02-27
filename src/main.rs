use ether_space::{
    info,
    modules::log::Log,
    registry::{Registry, Wrapper},
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
    println!("{}", serde_yaml::to_string(&window.world).unwrap());
    let binding = window.world.registry.unwrap();
    let reg = Wrapper(binding.lock().unwrap());
    println!("{}", serde_yaml::to_string(&reg).unwrap());

    info!("Exiting");
}

#[allow(dead_code)]
struct EtherSpaceEngine {
    pub world: World,
}
#[allow(dead_code)]
impl EtherSpaceEngine {
    pub fn new(registry: Arc<Mutex<Registry>>) -> Self {
        Self {
            world: World::new(0, registry),
        }
    }
}
