use super::ComponentSignature;
use crate::{events::Events, objects::Object, registry::ComponentRegistry};
use serde::{Deserialize, Serialize};

/// # Render Config
/// * `id` - A unique ID
/// * `shape` - Shape of the object
/// * `requires` - Components which the component requires
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RenderConfig {
    pub id: usize,
    pub shape: Shape,
    requires: Vec<ComponentSignature>,
}
impl RenderConfig {
    pub fn new_rect(id: usize, width: u32, height: u32) -> Self {
        Self {
            id,
            shape: Shape::Rectangle(Rectangle { width, height }),
            requires: vec![],
        }
    }
    pub fn new_circle(id: usize, radius: u32) -> Self {
        Self {
            id,
            shape: Shape::Circle(Circle { radius }),
            requires: vec![],
        }
    }
    pub fn get_requirements(&self) -> Vec<ComponentSignature> {
        self.requires.clone()
    }
    pub fn start(&mut self, _object: &mut Object) {}
    pub fn update(
        &mut self,
        _object: &mut Object,
        _component_registry: &ComponentRegistry,
        _events: &mut Events,
    ) {
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Circle {
    pub radius: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    None,
}
impl Default for Shape {
    fn default() -> Self {
        Self::None
    }
}
