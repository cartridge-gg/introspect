use crate::i_type::IntrospectItemTrait;
use crate::serde::ToISerdeImpl;
use crate::{IEnum, IVariant};

impl ToISerdeImpl for IEnum {
    fn iserde_body(&self) -> String {
        let enum_call = self.full_call();
        let variants = self
            .variants
            .iter()
            .map(|v| v.iserde_variant(&enum_call))
            .collect::<Vec<_>>()
            .join("\n");
        format!("match self {{\n{variants}\n}};")
    }
}

impl IVariant {
    pub fn iserde_variant(&self, enum_name: &str) -> String {
        let selector = &self.selector;
        let variant_name = &self.name;
        match self.ty {
            None => format!("{enum_name}::{variant_name} => output.append({selector}),"),
            Some(_) => format!(
                "{enum_name}::{variant_name}(value) => introspect::iserialize_keyed_type({selector}, value, ref output),",
            ),
        }
    }
}
