use super::ComponentSignature;
use crate::modules::vector::Vector2;
use serde::{Deserialize, Serialize};

/// # Translational
/// * `id` - A unique ID
/// * `velocity` - Velocity of the object
/// * `requires` - Components which the component requires
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Translational {
    pub id: usize,
    pub velocity: Vector2,
    requires: Vec<ComponentSignature>,
}
impl Translational {
    pub fn new(id: usize, velocity: Vector2) -> Self {
        Self {
            id,
            velocity,
            requires: vec![],
        }
    }
    pub fn get_requirements(&self) -> Vec<ComponentSignature> {
        self.requires.clone()
    }
}
