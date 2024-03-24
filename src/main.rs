use ether_space::{
    components::{Component, ComponentSignature},
    engine::ESEngine,
    log,
    modules::vector::Vector2,
    renderer::{Renderer, SDLRenderer},
};

fn main() {
    log!(info "Initializing");

    let mut engine = ESEngine::load_file("data.txt").unwrap_or_else(|| {
        log!(warn "Failed to load from file");
        let mut engine = ESEngine::new(None);
        let obj1 = engine.object_registry.create_object();
        let translational = engine
            .component_registry
            .create_translational(Vector2::default());
        engine.object_registry.add_component(obj1, translational);

        let obj2 = engine.object_registry.create_object();

        engine.world.add_object(
            obj1,
            &mut engine.object_registry,
            &mut engine.component_registry,
        );
        engine.world.add_object(
            obj2,
            &mut engine.object_registry,
            &mut engine.component_registry,
        );
        engine
    });
    engine.renderer = Some(Box::new(SDLRenderer::new()));

    engine.start();

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
        engine.frame += 1;
        engine.world.objects.clone().iter().for_each(|id| {
            let object = &engine.object_registry.0[*id];
            if let Some(Component::Transform(transform)) =
                object.get_component(ComponentSignature::Transform, &engine.component_registry)
            {
                renderer.draw_rectangle(
                    transform.position.x as u32,
                    transform.position.y as u32,
                    10,
                    10,
                );
            }
        });
        if renderer.end_loop() {
            break;
        }
    }

    engine.to_file("data.txt");
    log!(info "Exiting");
}
