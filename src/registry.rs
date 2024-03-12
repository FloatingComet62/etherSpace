use crate::{
    components::{transform::Transform, translational::Translational, Component},
    modules::vector::Vector2,
    objects::Object,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Registry {
    pub components: Vec<Component>,
    pub objects: Vec<Object>,
}
impl Registry {
    pub fn new() -> Self {
        Self {
            components: vec![],
            objects: vec![],
        }
    }
    #[inline]
    pub fn create_transform(&mut self, position: Vector2) -> usize {
        let id = self.components.len();
        self.components
            .push(Component::Transform(Transform::new(id, position)));
        id
    }
    #[inline]
    pub fn create_translational(&mut self, velocity: Vector2) -> usize {
        let id = self.components.len();
        self.components
            .push(Component::Translational(Translational::new(id, velocity)));
        id
    }
    #[inline]
    pub fn create_object(&mut self) -> usize {
        let id = self.objects.len() as usize;
        self.objects.push(Object::new(id));
        id
    }
    #[inline]
    pub fn add_component(&mut self, object_id: usize, component_id: usize) {
        self.objects[object_id as usize]
            .components
            .push(component_id);
    }
}
