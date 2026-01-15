use introspect_macros::i_type::default::{TypeMod, TypeModMemberTrait};
use introspect_macros::i_type::{AttributeParser, AttributeVariant, DefaultIExtractor};
use introspect_macros::{Attribute, Member};
use introspect_rust_macros::macro_attributes;

use crate::{TableError, TableResult};

#[derive(Default)]
#[macro_attributes]
pub struct PrimaryAttribute {
    #[skip_accessors]
    type_mod: TypeMod,
    name: String,
}

impl TypeModMemberTrait for PrimaryAttribute {
    fn get_mut_type_mod(&mut self) -> &mut Option<TypeMod> {
        &mut self.type_mod
    }
}

impl AttributeParser<Member, PrimaryAttribute> for DefaultIExtractor {
    type Error = TableError;
    fn parse_attribute(
        &self,
        _item: &mut Member,
        macro_attributes: &mut PrimaryAttribute,
        attribute: Attribute,
    ) -> TableResult<Vec<AttributeVariant>> {
        if let Some(r) = macro_attributes.extract_type_mod_return_empty(&attribute) {
            return r.map_err(From::from);
        }
        match attribute.name.as_str() {
            "name" => macro_attributes.set_name_return_empty(attribute.single_unnamed_arg()?),
            _ => attribute.into(),
        }
    }
}
