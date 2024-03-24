use crate::{
    registry::{ComponentRegistry, ObjectRegistry},
    renderer::Renderer,
    world::World,
};
use core::fmt::Debug;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use std::{fmt, fs};

pub struct ESEngine {
    pub world: World,
    pub component_registry: ComponentRegistry,
    pub object_registry: ObjectRegistry,
    pub renderer: Option<Box<dyn Renderer>>,
}
impl ESEngine {
    pub fn new(renderer: Option<Box<dyn Renderer>>) -> Self {
        Self {
            world: World::new(0),
            component_registry: ComponentRegistry::new(),
            object_registry: ObjectRegistry::new(),
            renderer,
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
            renderer: None,
        }
    }
    pub fn to_file(&self, file_name: &str) -> Option<()> {
        fs::write(file_name, serde_yaml::to_string(self).ok()?).ok()?;
        Some(())
    }
    pub fn load_file(file_name: &str) -> Option<Self> {
        serde_yaml::from_str(&fs::read_to_string(file_name).ok()?).ok()?
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
