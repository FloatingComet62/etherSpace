use crate::{registry::Registry, renderer::Renderer, world::World};
use core::fmt::Debug;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use std::{fmt, fs};

pub struct ESEngine {
    pub world: World,
    pub registry: Registry,
    pub renderer: Option<Box<dyn Renderer>>,
}
impl ESEngine {
    pub fn new(renderer: Box<dyn Renderer>) -> Self {
        Self {
            world: World::new(0),
            registry: Registry::new(),
            renderer: Some(renderer),
        }
    }
    pub fn from_yaml(world: World, registry: Registry) -> Self {
        Self {
            world,
            registry,
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
            registry: Registry,
        }
        let n = StrippedEngine {
            world: self.world.clone(),
            registry: self.registry.clone(),
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
        let _ = state.serialize_field("registry", &self.registry);
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
            Registry,
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
                let registry = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(ESEngine::from_yaml(world, registry))
            }

            fn visit_map<V>(self, mut map: V) -> Result<ESEngine, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut world = None;
                let mut registry = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::World => {
                            if world.is_some() {
                                return Err(de::Error::duplicate_field("world"));
                            }
                            world = Some(map.next_value()?);
                        }
                        Fields::Registry => {
                            if registry.is_some() {
                                return Err(de::Error::duplicate_field("registry"));
                            }
                            registry = Some(map.next_value()?);
                        }
                    }
                }
                let world = world.ok_or_else(|| de::Error::missing_field("world"))?;
                let registry = registry.ok_or_else(|| de::Error::missing_field("registry"))?;
                Ok(ESEngine::from_yaml(world, registry))
            }
        }

        const FIELDS: &'static [&'static str] = &["world", "registry"];
        deserializer.deserialize_struct("Engine", FIELDS, V)
    }
}
