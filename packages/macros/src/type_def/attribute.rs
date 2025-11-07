use crate::attribute::Attribute;

const ATTRIBUTE_TPL: &str = include_str!("../../templates/attribute.cairo");

pub fn parse_member_attributes(attributes: &[Attribute]) -> Vec<String> {
    let mut attrs = Vec::new();
    for attr in attributes {
        if attr.name != "key" {
            attrs.push(attr.to_string());
        }
    }
    attrs
}

pub fn make_attribute_string(id: &str, data: &[String]) -> String {
    ATTRIBUTE_TPL
        .replace("{{id}}", id)
        .replace("{{data}}", &data.join(", "))
}

// fn parse_member_attribute(attribute: Attribute) -> String{

// }
pub fn make_attributes_string(_attributes: &[Attribute]) -> String {
    "".to_string()
}
