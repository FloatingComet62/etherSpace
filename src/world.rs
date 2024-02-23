use crate::{
    critical,
    modules::{
        log::Log,
        vector::Vector2,
    },
    registry::Registry,
};
use std::sync::{Arc, Mutex};

/// # World
/// * `id` - A unique id
/// * `gravity` - Global gravity
pub struct World {
    pub id: u32,
    pub objects: Vec<u32>,
    pub registry: Arc<Mutex<Registry>>,
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
