use super::Registry;
use crate::objects::Object;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ObjectRegistry {
    objects: Vec<Object>,
}
impl ObjectRegistry {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    #[inline]
    pub fn create_object(&mut self, registry: Arc<Mutex<Registry>>) -> u32 {
        let id = self.objects.len() as u32;
        self.objects.push(Object::new(id, registry));
        id
    }
    #[inline]
    pub fn add_component(&mut self, object_id: u32, component_id: u32) {
        self.objects[object_id as usize]
            .get_component_ids_mut()
            .push(component_id);
    }
    #[inline]
    pub fn get_object(&self, object_id: u32) -> &Object {
        &self.objects[object_id as usize]
    }
}
