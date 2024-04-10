use crate::{
    components::{transform::Transform, translational::Translational, render_config::RenderConfig, Component},
    modules::vector::Vector2,
    objects::Object,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ComponentRegistry(pub Vec<Component>);
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ObjectRegistry(pub Vec<Object>);

impl ComponentRegistry {
    pub fn new() -> Self {
        Self(vec![])
    }
    #[inline]
    pub fn create_transform(&mut self, position: Vector2) -> usize {
        let id = self.0.len();
        self.0
            .push(Component::Transform(Transform::new(id, position)));
        id
    }
    #[inline]
    pub fn create_translational(&mut self, velocity: Vector2) -> usize {
        let id = self.0.len();
        self.0
            .push(Component::Translational(Translational::new(id, velocity)));
        id
    }
    #[inline]
    pub fn create_renderconfig_rect(&mut self, width: u32, height: u32) -> usize {
        let id = self.0.len();
        self.0.push(Component::RenderConfig(RenderConfig::new_rect(
            id, width, height,
        )));
        id
    }
    #[inline]
    pub fn create_renderconfig_circle(&mut self, radius: u32) -> usize {
        let id = self.0.len();
        self.0
            .push(Component::RenderConfig(RenderConfig::new_circle(
                id, radius,
            )));
        id
    }
}

impl ObjectRegistry {
    pub fn new() -> Self {
        Self(vec![])
    }
    #[inline]
    pub fn create_object(&mut self) -> usize {
        let id = self.0.len() as usize;
        self.0.push(Object::new(id));
        id
    }
    #[inline]
    pub fn add_component(&mut self, object_id: usize, component_id: usize) {
        self.0[object_id as usize].components.push(component_id);
    }
}
