use super::ComponentSignature;
use crate::{
    modules::vector::Vector2,
    objects::Object,
};
use serde::{Serialize, Deserialize};

/// # Transform
/// * `id` - A unique ID
/// * `position` - Position of the object
/// * `requires` - Components which the component requires
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Transform {
    pub id: u32,
    pub position: Vector2<f64>,
    requires: Vec<ComponentSignature>,
}
impl Transform {
    pub fn new(id: u32, position: Vector2<f64>) -> Self {
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
    pub fn update(&mut self, _object: &mut Object) {}
}
