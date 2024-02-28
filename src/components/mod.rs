use crate::objects::Object;
use std::fmt::{Debug, Display};
use serde::{Serialize, Deserialize};

pub mod transform;
pub mod translational;

#[derive(Clone, Serialize, Deserialize)]
pub enum Component {
    Transform(transform::Transform),
    Translational(translational::Translational),
}
impl Component {
    pub fn get_id(&self) -> u32 {
        match self {
            Component::Transform(component) => component.id,
            Component::Translational(component) => component.id,
        }
    }
    pub fn start(&mut self, object: &mut Object) {
        match self {
            Component::Transform(component) => component.start(object),
            Component::Translational(component) => component.start(object),
        }
    }
    pub fn update(&mut self, object: &&mut Object) {
        match self {
            Component::Transform(component) => component.update(object),
            Component::Translational(component) => component.update(object),
        }
    }
    pub fn signature(&self) -> ComponentSignature {
        match self {
            Component::Transform(_) => ComponentSignature::Transform,
            Component::Translational(_) => ComponentSignature::TranslationalPhysics,
        }
    }
    pub fn get_requirements(&self) -> Vec<ComponentSignature> {
        match self {
            Component::Transform(component) => component.get_requirements(),
            Component::Translational(component) => component.get_requirements(),
        }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum ComponentSignature {
    Transform,
    TranslationalPhysics,
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
        }
    }
}
