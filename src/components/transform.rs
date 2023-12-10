use random_number::random;

use crate::{
    modules::vector::Vector2,
    objects::Object,
    serializer::{serializer, serializer_invec, SerialItem, Serialize},
};

use super::Component;

pub struct Transform {
    pub id: u32,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
}
impl Transform {
    pub fn new() -> Self {
        Self {
            id: random!(),
            position: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
        }
    }
}
impl Component for Transform {
    fn start(&mut self, _object: &Object) {}
    fn update(&mut self, _object: &Object) {}
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
            SerialItem::new_str("velocity", vec_printer(&self.velocity)),
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
