use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::{events::Events, objects::Object, registry::ComponentRegistry};

pub mod render_config;
pub mod transform;
pub mod translational;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Component {
    Transform(transform::Transform),
    Translational(translational::Translational),
    RenderConfig(render_config::RenderConfig),
}
impl Component {
    pub fn id(&self) -> usize {
        match self {
            Component::Transform(component) => component.id,
            Component::Translational(component) => component.id,
            Component::RenderConfig(component) => component.id,
        }
    }
    pub fn signature(&self) -> ComponentSignature {
        match self {
            Component::Transform(_) => ComponentSignature::Transform,
            Component::Translational(_) => ComponentSignature::TranslationalPhysics,
            Component::RenderConfig(_) => ComponentSignature::RenderConfig,
        }
    }
    pub fn get_requirements(&self) -> Vec<ComponentSignature> {
        match self {
            Component::Transform(component) => component.get_requirements(),
            Component::Translational(component) => component.get_requirements(),
            Component::RenderConfig(component) => component.get_requirements(),
        }
    }
    pub fn start(&mut self, object: &mut Object) {
        match self {
            Component::Transform(component) => component.start(object),
            Component::Translational(component) => component.start(object),
            Component::RenderConfig(component) => component.start(object),
        }
    }
    pub fn update(
        &mut self,
        object: &mut Object,
        component_registry: &ComponentRegistry,
        event: &mut Events,
    ) {
        match self {
            Component::Transform(component) => component.update(object, component_registry, event),
            Component::Translational(component) => {
                component.update(object, component_registry, event)
            }
            Component::RenderConfig(component) => {
                component.update(object, component_registry, event)
            }
        }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum ComponentSignature {
    Transform,
    TranslationalPhysics,
    RenderConfig,
}
impl Debug for ComponentSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self))
    }
}
impl Display for ComponentSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ComponentSignature::Transform => f.write_str("Transform"),
            ComponentSignature::TranslationalPhysics => f.write_str("Translational Physics"),
            ComponentSignature::RenderConfig => f.write_str("Render Config"),
        }
    }
}
