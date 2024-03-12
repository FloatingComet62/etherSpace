use super::ComponentSignature;
use crate::modules::vector::Vector2;
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
}
