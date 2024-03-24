use crate::{
    components::{Component, ComponentSignature},
    log,
    registry::ComponentRegistry,
};
use serde::{Deserialize, Serialize};

/// # Object
/// * `id` - A unique ID
/// * `components` - A vector of ID of components
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Object {
    pub id: usize,
    pub components: Vec<usize>,
}
impl Object {
    /// * `id` - ID of the object
    /// * `registry` - The entire registry of etherSpace
    pub fn new(id: usize) -> Self {
        Self {
            id,
            components: Vec::new(),
        }
    }
    pub fn new_from_yaml(id: usize, components: Vec<usize>) -> Self {
        Self { id, components }
    }
    /// * `signature` - Signature of the component to find
    pub fn get_component<'a>(
        &self,
        signature: ComponentSignature,
        registry: &'a ComponentRegistry,
    ) -> Option<&'a Component> {
        for component_id in self.components.iter() {
            let component = &registry.0[*component_id as usize];
            if component.signature() == signature {
                return Some(component);
            }
        }
        None
    }
    /// * `signature` - Signature of the component to find
    pub fn get_component_mut<'a>(
        &self,
        signature: ComponentSignature,
        registry: &'a mut ComponentRegistry,
    ) -> Option<&'a mut Component> {
        for component_id in self.components.iter() {
            let component = &registry.0[*component_id as usize];
            if component.signature() == signature {
                return Some(&mut registry.0[*component_id as usize]);
            }
        }
        None
    }
    /// Add a component to the object
    /// * `component_id` - Component ID of the component to add
    pub fn add_component(&mut self, component_id: usize, registry: &mut ComponentRegistry) {
        let component = &registry.0[component_id as usize];
        if self
            .get_component(component.signature(), registry)
            .is_some()
        {
            log!(err crate
                "Cannot add the same component twice ({:?}) to object ({:})",
                component.signature(),
                self.id
            );
        }
        self.components.push(component_id);
    }
}
