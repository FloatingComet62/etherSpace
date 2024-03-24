use super::ComponentSignature;
use crate::{events::{Action, Events}, modules::vector::Vector2, objects::Object, registry::ComponentRegistry};
use serde::{Deserialize, Serialize};

/// # Translational
/// * `id` - A unique ID
/// * `velocity` - Velocity of the object
/// * `requires` - Components which the component requires
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Translational {
    pub id: usize,
    pub velocity: Vector2,
    requires: Vec<ComponentSignature>,
}
impl Translational {
    pub fn new(id: usize, velocity: Vector2) -> Self {
        Self {
            id,
            velocity,
            requires: vec![],
        }
    }
    pub fn get_requirements(&self) -> Vec<ComponentSignature> {
        self.requires.clone()
    }
    pub fn start(&mut self, _object: &mut Object) {}
    pub fn update(&mut self, object: &mut Object, component_registry: &ComponentRegistry, events: &mut Events) {
        for comp_id in object.components.iter() {
            let comp = &component_registry.0[*comp_id];
            if comp.signature() == ComponentSignature::Transform {
                events.add_message_event(self.id, *comp_id, Action::INC, self.velocity.x, "pos_x".to_string());
                events.add_message_event(self.id, *comp_id, Action::INC, self.velocity.y, "pos_y".to_string());
                break;
            }
        }
    }
}
