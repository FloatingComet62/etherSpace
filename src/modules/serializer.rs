use colored::Colorize;

#[macro_export]
macro_rules! gen_indent {
    ($ind: expr) => {
        "\t".repeat($ind.into())
    };
}

#[derive(Clone)]
pub struct SerialItem {
    pub key: String,
    pub value: String,
}
impl SerialItem {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
    pub fn new_str(key: &str, value: String) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }
}

#[inline]
pub fn serializer(serial_items: Vec<SerialItem>, indent: u8) -> String {
    serializer_invec_option(indent, serial_items, false)
}
#[inline]
pub fn serializer_invec(serial_items: Vec<SerialItem>, indent: u8) -> String {
    serializer_invec_option(indent, serial_items, true)
}

fn serializer_invec_option(
    indent: u8,
    serial_items: Vec<SerialItem>,
    is_inside_vec: bool,
) -> String {
    let mut output = String::new();

    for i in 0..serial_items.len() {
        let serial_item = &serial_items[i];
        output += &format!(
            "{}{}: {}{}",
            if is_inside_vec && i == 0 {
                "".to_string()
            } else {
                gen_indent!(indent)
            },
            serial_item.key.bright_green(),
            serial_item.value,
            if i < serial_items.len() - 1 || !is_inside_vec {
                "\n"
            } else {
                ""
            }
        );
    }
    output
}

pub trait Serialize {
    fn serialize(&self) -> String {
        self.serialize_nest(0)
    }
    fn serialize_reg(&self, registry: Registry) -> String {
        self.serialize_nest_reg(0, registry)
    }
    fn serial_items(&self, indent: u8) -> Vec<SerialItem>;
    fn serial_items_reg(&self, indent: u8, registry: Registry) -> Vec<SerialItem>;
    fn serialize_nest(&self, indent: u8) -> String {
        serializer(self.serial_items(indent), indent)
    }
    fn serialize_nest_reg(&self, indent: u8, registry: Registry) -> String {
        serializer(self.serial_items_reg(indent, registry), indent)
    }
    fn serialize_invec(&self, indent: u8) -> String {
        serializer_invec(self.serial_items(indent), indent)
    }
}

#[inline]
pub fn serializer_vec(vec: &Vec<impl Serialize>) -> String {
    serializer_vec_nest_option(vec, 0)
}
#[inline]
pub fn serializer_vec_nest(vec: &Vec<impl Serialize>, indent: u8) -> String {
    serializer_vec_nest_option(vec, indent)
}

fn serializer_vec_nest_option(vec: &Vec<impl Serialize>, indent: u8) -> String {
    if vec.len() == 0 {
        return "[]".to_string();
    }
    let terminator = format!("\n{}", gen_indent!(indent));
    let mut str = String::new();
    str += &format!("[{}", terminator);
    for i in 0..vec.len() {
        let item = &vec[i];
        str += &item.serialize_invec(indent);
        if i < vec.len() - 1 {
            str += &format!(";{}", terminator);
        } else {
            str += &format!(";\n{}]", gen_indent!(indent - 1));
        }
    }

    str
}

use crate::{components::{transform::Transform, translational::Translational, Component}, critical, modules::{log::Log, vector::Vector2}, objects::Object, registry::Registry, world::World};
impl Serialize for World {
    fn serial_items(&self, indent: u8) -> Vec<SerialItem> {
        let object_map: Vec<Object>;
        {
            let raw_registry = self.registry.lock();
            if raw_registry.is_err() {
                critical!("Registry is locked");
            }
            let registry = raw_registry.unwrap();
            object_map = self
                .objects
                .iter()
                .map(|obj_id| (*registry.get_object(*obj_id)).clone())
                .collect();
        }
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str("objects", serializer_vec_nest(&object_map, indent + 1)),
        ]
        .to_vec()
    }
    fn serial_items_reg(&self, indent: u8, registry: Registry) -> Vec<SerialItem> {
        let object_map = self
                .objects
                .iter()
                .map(|obj_id| (*registry.get_object(*obj_id)).clone())
                .collect();
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str("objects", serializer_vec_nest(&object_map, indent + 1)),
        ]
        .to_vec()
    }
}
impl Serialize for Object {
    fn serial_items(&self, indent: u8) -> Vec<SerialItem> {
        let component_map: Vec<Component>;
        {
            let raw_registry = self.registry.lock();
            if raw_registry.is_err() {
                critical!("Registry is locked");
            }
            let registry = raw_registry.unwrap();
            component_map = self
                .components
                .iter()
                .map(|comp_id| (*registry.get_component(*comp_id)).clone())
                .collect();
        }
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str(
                "components",
                serializer_vec_nest(&component_map, indent + 1),
            ),
        ]
        .to_vec()
    }
    fn serial_items_reg(&self, indent: u8, registry: Registry) -> Vec<SerialItem> {
        let component_map = self
                .components
                .iter()
                .map(|comp_id| (*registry.get_component(*comp_id)).clone())
                .collect();
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str(
                "components",
                serializer_vec_nest(&component_map, indent + 1),
            ),
        ]
        .to_vec()
    }
}
impl Serialize for Component {
    fn serialize(&self) -> String {
        match &self {
            Component::Transform(component) => component.serialize(),
            Component::Translational(component) => component.serialize(),
        }
    }

    fn serial_items(&self, indent: u8) -> Vec<SerialItem> {
        match &self {
            Component::Transform(component) => component.serial_items(indent),
            Component::Translational(component) => component.serial_items(indent),
        }
    }

    fn serialize_nest(&self, indent: u8) -> String {
        match &self {
            Component::Transform(component) => component.serialize_nest(indent),
            Component::Translational(component) => component.serialize_nest(indent),
        }
    }

    fn serialize_invec(&self, indent: u8) -> String {
        match &self {
            Component::Transform(component) => component.serialize_invec(indent),
            Component::Translational(component) => component.serialize_invec(indent),
        }
    }
    fn serial_items_reg(&self, indent: u8, registry: Registry) -> Vec<SerialItem> {
        match &self {
            Component::Transform(component) => component.serial_items_reg(indent, registry),
            Component::Translational(component) => component.serial_items_reg(indent, registry),
        }
    }
}
impl Serialize for Transform {
    fn serial_items(&self, _indent: u8) -> Vec<SerialItem> {
        let vec_printer = |vec: &Vector2<f64>| format!("Vector2({}, {})", vec.x, vec.y);
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str("position", vec_printer(&self.position)),
        ]
        .to_vec()
    }
    fn serial_items_reg(&self, indent: u8, _registry: Registry) -> Vec<SerialItem> {
        self.serial_items(indent)
    }
}
impl Serialize for Translational {
    fn serial_items(&self, _indent: u8) -> Vec<SerialItem> {
        let vec_printer = |vec: &Vector2<f64>| format!("Vector2({}, {})", vec.x, vec.y);
        [
            SerialItem::new_str("id", self.id.to_string()),
            SerialItem::new_str("velocity", vec_printer(&self.velocity)),
        ]
        .to_vec()
    }
    fn serial_items_reg(&self, indent: u8, _registry: Registry) -> Vec<SerialItem> {
        self.serial_items(indent)
    }
}
impl Serialize for u32 {
    fn serial_items(&self, _indent: u8) -> Vec<SerialItem> {
        [SerialItem::new_str("value", self.to_string())].to_vec()
    }
    fn serial_items_reg(&self, indent: u8, _registry: Registry) -> Vec<SerialItem> {
        self.serial_items(indent)
    }
}
