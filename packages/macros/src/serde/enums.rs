use crate::items::ItemTrait;
use crate::serde::{ISERDE_SERIALIZE_CALL, ToISerdeImpl};
use crate::{Enum, Variant};
use indent::indent_by;
use indoc::formatdoc;

impl<'db> ToISerdeImpl for Enum<'db> {
    fn iserde_body(&self) -> String {
        let enum_call = self.full_call();
        let variants = self
            .variants
            .iter()
            .map(|v| iserde_variant(&enum_call, v))
            .collect::<Vec<_>>()
            .join("\n");
        let variants = indent_by(4, variants);
        formatdoc!(
            "
        match self {{
            {variants}
        }};"
        )
    }
}

fn iserde_variant<'db>(enum_name: &str, variant: &Variant<'db>) -> String {
    match variant.ty {
        None => formatdoc!(
            "{enum_name}::{variant_name} => output.append({selector}),",
            selector = variant.selector,
            variant_name = variant.name
        ),
        Some(_) => formatdoc!(
            "{enum_name}::{variant_name}(value) => {{
                output.append({selector});
                {ISERDE_SERIALIZE_CALL}((value), ref output)
            }},",
            selector = variant.selector,
            variant_name = variant.name
        ),
    }
}
