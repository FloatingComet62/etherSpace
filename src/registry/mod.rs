use self::{component_registry::ComponentRegistry, object_registry::ObjectRegistry};
use crate::{components::Component, modules::vector::Vector2, objects::Object};
use std::sync::{Arc, Mutex, MutexGuard};
use serde::{Serialize, Deserialize};

pub mod component_registry;
pub mod object_registry;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Registry {
    component: ComponentRegistry,
    object: ObjectRegistry,
}
impl Registry {
    pub fn new() -> Self {
        Self {
            component: ComponentRegistry::new(),
            object: ObjectRegistry::new(),
        }
    }
    #[inline]
    pub fn create_transform(&mut self, position: Vector2<f64>) -> u32 {
        self.component.create_transform(position)
    }
    #[inline]
    pub fn create_translational(&mut self, velocity: Vector2<f64>) -> u32 {
        self.component.create_translational(velocity)
    }
    #[inline]
    pub fn get_component(&self, component_id: u32) -> &Component {
        self.component.get_component(component_id)
    }
    #[inline]
    pub fn get_component_mut(&mut self, component_id: u32) -> &mut Component {
        self.component.get_component_mut(component_id)
    }
    #[inline]
    pub fn create_object(&mut self, reference: Arc<Mutex<Registry>>) -> u32 {
        self.object.create_object(Arc::clone(&reference))
    }
    #[inline]
    pub fn add_component(&mut self, object_id: u32, component_id: u32) {
        self.object.add_component(object_id, component_id)
    }
    #[inline]
    pub fn get_object(&self, object_id: u32) -> &Object {
        self.object.get_object(object_id)
    }
}

pub struct Wrapper<'a>(pub MutexGuard<'a, Registry>);
impl<'a> Serialize for Wrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.0.serialize(serializer)
    }
}
