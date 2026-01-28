use crate::serde::ToISerdeImpl;
use crate::{I_PATH, IEnum, IVariant, ItemTrait};

impl ToISerdeImpl for IEnum {
    fn serialize_body(&self) -> String {
        let enum_call = self.full_call();
        let variants: String = self
            .variants
            .iter()
            .map(|v| v.serialize_variant(&enum_call))
            .collect();
        format!("match self {{\n{variants}\n}};")
    }

    fn deserialize_body(&self) -> String {
        let enum_call = self.full_call();
        let variants: String = self
            .variants
            .iter()
            .map(|v| v.deserialize_variant(&enum_call))
            .collect();
        format!("match *serialized.pop_front()? {{{variants} _ => None}}")
    }
}

impl IVariant {
    pub fn serialize_variant(&self, enum_name: &str) -> String {
        let selector = self.selector.to_fixed_hex_string();
        let variant_name = &self.field;
        match self.ty {
            None => format!("{enum_name}::{variant_name} => output.append({selector}),"),
            Some(_) => format!(
                "{enum_name}::{variant_name}(value) => {I_PATH}::iserialize_keyed_type({selector}, value, ref output),",
            ),
        }
    }

    pub fn deserialize_variant(&self, enum_name: &str) -> String {
        let selector = self.selector.to_fixed_hex_string();
        let field = &self.field;
        match &self.ty {
            None => format!("{selector} => Some({enum_name}::{field}),"),
            Some(_) => format!(
                "{selector} => Some({enum_name}::{field}({I_PATH}::ideserialize(ref serialized)?)),",
            ),
        }
    }
}
