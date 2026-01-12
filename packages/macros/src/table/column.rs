use crate::i_type::TypeDefVariant;
use crate::i_type::structs::IMember;
use crate::utils::string_to_keccak_felt;
use crate::{AsCairo, AsCairoBytes, CollectionsAsCairo, IAttribute, Ty};
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug)]
pub struct Column {
    pub id: Felt,
    pub name: String,
    pub member: String,
    pub attributes: Vec<IAttribute>,
    pub ty: Ty,
    pub type_def: TypeDefVariant,
}

pub fn column_def_tpl(id: &str, name: &str, attributes: &str, type_def: &str) -> String {
    format!("introspect::column_def({id}, {name}, {attributes}, {type_def})")
}

impl IMember {
    pub fn to_column_default(&self) -> Column {
        Column {
            id: string_to_keccak_felt(&self.name),
            name: self.name.clone(),
            member: self.name.clone(),
            ty: self.ty.clone(),
            attributes: self.attributes.clone(),
            type_def: TypeDefVariant::Default,
        }
    }
    pub fn to_column(&self, id: Felt, name: String) -> Column {
        Column {
            id,
            name,
            member: self.name.clone(),
            ty: self.ty.clone(),
            attributes: self.attributes.clone(),
            type_def: self.type_def.clone(),
        }
    }
}

impl AsCairo for Column {
    fn as_cairo(&self) -> String {
        column_def_tpl(
            &self.id.as_cairo(),
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
            self.type_def.type_def(self.ty.clone()).as_str(),
        )
    }
}
