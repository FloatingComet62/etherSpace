use ether_space::{
    info,
    modules::{
        log::Log,
        serializer::Serialize,
        units::{generate_powers_from_unit_str_composite, Unit},
    },
    registry::Registry,
    unit,
    world::World,
};
/// Note that using Mutex introduces potential for contention (threads waiting for the lock) which
/// might affect performance. There are other synchronization primitives in Rust, like RwLock,
/// Atomic* types, and channels (mpsc) that might be suitable depending on the specific use case.
use std::sync::{Arc, Mutex};

fn main() {
    info!("Initializing");
    // let acceleration = unit!(5.0, "m/s^2");
    // let mass = unit!(20.0, "kg");
    // let force = mass * acceleration;
    let electric_field = unit!(25.0, "N/C");
    let charge = unit!(2.0, "C");
    println!("{}", electric_field * charge);

    let mass = unit!(10.0, "kg");
    let velocity = unit!(50.0, "m/s");
    let energy = mass * velocity.clone() * velocity / 2.0;
    println!("{}", energy);
    std::process::exit(0);

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
