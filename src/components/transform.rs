use super::ComponentSignature;
use crate::{
    modules::{
        serializer::{SerialItem, Serialize},
        vector::Vector2,
    },
    objects::Object,
};

/// # Transform
/// * `id` - A unique ID
/// * `position` - Position of the object
/// * `requires` - Components which the component requires
#[derive(Clone)]
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
impl Serialize for Transform {
    fn serial_items(&self, _indent: u8) -> Vec<SerialItem> {
        let vec_printer = |vec: &Vector2<f64>| format!("Vector2({}, {})", vec.x, vec.y);
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str("position", vec_printer(&self.position)),
        ]
        .to_vec()
    }
}
