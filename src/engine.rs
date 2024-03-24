use crate::{
    components::{Component, ComponentSignature},
    events::Events,
    log,
    registry::{ComponentRegistry, ObjectRegistry},
    world::World,
};
use core::fmt::Debug;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use std::{fmt, fs};

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

pub struct ESEngine {
    pub world: World,
    pub component_registry: ComponentRegistry,
    pub object_registry: ObjectRegistry,
    pub frame: u32,
}
impl ESEngine {
    pub fn new() -> Self {
        Self {
            world: World::new(0),
            component_registry: ComponentRegistry::new(),
            object_registry: ObjectRegistry::new(),
            frame: 0,
        }
    }
    pub fn from_yaml(
        world: World,
        component_registry: ComponentRegistry,
        object_registry: ObjectRegistry,
    ) -> Self {
        Self {
            world,
            component_registry,
            object_registry,
            frame: 0,
        }
    }
    pub fn to_file(&self, file_name: &str) -> Option<()> {
        fs::write(file_name, serde_yaml::to_string(self).ok()?).ok()?;
        Some(())
    }
    pub fn load_file(file_name: &str) -> Option<Self> {
        serde_yaml::from_str(&fs::read_to_string(file_name).ok()?).ok()?
    }
    pub fn start(&mut self) {
        self.world.objects.iter_mut().for_each(|id| {
            log!(info crate "Initialization of Object({})", id);
            let object = &mut self.object_registry.0[*id];
            let obj_reg_clone = self.component_registry.0.clone();
            let mut binding = object
                .components
                .iter()
                .map(|id| &obj_reg_clone[*id])
                .collect();
            requirement_sort(&mut binding);
            binding
                .iter()
                .enumerate()
                .for_each(|(i, binding_item)| object.components[i] = binding_item.id());
            // don't par_iter this in the future (keeping requirements in check)
            object.components.clone().iter().for_each(|id| {
                let component = &mut self.component_registry.0[*id];
                log!(info crate "Initialization of Component({}:{})", id, component.signature());
                component.start(object);
            });
        });
    }
    pub fn update(&mut self, events: &mut Events) {
        self.world.objects.iter().for_each(|id| {
            let object = &mut self.object_registry.0[*id];
            log!(info crate "[{}] Updating of Object({})", self.frame, id);
            object.components.clone().iter().for_each(|id| {
                let comp_reg_clone = self.component_registry.clone();
                let component = &mut self.component_registry.0[*id];
                log!(info crate
                    "[{}] Updating of Component({}:{})", self.frame, id, component.signature()
                );
                component.update(object, &comp_reg_clone, events);
            });
        });
    }
}
impl Debug for ESEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct StrippedEngine {
            world: World,
            component_registry: ComponentRegistry,
            object_registry: ObjectRegistry,
        }
        let n = StrippedEngine {
            world: self.world.clone(),
            component_registry: self.component_registry.clone(),
            object_registry: self.object_registry.clone(),
        };
        f.write_fmt(format_args!("{:?}", n))
    }
}
impl Serialize for ESEngine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Engine", 3)?;
        let _ = state.serialize_field("world", &self.world);
        let _ = state.serialize_field("component_registry", &self.component_registry);
        let _ = state.serialize_field("object_registry", &self.object_registry);

        state.end()
    }
}
impl<'de> Deserialize<'de> for ESEngine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Fields {
            World,
            #[serde(rename = "component_registry")]
            ComponentRegistry,
            #[serde(rename = "object_registry")]
            ObjectRegistry,
        }

        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = ESEngine;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Engine")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<ESEngine, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let world = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let component_registry = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let object_registry = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(ESEngine::from_yaml(
                    world,
                    component_registry,
                    object_registry,
                ))
            }

            fn visit_map<V>(self, mut map: V) -> Result<ESEngine, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut world = None;
                let mut component_registry = None;
                let mut object_registry = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::World => {
                            if world.is_some() {
                                return Err(de::Error::duplicate_field("world"));
                            }
                            world = Some(map.next_value()?);
                        }
                        Fields::ComponentRegistry => {
                            if component_registry.is_some() {
                                return Err(de::Error::duplicate_field("component_registry"));
                            }
                            component_registry = Some(map.next_value()?);
                        }
                        Fields::ObjectRegistry => {
                            if object_registry.is_some() {
                                return Err(de::Error::duplicate_field("object_registry"));
                            }
                            object_registry = Some(map.next_value()?);
                        }
                    }
                }
                let world = world.ok_or_else(|| de::Error::missing_field("world"))?;
                let component_registry = component_registry
                    .ok_or_else(|| de::Error::missing_field("component_registry"))?;
                let object_registry =
                    object_registry.ok_or_else(|| de::Error::missing_field("object_registry"))?;
                Ok(ESEngine::from_yaml(
                    world,
                    component_registry,
                    object_registry,
                ))
            }
        }

        const FIELDS: &'static [&'static str] = &["world", "component_registry", "object_registry"];
        deserializer.deserialize_struct("Engine", FIELDS, V)
    }
}
