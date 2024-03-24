use super::{Component, ComponentSignature};
use crate::{modules::vector::Vector2, objects::Object, registry::ComponentRegistry};
use serde::{Deserialize, Serialize};

/// # Transform
/// * `id` - A unique ID
/// * `position` - Position of the object
/// * `requires` - Components which the component requires
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Transform {
    pub id: usize,
    pub position: Vector2,
    requires: Vec<ComponentSignature>,
}
impl Transform {
    pub fn new(id: usize, position: Vector2) -> Self {
        Self {
            id,
            position,
            requires: vec![],
        }
    }
    pub fn get_requirements(&self) -> Vec<ComponentSignature> {
        self.requires.clone()
    }
    pub fn start(&mut self, _object: &mut Object) {}
    pub fn update(&mut self, object: &mut Object, component_registry: &ComponentRegistry) {
        if let Some(Component::Translational(translational)) = object.get_component(
            ComponentSignature::TranslationalPhysics,
            &component_registry,
        ) {
            self.position.x += translational.velocity.x;
            self.position.y += translational.velocity.y;
        }
    }
}
