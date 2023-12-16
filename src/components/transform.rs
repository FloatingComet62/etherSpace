use super::ComponentSignature;
use crate::{
    modules::{
        serializer::{serializer, serializer_invec, SerialItem, Serialize},
        vector::Vector2,
    },
    objects::Object,
};

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
            requires: vec![ComponentSignature::Transform],
        }
    }
    pub fn get_requirements(&self) -> Vec<ComponentSignature> {
        self.requires.clone()
    }
    pub fn start(&mut self, _object: &mut Object) {}
    pub fn update(&mut self, _object: &mut Object) {}
}
impl Serialize for Transform {
    fn serialize(&self) -> String {
        self.serialize_nest(0)
    }
    fn serial_items(&self, _indent: u8) -> Vec<SerialItem> {
        let vec_printer = |vec: &Vector2<f64>| format!("Vector2({}, {})", vec.x, vec.y);
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str("position", vec_printer(&self.position)),
        ]
        .to_vec()
    }
    fn serialize_nest(&self, indent: u8) -> String {
        serializer(self.serial_items(indent), indent)
    }
    fn serialize_invec(&self, indent: u8) -> String {
        serializer_invec(self.serial_items(indent), indent)
    }
}
