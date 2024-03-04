use crate::{
    components::{Component, ComponentSignature},
    log,
    registry::Registry,
};
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use std::fmt;

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
            log!(err "Don't eat your own tail");
        }
        let required_node =
            &root[convert_signature_to_index(root, requirement).unwrap_or_else(|| {
                log!(err "reached unreachable");
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
                log!(err "reached unreachable");
            })],
        );
    }
    vector.clear();
    vector.append(&mut new_vec);
}

/// # Object
/// * `id` - A unique ID
/// * `components` - A vector of ID of components
#[derive(Clone, Debug, Default)]
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
    pub fn get_component(&self, signature: ComponentSignature, registry: &Registry) -> Option<Component> {
        for component_id in self.components.iter() {
            let component = &registry.components[*component_id as usize];
            if component.signature() == signature {
                return Some(component.clone());
            }
        }
        None
    }
    /// Add a component to the object
    /// * `component_id` - Component ID of the component to add
    pub fn add_component(&mut self, component_id: usize, registry: &mut Registry) {
        let component = &registry.components[component_id as usize];
        if self
            .get_component(component.signature(), registry)
            .is_some()
        {
            log!(err
                "Cannot add the same component twice ({:?}) to object ({:})",
                component.signature(),
                self.id
            );
        }
        self.components.push(component_id);
    }
}
impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Object", 3)?;
        let _ = state.serialize_field("id", &self.id);
        let _ = state.serialize_field("components", &self.components);
        state.end()
    }
}
impl<'de> Deserialize<'de> for Object {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Fields {
            ID,
            Components,
        }

        struct ObjectVisitor;
        impl<'de> Visitor<'de> for ObjectVisitor {
            type Value = Object;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Object")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Object, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let id = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let components = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Object::new_from_yaml(id, components))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Object, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut components = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::ID => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Fields::Components => {
                            if components.is_some() {
                                return Err(de::Error::duplicate_field("components"));
                            }
                            components = Some(map.next_value()?);
                        }
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let components =
                    components.ok_or_else(|| de::Error::missing_field("components"))?;
                Ok(Object::new_from_yaml(id, components))
            }
        }

        const FIELDS: &'static [&'static str] = &["id", "components"];
        deserializer.deserialize_struct("World", FIELDS, ObjectVisitor)
    }
}
