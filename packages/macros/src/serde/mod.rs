use indent::indent_by;

use crate::IntrospectItem;
use crate::items::IntrospectItemTrait;

mod derive;
mod enums;
mod structs;

const ISERDE_IMPL_TPL: &str = include_str!("../../templates/iserde_impl.cairo");
const ISERDE_SERIALIZE_CALL: &str = "introspect::ISerde::iserialize";

pub trait ToISerdeImpl
where
    Self: IntrospectItemTrait,
{
    fn to_iserde_impl(&self) -> String {
        ISERDE_IMPL_TPL
            .replace("{{name}}", self.name())
            .replace("{{full_name}}", &self.full_name())
            .replace("{{body}}", &indent_by(8, self.iserde_body()))
            .replace(
                "{{impl_params}}",
                &self.generics_with_traits(&["introspect::ISerde"]),
            )
    }
    fn iserde_body(&self) -> String;
}

impl ToISerdeImpl for IntrospectItem {
    fn iserde_body(&self) -> String {
        match self {
            IntrospectItem::Struct(s) => s.iserde_body(),
            IntrospectItem::Enum(e) => e.iserde_body(),
        }
    }
}
