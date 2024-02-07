use modules::serializer::{SerialItem, Serialize};

pub mod components;
pub mod errors;
pub mod modules;
pub mod objects;
pub mod registry;
pub mod shapes;
pub mod world;

impl Serialize for u32 {
    fn serial_items(&self, _indent: u8) -> Vec<SerialItem> {
        [SerialItem::new_str("value", self.to_string())].to_vec()
    }
}
