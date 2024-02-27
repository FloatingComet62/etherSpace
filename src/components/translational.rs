use crate::{
    modules::vector::Vector2,
    objects::Object,
};
use serde::{Serialize, Deserialize};
use super::ComponentSignature;

/// # Translational
/// * `id` - A unique ID
/// * `velocity` - Velocity of the object
/// * `requires` - Components which the component requires
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Translational {
    pub id: u32,
    pub velocity: Vector2<f64>,
    requires: Vec<ComponentSignature>,
}
impl Translational {
    pub fn new(id: u32, velocity: Vector2<f64>) -> Self {
        Self {
            id,
            velocity,
            requires: vec![ComponentSignature::Transform],
        }
    }
    pub fn get_requirements(&self) -> Vec<ComponentSignature> {
        self.requires.clone()
    }
    pub fn start(&mut self, _object: &mut Object) {}
    pub fn update(&mut self, _object: &mut Object) {}
}
