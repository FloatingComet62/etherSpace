use ether_space::{add, create, engine::ESEngine, log, start, update};

fn main() {
    log!(info "Initializing");

    let mut engine = ESEngine::new();
    let obj_id = create!(object engine.registry);
    add!(object to world engine, obj_id);

    let obj_id = create!(object engine.registry);
    let comp_id = create!(translational engine.registry);
    add!(component to object engine.registry, obj_id, comp_id);
    add!(object to world engine, obj_id);

    start!(objects engine.registry, engine.world.objects);
    update!(objects engine.registry, engine.world);

    log!(info object engine);
    log!(info "Exiting");
}
