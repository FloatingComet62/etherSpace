use crate::{
    critical,
    modules::{
        log::Log,
        vector::Vector2,
    },
    registry::Registry,
};
use std::{fmt, sync::{Arc, Mutex}};
use serde::{ser::SerializeStruct, Deserialize, Serialize, de::{self, Visitor, SeqAccess, MapAccess}};

/// # World
/// * `id` - A unique id
/// * `gravity` - Global gravity
#[derive(Default)]
pub struct World {
    pub id: u32,
    pub objects: Vec<u32>,
    pub registry: Option<Arc<Mutex<Registry>>>,
    pub gravity: f32,
}
impl World {
    /// * `id` - ID of the world
    /// * `registry` - The entire registry of etherSpace
    pub fn new(id: u32, registry: Arc<Mutex<Registry>>) -> Self {
        Self {
            id,
            objects: Vec::new(),
            gravity: 9.8,
            registry: Some(registry),
        }
    }
    /// * `id` - ID of the world
    /// registry is initialized to None
    pub fn new_noreg(id: u32) -> Self {
        Self {
            id,
            objects: Vec::new(),
            gravity: 9.8,
            registry: None,
        }
    }
    pub fn new_from_yaml(id: u32, objects: Vec<u32>, gravity: f32) -> Self {
        Self {
            id,
            objects,
            gravity,
            registry: None,
        }
    }
    pub fn add_registry(&mut self, registry: Registry) -> Option<()> {
        if self.registry.is_some() {
            return None;
        }
        self.registry = Some(Arc::new(Mutex::new(registry)));
        Some(())
    }
    pub fn create_object(&mut self) -> Option<u32> {
        // PROBABLY NOT CLONE HERE
        let binding = self.registry.clone()?;
        let mut registry = binding.lock().ok()?;
        let id = registry.create_object(Arc::clone(&binding));
        self.objects.push(id);

        let comp_id = registry.create_transform(Vector2::default());
        registry.add_component(id, comp_id);
        Some(id)
    }
    pub fn load_from_file() -> Self {
        critical!("Todo");
    }
}
impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
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
            D: serde::Deserializer<'de> {

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Fields { ID, Objects, Gravity }

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
                let id = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let objects = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let gravity = seq.next_element()?
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
