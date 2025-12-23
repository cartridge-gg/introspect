use crate::attribute::{Attribute, IAttribute};

const ATTRIBUTE_TPL: &str = include_str!("../../templates/attribute.cairo");

pub fn make_attribute_string(id: &str, data: &[String]) -> String {
    ATTRIBUTE_TPL
        .replace("{{id}}", id)
        .replace("{{data}}", &data.join(", "))
}

// fn parse_member_attribute(attribute: Attribute) -> String{

// }
pub fn make_attributes_string(_attributes: &[IAttribute]) -> String {
    "".to_string()
}
