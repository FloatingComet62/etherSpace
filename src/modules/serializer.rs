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
    fn serial_items(&self, indent: u8) -> Vec<SerialItem>;
    fn serialize_nest(&self, indent: u8) -> String {
        serializer(self.serial_items(indent), indent)
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
