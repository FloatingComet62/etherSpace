use crate::{
    critical,
    modules::{
        log::Log,
        serializer::{serializer, serializer_invec, serializer_vec_nest, SerialItem, Serialize},
        vector::Vector2,
    },
    objects::Object,
    registry::Registry,
};
use std::sync::{Arc, Mutex};

/// # World
/// * `id` - A unique id
/// * `gravity` - Global gravity
///
/// The world contains the collection of objects
pub struct World {
    pub id: u32,
    objects: Vec<u32>,
    registry: Arc<Mutex<Registry>>,
    pub gravity: f32,
}
impl World {
    /// Initialize the world
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
    /// Create an object, add it to the registry and return of id if successful
    /// ### Returns
    /// ID of object created
    #[inline]
    pub fn create_object(&mut self) -> Option<u32> {
        let mut registry = self.registry.lock().ok()?;
        let id = registry.create_object(Arc::clone(&self.registry));
        self.objects.push(id);

        let comp_id = registry.create_transform(Vector2::default());
        registry.add_component(id, comp_id);
        Some(id)
    }
    /// Load the world from a file
    pub fn load_from_file() -> Self {
        critical!("Todo");
    }
}

impl Serialize for World {
    fn serialize(&self) -> String {
        self.serialize_nest(0)
    }
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
    fn serialize_nest(&self, indent: u8) -> String {
        serializer(self.serial_items(indent), indent)
    }
    fn serialize_invec(&self, indent: u8) -> String {
        serializer_invec(self.serial_items(indent), indent)
    }
}
