use ether_space::{create, add, engine::ESEngine, log};

fn main() {
    log!(info "Initializing");

    let mut engine = ESEngine::new();
    let obj_id = create!(object engine.registry);
    add!(object to world engine, obj_id);
    log!(info object engine);

    log!(info "Exiting");
}
