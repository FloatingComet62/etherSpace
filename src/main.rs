use ether_space::{create, engine::ESEngine, info, modules::log::Log};

fn main() {
    info!("Initializing");

    let mut engine = ESEngine::new();
    create!(object engine);
    create!(object engine);
    info!(object engine);

    info!("Exiting");
}
