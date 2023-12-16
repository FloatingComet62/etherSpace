use modules::serializer::{serializer, serializer_invec, SerialItem, Serialize};

pub mod components;
pub mod errors;
pub mod modules;
pub mod objects;
pub mod registry;
pub mod shapes;
pub mod world;

impl Serialize for u32 {
    fn serialize(&self) -> String {
        self.serialize_nest(0)
    }

    fn serial_items(&self, _indent: u8) -> Vec<SerialItem> {
        [SerialItem::new_str("value", self.to_string())].to_vec()
    }

    fn serialize_nest(&self, indent: u8) -> String {
        serializer(self.serial_items(indent), indent)
    }

    fn serialize_invec(&self, indent: u8) -> String {
        serializer_invec(self.serial_items(indent), indent)
    }
}
