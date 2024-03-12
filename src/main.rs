use ether_space::{
    add, create, engine::ESEngine, log, renderer::Renderer, renderer::SDLRenderer, start, update,
};

fn main() {
    log!(info "Initializing");

    let mut engine = ESEngine::load_file("data.txt").expect("Failed to load the engine");
    engine.renderer = Some(Box::new(SDLRenderer::new()));

    start!(objects engine.registry, engine.world.objects);

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
        update!(objects engine.registry, engine.world, i);
        if renderer.end_loop() {
            break;
        }
    }

    log!(info "Exiting");
}
