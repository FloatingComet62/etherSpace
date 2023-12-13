use crate::{objects::Object, serializer::Serialize};
use downcast_rs::{impl_downcast, DowncastSync};

pub trait Component: Serialize + DowncastSync {
    fn start(&mut self, object: &mut Object);
    fn update(&mut self, object: &mut Object);
    fn signature(&self) -> ComponentSignature;
}
impl_downcast!(sync Component);

// This took me 4 days to figure out
// https://www.reddit.com/r/rust/comments/droxdg/why_arent_traits_impld_for_boxdyn_trait/
impl Serialize for Box<dyn Component> {
    fn serialize(&self) -> String {
        (**self).serialize()
    }

    fn serial_items(&self, indent: u8) -> Vec<crate::serializer::SerialItem> {
        (**self).serial_items(indent)
    }

    fn serialize_nest(&self, indent: u8) -> String {
        (**self).serialize_nest(indent)
    }

    fn serialize_invec(&self, indent: u8) -> String {
        (**self).serialize_invec(indent)
    }
}

#[derive(PartialEq)]
pub enum ComponentSignature {
    Transform,
    TranslationalPhysics,
}

pub mod transform;
pub mod translational;
