use random_number::random;

use crate::{
    log::Log,
    modules::vector::Vector2,
    objects::Object,
    serializer::{serializer, serializer_invec, SerialItem, Serialize},
};

use super::{transform::Transform, Component, ComponentSignature};

pub struct Translational {
    pub id: u32,
    pub velocity: Vector2<f64>,
    transform: Option<Box<&'static mut Transform>>,
}
impl Translational {
    pub fn new() -> Self {
        Self {
            id: random!(),
            velocity: Vector2::new(0.0, 0.0),
            transform: None,
        }
    }
}
impl Component for Translational {
    fn start(&mut self, object: &mut Object) {
        let response = object.get_component_mut(ComponentSignature::Transform);
        match response {
            Some(mut res) => {
                let t = res.as_mut();
                let u: Option<&mut Transform> = t.downcast_mut::<Transform>();
                if let Some(k) = u {
                    self.transform = Some(Box::new(k));
                }
            }
            None => Log::critical("Transform component is required for Translational component"),
        }
    }
    fn update(&mut self, _object: &mut Object) {}
    fn signature(&self) -> ComponentSignature {
        ComponentSignature::TranslationalPhysics
    }
}
impl Serialize for Translational {
    fn serialize(&self) -> String {
        self.serialize_nest(0)
    }
    fn serial_items(&self, _indent: u8) -> Vec<SerialItem> {
        let vec_printer = |vec: &Vector2<f64>| format!("Vector2({}, {})", vec.x, vec.y);
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str("velocity", vec_printer(&self.velocity)),
        ]
        .to_vec()
    }
    fn serialize_nest(&self, indent: u8) -> String {
        serializer(self.serial_items(indent), indent)
    }
    fn serialize_invec(&self, indent: u8) -> String {
        serializer_invec(self.serial_items(indent), indent)
    }
}
