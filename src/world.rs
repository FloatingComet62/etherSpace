use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use std::fmt;

use crate::{
    components::ComponentSignature,
    log,
    modules::vector::Vector2,
    registry::{ComponentRegistry, ObjectRegistry},
};

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
    pub fn add_object(
        &mut self,
        object_id: usize,
        object_registry: &mut ObjectRegistry,
        component_registry: &mut ComponentRegistry,
    ) {
        let obj = &object_registry.0[object_id];
        if let None = obj.get_component(ComponentSignature::Transform, &component_registry) {
            let transform = component_registry.create_transform(Vector2::default());
            log!(
                warn crate "Object({}) is missing a transform, a default Transform({}) was created",
                object_id,
                transform
            );
            object_registry.add_component(object_id, transform);
        }
        self.objects.push(object_id)
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
