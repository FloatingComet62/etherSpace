use crate::{
    components::ComponentSignature,
    critical,
    modules::{
        log::Log,
        serializer::{serializer, serializer_invec, serializer_vec_nest, SerialItem, Serialize},
    },
    registry::Registry,
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Object {
    pub id: u32,
    components: Vec<u32>,
    registry: Arc<Mutex<Registry>>,
}
impl Object {
    pub fn new(id: u32, registry: Arc<Mutex<Registry>>) -> Self {
        Self {
            id,
            components: Vec::new(),
            registry,
        }
    }
    pub fn get_component(&self, signature: ComponentSignature) -> Option<u32> {
        let raw_registry = self.registry.lock();
        if raw_registry.is_err() {
            critical!("Registry is locked");
            return None;
        }
        let registry = raw_registry.unwrap();
        for component_id in self.components.iter() {
            let component = registry.get_component(*component_id);
            if component.signature() == signature {
                return Some(component_id.clone());
            }
        }
        None
    }
    pub fn get_component_ids(&self) -> Vec<u32> {
        self.components.clone()
    }
    pub fn add_component(&mut self, component_id: u32) -> Option<()> {
        {
            let raw_registry = self.registry.lock();
            if raw_registry.is_err() {
                critical!("Registry is locked");
                return None;
            }
            let registry = raw_registry.unwrap();
            let component = registry.get_component(component_id);
            if self.get_component(component.signature()).is_some() {
                critical!(
                    "Cannot add the same component twice ({:?}) to object ({:})",
                    component.signature(),
                    self.id
                );
                return None;
            }
        }
        self.components.push(component_id);
        return Some(());
    }
}
impl Serialize for Object {
    fn serialize(&self) -> String {
        self.serialize_nest(0)
    }
    fn serial_items(&self, indent: u8) -> Vec<SerialItem> {
        let component_map: Vec<Object>;
        {
            let raw_registry = self.registry.lock();
            if raw_registry.is_err() {
                critical!("Registry is locked");
                return vec![];
            }
            let registry = raw_registry.unwrap();
            component_map = self
                .components
                .iter()
                .map(|obj_id| (*registry.get_object(*obj_id)).clone())
                .collect();
        }
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str(
                "components",
                serializer_vec_nest(&component_map, indent + 1),
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
