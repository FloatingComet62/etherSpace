use crate::{
    components::{transform::Transform, translational::Translational, Component},
    modules::vector::Vector2,
    objects::Object,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Registry {
    component: Vec<Component>,
    objects: Vec<Object>,
}
impl Registry {
    pub fn new() -> Self {
        Self {
            component: vec![],
            objects: vec![],
        }
    }
    #[inline]
    pub fn create_transform(&mut self, position: Vector2<f64>) -> u32 {
        let id = self.component.len() as u32;
        self.component
            .push(Component::Transform(Transform::new(id, position)));
        id
    }
    #[inline]
    pub fn create_translational(&mut self, velocity: Vector2<f64>) -> u32 {
        let id = self.component.len() as u32;
        self.component
            .push(Component::Translational(Translational::new(id, velocity)));
        id
    }
    #[inline]
    pub fn get_component(&self, component_id: u32) -> &Component {
        &self.component[component_id as usize]
    }
    #[inline]
    pub fn get_component_mut(&mut self, component_id: u32) -> &mut Component {
        &mut self.component[component_id as usize]
    }
    #[inline]
    pub fn create_object(&mut self) -> u32 {
        let id = self.objects.len() as u32;
        self.objects.push(Object::new(id));
        id
    }
    #[inline]
    pub fn add_component(&mut self, object_id: u32, component_id: u32) {
        self.objects[object_id as usize]
            .get_component_ids_mut()
            .push(component_id);
    }
    #[inline]
    pub fn get_object(&self, object_id: u32) -> &Object {
        &self.objects[object_id as usize]
    }
}
