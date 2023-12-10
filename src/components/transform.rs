use random_number::random;

use crate::serializer::{serializer, serializer_invec};
use crate::{
    objects::Object,
    serializer::{SerialItem, Serialize},
};

use super::Component;

pub struct Transform {
    pub id: u8,
}
impl Transform {
    pub fn new() -> Self {
        Self { id: random!() }
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
        [SerialItem::new_str("id", self.id.to_string())].to_vec()
    }
    fn serialize_nest(&self, indent: u8) -> String {
        serializer(self.serial_items(indent), indent)
    }
    fn serialize_invec(&self, indent: u8) -> String {
        serializer_invec(self.serial_items(indent), indent)
    }
}
