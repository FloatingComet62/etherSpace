use crate::{
    components::{Component, ComponentSignature},
    log,
    registry::ComponentRegistry,
};
use serde::{Deserialize, Serialize};

fn convert_signature_to_index(
    vector: &Vec<&Component>,
    signature: &ComponentSignature,
) -> Option<usize> {
    for (i, item) in vector.iter().enumerate() {
        if item.signature() == *signature {
            return Some(i);
        }
    }
    None
}
fn trace_dependencies(
    root: &Vec<&Component>,
    requirements: &Vec<ComponentSignature>,
    trail: &mut Vec<ComponentSignature>,
) -> Vec<ComponentSignature> {
    let mut new_sort = vec![];
    for requirement in requirements.iter() {
        if trail.contains(requirement) {
            log!(err crate "Don't eat your own tail");
        }
        let required_node =
            &root[convert_signature_to_index(root, requirement).unwrap_or_else(|| {
                log!(err crate "Requirement {:?} are not fulfiled", requirement);
            })];
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
    new_sort
}
pub fn requirement_sort(vector: &mut Vec<&Component>) {
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
        new_vec.push(
            vector[convert_signature_to_index(vector, item).unwrap_or_else(|| {
                log!(err crate "reached unreachable");
            })],
        );
    }
    vector.clear();
    vector.append(&mut new_vec);
}

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
