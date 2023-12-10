use random_number::random;

use crate::serializer::serializer_vec_nest;

use super::log::Log;
use super::objects::Object;
use super::serializer::{serializer, serializer_invec, SerialItem, Serialize};

pub struct World {
    id: u32,
    objects: Vec<Object>,
    pub gravity: f32,
}
impl World {
    pub fn new() -> Self {
        Self {
            id: random!(),
            objects: Vec::new(),
            gravity: 10.0,
        }
    }
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
    pub fn load_from_file() -> Self {
        Log::critical("Todo");
        Self::new()
    }
}

impl Serialize for World {
    fn serialize(&self) -> String {
        self.serialize_nest(0)
    }
    fn serial_items(&self, indent: u8) -> Vec<SerialItem> {
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str("objects", serializer_vec_nest(&self.objects, indent + 1)),
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
