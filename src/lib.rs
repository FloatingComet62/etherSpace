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
#[macro_export]
macro_rules! start {
    (component $component: expr, $object: expr) => {
        match $component {
            ether_space::components::Component::Transform(component) => {}
            ether_space::components::Component::Translational(component) => {}
        }
    };
    (components $comp_registry: expr, $object: expr, $component_ids: expr) => {
        let obj_reg_clone = $comp_registry.0.clone();
        let mut binding = $component_ids
            .iter()
            .map(|id| &obj_reg_clone[*id])
            .collect();
        ether_space::objects::requirement_sort(&mut binding);
        // object.components is updated for future purposes
        binding
            .iter()
            .enumerate()
            .for_each(|(i, binding_item)| $component_ids[i] = binding_item.get_id());
        // don't par_iter this in the future (keeping requirements in check)
        $component_ids.iter().for_each(|id| {
            let mut component = &mut $comp_registry.0[*id];
            log!(info "Initialization of Component({}:{})", id, component.signature());
            start!(component component, $object);
        });
    };
    (objects $comp_registry: expr, $obj_registry: expr, $object_ids: expr) => {
        $object_ids.iter_mut().for_each(|id| {
            log!(info "Initialization of Object({})", id);
            let mut object = &mut $obj_registry.0[*id];
            start!(components $comp_registry, object, object.components);
        });
    }
}
#[macro_export]
macro_rules! update {
    (component $component: expr, $object: expr, $comp_registry: expr, $frame: expr) => {
        match $component {
            ether_space::components::Component::Transform(component) => {
                if let Some(ether_space::components::Component::Translational(translational)) =
                $object.get_component(
                    ether_space::components::ComponentSignature::TranslationalPhysics,
                    &$comp_registry
                ) {
                    component.position.x += translational.velocity.x;
                    component.position.y += translational.velocity.y;
                }
            }
            ether_space::components::Component::Translational(component) => {}
        }
    };
    (components $comp_registry: expr, $object: expr, $frame: expr) => {
        $object.components.iter().for_each(|id| {
            let comp_reg_clone = $comp_registry.clone();
            let component = &mut $comp_registry.0[*id];
            log!(info "[{}] Updating of Component({}:{})", $frame, id, component.signature());
            update!(component component, $object, comp_reg_clone, $frame);
        });
    };
    (objects $comp_registry: expr, $obj_registry: expr, $world: expr, $frame: expr) => {
        $world.objects.iter().for_each(|id| {
            let object = &mut $obj_registry.0[*id];
            log!(info "[{}] Updating of Object({})", $frame, id);
            update!(components $comp_registry, object, $frame);
        });
    }
}
#[macro_export]
macro_rules! render {
    ($objects: expr, $renderer: expr, $comp_registry: expr, $obj_registry: expr) => {
        $objects.iter().for_each(|id| {
            let object = &$obj_registry.0[*id];
            if let Some(ether_space::components::Component::Transform(transform)) =
            object.get_component(
                ether_space::components::ComponentSignature::Transform,
                &$comp_registry
            ) {
                $renderer.draw_rectangle(
                    transform.position.x as u32,
                    transform.position.y as u32,
                    10,
                    10,
                );    
            }
        });
    };
}
