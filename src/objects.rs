use super::serializer::{serializer, serializer_invec, SerialItem, Serialize};
use crate::{components::Component, serializer::serializer_vec_nest};
use random_number::random;

pub struct Object {
    pub id: u32,
    components: Vec<Box<dyn Component>>,
}
impl Object {
    pub fn new() -> Self {
        Self {
            id: random!(),
            components: Vec::new(),
        }
    }
    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
}

impl Serialize for Object {
    fn serialize(&self) -> String {
        self.serialize_nest(0)
    }
    fn serial_items(&self, indent: u8) -> Vec<SerialItem> {
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str(
                "components",
                serializer_vec_nest(&self.components, indent + 1),
            ),
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
