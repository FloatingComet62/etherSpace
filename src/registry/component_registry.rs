use crate::{
    components::{transform::Transform, translational::Translational, Component},
    modules::vector::Vector2,
};

#[derive(Clone)]
pub struct ComponentRegistry {
    component: Vec<Component>,
}
impl ComponentRegistry {
    pub fn new() -> Self {
        Self { component: vec![] }
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
}
