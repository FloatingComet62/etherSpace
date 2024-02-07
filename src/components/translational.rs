use crate::{
    modules::{
        serializer::{SerialItem, Serialize},
        vector::Vector2,
    },
    objects::Object,
};

use super::ComponentSignature;

/// # Translational
/// * `id` - A unique ID
/// * `velocity` - Velocity of the object
/// * `requires` - Components which the component requires
#[derive(Clone)]
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
impl Serialize for Translational {
    fn serial_items(&self, _indent: u8) -> Vec<SerialItem> {
        let vec_printer = |vec: &Vector2<f64>| format!("Vector2({}, {})", vec.x, vec.y);
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str("velocity", vec_printer(&self.velocity)),
        ]
        .to_vec()
    }
}
