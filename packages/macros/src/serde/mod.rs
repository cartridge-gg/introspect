use crate::i_type::IntrospectItemTrait;
use crate::{I_PATH, IItem, ItemTrait};
use const_format::formatcp;

mod derive;
mod enums;
mod structs;

const ISERDE_IMPL_TPL: &str = include_str!("../../templates/iserde_impl.cairo");
const ISERDE_SERIALIZE_CALL: &str = formatcp!("{I_PATH}::iserialize");
const ISERDE_DESERIALIZE_CALL: &str = formatcp!("{I_PATH}::ideserialize");
const ISERDE_TRAIT: &str = formatcp!("{I_PATH}::ISerde");

pub trait ToISerdeImpl
where
    Self: IntrospectItemTrait + ItemTrait,
{
    fn serialize_body(&self) -> String;
    fn deserialize_body(&self) -> String;
    fn to_iserde_impl(&self) -> String {
        ISERDE_IMPL_TPL
            .replace("{{i_path}}", I_PATH)
            .replace("{{name}}", self.name())
            .replace("{{full_name}}", &self.full_name())
            .replace("{{serialize_body}}", &self.serialize_body())
            .replace("{{deserialize_body}}", &self.deserialize_body())
            .replace(
                "{{impl_params}}",
                &self.generics_with_traits(&[ISERDE_TRAIT, "Drop"]),
            )
    }
}

impl ToISerdeImpl for IItem {
    fn serialize_body(&self) -> String {
        match self {
            IItem::Struct(s) => s.serialize_body(),
            IItem::Enum(e) => e.serialize_body(),
        }
    }

    fn deserialize_body(&self) -> String {
        match self {
            IItem::Struct(s) => s.deserialize_body(),
            IItem::Enum(e) => e.deserialize_body(),
        }
    }
}
