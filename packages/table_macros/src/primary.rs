use introspect_macros::table::PrimaryTypeDefVariant;
use introspect_macros::{IAttribute, Ty};

use crate::{Column, TableError};

#[derive(Clone, Debug)]
pub struct Primary {
    pub name: String,
    pub member: String,
    pub attributes: Vec<IAttribute>,
    pub ty: Ty,
    pub type_def: PrimaryTypeDefVariant,
}

// impl IExtract for Primary {
//     type SyntaxType = Member;
//     type Error = TableError;
//     fn iextract(member: &mut Member) -> TableResult<Primary> {
//         let (TypeModAndName { type_mod, name }, attributes) = member.extract_attributes()?;
//         Ok(Primary {
//             name: name.unwrap_or_else(|| member.name.clone()),
//             member: member.name.clone(),
//             attributes,
//             ty: member.ty.clone(),
//             type_def: type_mod.get_type_def(&member.ty)?.try_into()?, //TODO: support type_mod,
//         })
//     }
// }

impl TryFrom<Column> for Primary {
    type Error = TableError;
    fn try_from(column: Column) -> Result<Self, Self::Error> {
        Ok(Primary {
            name: column.name,
            member: column.member,
            attributes: column.attributes,
            ty: column.ty,
            type_def: column.type_def.try_into()?,
        })
    }
}
