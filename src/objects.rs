use crate::{
    components::{Component, ComponentSignature},
    critical,
    modules::{
        log::Log,
        serializer::{serializer, serializer_invec, serializer_vec_nest, SerialItem, Serialize},
    },
    registry::Registry,
};
use std::sync::{Arc, Mutex};

fn convert_signature_to_index(
    vector: &Vec<&Component>,
    signature: &ComponentSignature,
) -> Option<usize> {
    for (i, item) in vector.iter().enumerate() {
        if item.signature() == *signature {
            return Some(i);
        }
    }
    return None;
}
fn trace_dependencies(
    root: &Vec<&Component>,
    requirements: &Vec<ComponentSignature>,
    trail: &mut Vec<ComponentSignature>,
) -> Vec<ComponentSignature> {
    let mut new_sort = vec![];
    for requirement in requirements.iter() {
        if trail.contains(requirement) {
            critical!("Don't eat your own tail");
        }
        let required_node = &root[convert_signature_to_index(root, requirement).unwrap()];
        if required_node.get_requirements().len() == 0 {
            if !new_sort.contains(&required_node.signature()) {
                new_sort.push(required_node.signature());
            }
            continue;
        }
        trail.push(required_node.signature());
        for to_add in trace_dependencies(root, &required_node.get_requirements(), trail) {
            if !new_sort.contains(&to_add) {
                new_sort.push(to_add);
            }
        }
        trail.pop();
        if !new_sort.contains(&required_node.signature()) {
            new_sort.push(required_node.signature());
        }
    }
    return new_sort;
}
fn requirement_sort(vector: &mut Vec<&Component>) {
    let mut new_sort = vec![];
    for item in vector.iter() {
        if item.get_requirements().len() == 0 {
            if !new_sort.contains(&item.signature()) {
                new_sort.push(item.signature());
            }
            continue;
        }
        for to_add in trace_dependencies(vector, &item.get_requirements(), &mut vec![]) {
            if !new_sort.contains(&to_add) {
                new_sort.push(to_add);
            }
        }
        if !new_sort.contains(&item.signature()) {
            new_sort.push(item.signature());
        }
    }
    let mut new_vec = vec![];
    for item in new_sort.iter() {
        new_vec.push(vector[convert_signature_to_index(vector, item).unwrap()]);
    }
    vector.clear();
    vector.append(&mut new_vec);
}

/// # Object
/// * `id` - A unique ID
/// * `components` - A vector of ID of components
///
/// An object is an entity in etherSpace
#[derive(Clone)]
pub struct Object {
    pub id: u32,
    components: Vec<u32>,
    registry: Arc<Mutex<Registry>>,
}
impl Object {
    /// Initialize the object
    /// * `id` - ID of the object
    /// * `registry` - The entire registry of etherSpace
    pub fn new(id: u32, registry: Arc<Mutex<Registry>>) -> Self {
        Self {
            id,
            components: Vec::new(),
            registry,
        }
    }
    /// Get a component from the object
    /// * `signature` - Signature of the component to find
    pub fn get_component(&self, signature: ComponentSignature) -> Option<u32> {
        let raw_registry = self.registry.lock();
        if raw_registry.is_err() {
            critical!("Registry is locked");
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
    /// Get the entire vector of components ID
    pub fn get_component_ids(&self) -> &Vec<u32> {
        &self.components
    }
    pub fn get_component_ids_mut(&mut self) -> &mut Vec<u32> {
        &mut self.components
    }
    /// Add a component to the object
    /// * `component_id` - Component ID of the component to add
    pub fn add_component(&mut self, component_id: u32) -> Option<()> {
        {
            let raw_registry = self.registry.lock();
            if raw_registry.is_err() {
                critical!("Registry is locked");
            }
            let registry = raw_registry.unwrap();
            let component = registry.get_component(component_id);
            if self.get_component(component.signature()).is_some() {
                critical!(
                    "Cannot add the same component twice ({:?}) to object ({:})",
                    component.signature(),
                    self.id
                );
            }
        }
        self.components.push(component_id);
        return Some(());
    }
    pub fn start(&mut self) {
        // Sort the components vector according to the requirements
        let raw_registry = self.registry.lock();
        if raw_registry.is_err() {
            critical!("Registry is locked");
        }
        let mut registry = raw_registry.unwrap();
        let mut binding = self
            .components
            .iter()
            .map(|component_id| registry.get_component(*component_id))
            .collect();
        requirement_sort(&mut binding);
        for (i, binding_item) in binding.iter().enumerate() {
            self.components[i] = binding_item.get_id();
        }

        for component_id in self.components.iter() {
            let component = registry.get_component_mut(*component_id);
            component.start(&mut self.clone());
        }
    }
    pub fn update(&mut self) {
        let raw_registry = self.registry.lock();
        if raw_registry.is_err() {
            critical!("Registry is locked");
        }
        let mut registry = raw_registry.unwrap();

        for component_id in self.components.iter() {
            let component = registry.get_component_mut(*component_id);
            component.update(&mut self.clone());
        }
    }
}
impl Serialize for Object {
    fn serialize(&self) -> String {
        self.serialize_nest(0)
    }
    fn serial_items(&self, indent: u8) -> Vec<SerialItem> {
        let component_map: Vec<Component>;
        {
            let raw_registry = self.registry.lock();
            if raw_registry.is_err() {
                critical!("Registry is locked");
            }
            let registry = raw_registry.unwrap();
            component_map = self
                .components
                .iter()
                .map(|comp_id| (*registry.get_component(*comp_id)).clone())
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
