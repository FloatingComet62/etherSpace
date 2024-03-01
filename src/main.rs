use ether_space::{create_object, engine::ESEngine, info, modules::log::Log};

fn main() {
    info!("Initializing");

    let mut engine = ESEngine::new();
    let _ = create_object!(engine);
    let _ = create_object!(engine);
    info!(object engine);

    info!("Exiting");
}
