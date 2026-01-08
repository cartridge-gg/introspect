use crate::items::structs::IMember;
use crate::{AsCairo, CollectionsAsCairo, IAttribute, Ty};
use starknet_types_core::felt::Felt;

const COLUMN_TYPE_DEF_TPL: &str = include_str!("../../templates/column_def.cairo");

pub struct Column {
    pub id: Felt,
    pub name: String,
    pub ty: Ty,
    pub attributes: Vec<IAttribute>,
}

impl IMember {
    fn to_column(&self, id: Felt) -> Column {
        Column {
            id,
            name: self.name.clone(),
            ty: self.ty.clone(),
            attributes: self.attributes.clone(),
        }
    }
}

impl AsCairo for Column {
    fn as_cairo(&self) -> String {
        COLUMN_TYPE_DEF_TPL
            .replace("{{id}}", &self.id.as_cairo())
            .replace("{{name}}", &self.name)
            .replace("{{attributes_str}}", &self.attributes.as_cairo_block())
            .replace("{{type_def}}", &self.ty)
    }
}
