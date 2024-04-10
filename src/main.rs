use ether_space::{
    components::{render_config::Shape, Component, ComponentSignature},
    engine::ESEngine,
    events::Events,
    log,
    modules::vector::Vector2,
    renderer::{Renderer, SDLRenderer},
};

fn main() {
    log!(info "Initializing");

    let mut engine = ESEngine::load_file("data.txt").unwrap_or_else(|| {
        log!(warn "Failed to load from file");
        let mut engine = ESEngine::new();
        {
            let obj1 = engine.object_registry.create_object();

            let translational = engine
                .component_registry
                .create_translational(Vector2::new(1.0, 1.0));
            engine.object_registry.add_component(obj1, translational);

            let rect_renderconfig = engine.component_registry.create_renderconfig_rect(10, 10);
            engine
                .object_registry
                .add_component(obj1, rect_renderconfig);

            engine.world.add_object(
                obj1,
                &mut engine.object_registry,
                &mut engine.component_registry,
            );
        }

        {
            let obj2 = engine.object_registry.create_object();

            let rect_renderconfig = engine.component_registry.create_renderconfig_rect(10, 10);
            engine
                .object_registry
                .add_component(obj2, rect_renderconfig);

            engine.world.add_object(
                obj2,
                &mut engine.object_registry,
                &mut engine.component_registry,
            );
        }
        engine
    });
    let mut renderer = SDLRenderer::new();
    engine.start();

    let mut events = Events(vec![]);

    loop {
        renderer.start_loop();
        engine.frame += 1;
        engine.update(&mut events);
        engine.world.objects.clone().iter().for_each(|id| {
            let object = &engine.object_registry.0[*id];
            if let Some(Component::Transform(transform)) =
                object.get_component(ComponentSignature::Transform, &engine.component_registry)
            {
                if let Some(Component::RenderConfig(render_config)) = object
                    .get_component(ComponentSignature::RenderConfig, &engine.component_registry)
                {
                    match &render_config.shape {
                        Shape::Rectangle(shape) => {
                            renderer.draw_rectangle(
                                transform.position.x as u32,
                                transform.position.y as u32,
                                shape.width,
                                shape.height,
                            );
                        }
                        Shape::Circle(shape) => {
                            renderer.draw_circle(
                                transform.position.x as u32,
                                transform.position.y as u32,
                                shape.radius,
                            );
                        }
                        Shape::None => {}
                    }
                }
            }
        });
        if renderer.end_loop() {
            break;
        }
    }

    // engine.to_file("data.txt");
    log!(info "Exiting");
}
