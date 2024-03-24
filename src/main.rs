use ether_space::{
    add, create, engine::ESEngine, log, render, renderer::{Renderer, SDLRenderer}, start, update
};

fn main() {
    log!(info "Initializing");

    let mut engine = ESEngine::load_file("data.txt").unwrap_or_else(|| {
        log!(warn "Failed to load from file");
        let mut engine = ESEngine::new(None);
        let obj1 = create!(object engine.object_registry);
        let translational = create!(translational engine.component_registry);
        add!(component to object engine.object_registry, obj1, translational);

        let obj2 = create!(object engine.object_registry);

        add!(object to world engine, obj1);
        add!(object to world engine, obj2);
        engine
    });
    engine.renderer = Some(Box::new(SDLRenderer::new()));

    start!(objects engine.component_registry, engine.object_registry, engine.world.objects);

    let mut i = 0;
    let renderer: &mut Box<dyn Renderer> = engine
        .renderer
        .as_mut()
        .expect("Failed to access the renderer");
    assert!(renderer.is::<SDLRenderer>());
    let renderer: &mut SDLRenderer = renderer
        .downcast_mut::<SDLRenderer>()
        .expect("Failed to downcast the Renderer");
    loop {
        renderer.start_loop();
        i += 1;
        update!(objects engine.component_registry, engine.object_registry, engine.world, i);
        render!(engine.world.objects, renderer, engine.component_registry, engine.object_registry);
        if renderer.end_loop() {
            break;
        }
    }

    engine.to_file("data.txt");
    log!(info "Exiting");
}
