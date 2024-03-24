use super::ComponentSignature;
use crate::{events::{Action, Events}, modules::vector::Vector2, objects::Object, registry::ComponentRegistry};
use serde::{Deserialize, Serialize};

/// # Transform
/// * `id` - A unique ID
/// * `position` - Position of the object
/// * `requires` - Components which the component requires
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Transform {
    pub id: usize,
    pub position: Vector2,
    requires: Vec<ComponentSignature>,
}
impl Transform {
    pub fn new(id: usize, position: Vector2) -> Self {
        Self {
            id,
            position,
            requires: vec![],
        }
    }
    pub fn get_requirements(&self) -> Vec<ComponentSignature> {
        self.requires.clone()
    }
    pub fn start(&mut self, _object: &mut Object) {}
    pub fn update(
        &mut self,
        _object: &mut Object,
        _component_registry: &ComponentRegistry,
        events: &mut Events
    ) {
        let mut to_remove = vec![];
        events.receive_message_events(self.id).for_each(|message_event| {
            macro_rules! action {
                ($var: expr, $string: expr) => {
                    if message_event.message == $string {
                        match message_event.action {
                            Action::INC => $var += message_event.value,
                            Action::DEC => $var -= message_event.value,
                            Action::SET => $var = message_event.value,
                        }
                    }
                }
            }
            action!(self.position.x, "pos_x");
            action!(self.position.y, "pos_y");
            to_remove.push(message_event.id);
        });
        events.remove(to_remove);
    }
}
