use crate::{
    critical,
    modules::{
        log::Log,
        serializer::{serializer_vec_nest, SerialItem, Serialize},
        vector::Vector2,
    },
    objects::Object,
    registry::Registry,
};
use std::sync::{Arc, Mutex};

/// # World
/// * `id` - A unique id
/// * `gravity` - Global gravity
pub struct World {
    pub id: u32,
    objects: Vec<u32>,
    registry: Arc<Mutex<Registry>>,
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
            registry,
        }
    }
    #[inline]
    pub fn create_object(&mut self) -> Option<u32> {
        let mut registry = self.registry.lock().ok()?;
        let id = registry.create_object(Arc::clone(&self.registry));
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
    fn serial_items(&self, indent: u8) -> Vec<SerialItem> {
        let object_map: Vec<Object>;
        {
            let raw_registry = self.registry.lock();
            if raw_registry.is_err() {
                critical!("Registry is locked");
            }
            let registry = raw_registry.unwrap();
            object_map = self
                .objects
                .iter()
                .map(|obj_id| (*registry.get_object(*obj_id)).clone())
                .collect();
        }
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str("objects", serializer_vec_nest(&object_map, indent + 1)),
        ]
        .to_vec()
    }
}
