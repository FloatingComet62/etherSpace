pub mod components;
pub mod engine;
pub mod errors;
pub mod modules;
pub mod objects;
pub mod registry;
pub mod renderer;
pub mod shapes;
pub mod world;

#[macro_export]
macro_rules! create {
    (object $obj_registry: expr) => {
        $obj_registry.create_object()
    };
    (transform $comp_registry: expr) => {
        create!(transform $comp_registry, ether_space::modules::vector::Vector2::default())
    };
    (transform $comp_registry: expr, $position: expr) => {
        $comp_registry.create_transform($position)
    };
    (translational $comp_registry: expr) => {
        create!(translational $comp_registry, ether_space::modules::vector::Vector2::default())
    };
    (translational $comp_registry: expr, $velocity: expr) => {
        $comp_registry.create_translational($velocity)
    };
}
#[macro_export]
macro_rules! add {
    (component to object $obj_registry: expr, $object_id: expr, $component_id: expr) => {
        $obj_registry.add_component($object_id, $component_id)
    };
    (object to world $engine: expr, $object_id: expr) => {{
        let obj = &$engine.object_registry.0[$object_id];
        if let None = obj.get_component(
            ether_space::components::ComponentSignature::Transform,
            &$engine.component_registry
        ) {
            let transform = create!(transform $engine.component_registry);
            log!(
                warn "Object({}) is missing a transform, a default Transform({}) was created",
                $object_id,
                transform
            );
            add!(component to object $engine.object_registry, $object_id, transform);
        }
        $engine.world.objects.push($object_id)
    }};
}
