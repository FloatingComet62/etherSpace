use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use std::fmt;

/// # World
/// * `id` - A unique id
/// * `gravity` - Global gravity
#[derive(Default, Debug, Clone)]
pub struct World {
    pub id: usize,
    pub objects: Vec<usize>,
    pub gravity: f32,
}
impl World {
    /// * `id` - ID of the world
    pub fn new(id: usize) -> Self {
        Self {
            id,
            objects: Vec::new(),
            gravity: 9.8,
        }
    }
    pub fn new_from_yaml(id: usize, objects: Vec<usize>, gravity: f32) -> Self {
        Self {
            id,
            objects,
            gravity,
        }
    }
}
#[macro_export]
macro_rules! create {
    (object $registry: expr) => {
        $registry.create_object()
    };
    (transform $registry: expr) => {
        $registry.create_transform(ether_space::modules::vector::Vector2::default())
    };
    (transform $registry: expr, $position: expr) => {
        $registry.create_transform($position)
    };
    (translational $registry: expr) => {
        $registry.create_translational(ether_space::modules::vector::Vector2::default())
    };
    (translational $registry: expr, $velocity: expr) => {
        $registry.create_translational($velocity)
    };
}
#[macro_export]
macro_rules! add {
    (component to object $registry: expr, $object_id: expr, $component_id: expr) => {
        $registry.add_component($object_id, $component_id)
    };
    (object to world $engine: expr, $object_id: expr) => {{
        let obj = &$engine.registry.objects[$object_id];
        if let None = obj.get_component(
            ether_space::components::ComponentSignature::Transform,
            &$engine.registry
        ) {
            let transform = create!(transform $engine.registry);
            log!(
                warn "Object({}) is missing a transform, a default Transform({}) was created",
                $object_id,
                transform
            );
            add!(component to object $engine.registry, $object_id, transform);
        }
        $engine.world.objects.push($object_id)
    }};
}
#[macro_export]
macro_rules! start {
    (component $component: expr, $object: expr) => {
        match $component.signature() {
            ether_space::components::ComponentSignature::Transform => {}
            ether_space::components::ComponentSignature::TranslationalPhysics => {}
        }
    };
    (components $registry: expr, $object: expr, $component_ids: expr) => {
        // Sort the components vector according to the requirements
        let mut binding = $component_ids
            .iter()
            .map(|id| &$registry.components[*id])
            .collect();
        ether_space::objects::requirement_sort(&mut binding);
        binding
            .iter()
            .enumerate()
            .for_each(|(i, binding_item)| $component_ids[i] = binding_item.get_id());
        // don't par_iter this in the future (keeping requirements in check)
        $component_ids.iter().for_each(|id| {
            let mut component = &mut $registry.components[*id];
            log!(info "Initialization of Component({}:{})", id, component.signature());
            start!(component component, $object);
        });
    };
    (objects $registry: expr, $object_ids: expr) => {
        $object_ids.iter_mut().for_each(|id| {
            log!(info "Initialization of Object({})", id);
            let mut object = &mut $registry.objects[*id];
            start!(components $registry, object, object.components);
        });
    }
}
#[macro_export]
macro_rules! update {
    (component $component: expr, $object: expr, $registry: expr, $frame: expr) => {
        match $component {
            ether_space::components::Component::Transform(component) => {}
            ether_space::components::Component::Translational(component) => {}
        }
    };
    (components $registry: expr, $object: expr, $frame: expr) => {
        $object.components.iter().for_each(|id| {
            let component = &mut $registry.components[*id];
            log!(info "[{}] Updating of Component({}:{})", $frame, id, component.signature());
            update!(component component, $object, $registry, $frame);
        });
    };
    (objects $registry: expr, $world: expr, $frame: expr) => {
        $world.objects.iter().for_each(|id| {
            let object = &mut $registry.objects[*id];
            log!(info "[{}] Updating of Object({})", $frame, id);
            update!(components $registry, object, $frame);
        });
    }
}
impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("World", 3)?;
        let _ = state.serialize_field("id", &self.id);
        let _ = state.serialize_field("objects", &self.objects);
        let _ = state.serialize_field("gravity", &self.gravity);
        state.end()
    }
}
impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Fields {
            ID,
            Objects,
            Gravity,
        }

        struct WorldVisitor;
        impl<'de> Visitor<'de> for WorldVisitor {
            type Value = World;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct World")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<World, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let id = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let objects = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let gravity = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(World::new_from_yaml(id, objects, gravity))
            }

            fn visit_map<V>(self, mut map: V) -> Result<World, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut objects = None;
                let mut gravity = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::ID => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Fields::Objects => {
                            if objects.is_some() {
                                return Err(de::Error::duplicate_field("objects"));
                            }
                            objects = Some(map.next_value()?);
                        }
                        Fields::Gravity => {
                            if gravity.is_some() {
                                return Err(de::Error::duplicate_field("gravity"));
                            }
                            gravity = Some(map.next_value()?);
                        }
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let objects = objects.ok_or_else(|| de::Error::missing_field("objects"))?;
                let gravity = gravity.ok_or_else(|| de::Error::missing_field("gravity"))?;
                Ok(World::new_from_yaml(id, objects, gravity))
            }
        }

        const FIELDS: &'static [&'static str] = &["id", "objects", "gravity"];
        deserializer.deserialize_struct("World", FIELDS, WorldVisitor)
    }
}
