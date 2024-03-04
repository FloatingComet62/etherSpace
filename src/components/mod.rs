use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

pub mod transform;
pub mod translational;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Component {
    Transform(transform::Transform),
    Translational(translational::Translational),
}
impl Component {
    pub fn get_id(&self) -> usize {
        match self {
            Component::Transform(component) => component.id,
            Component::Translational(component) => component.id,
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
