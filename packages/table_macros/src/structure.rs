use crate::primary::Primary;
use crate::templates::{
    columns_mod_name_tpl, member_impl_tpl, record_id_impl_tpl, serialize_member_call_tpl,
    struct_impl_name_tpl, structure_impls_tpl,
};
use crate::{Column, TableError, TableResult};
use introspect_macros::i_type::{DefaultIExtractor, IExtract, ITys};
use introspect_macros::table::primary::PrimaryTypeDefVariant;
use introspect_macros::ty::TyItem;
use introspect_macros::type_def::CairoElementDefWith;
use introspect_macros::{
    AsCairo, AsCairoBytes, AttributesTrait, CairoElementDefs, CollectionsAsCairo, Member, Struct,
    Ty,
};

#[derive(Clone, Debug)]
pub enum KeyType {
    None,
    Primary(Primary),
    Custom(Vec<Key>),
}

#[derive(Clone, Debug)]
pub struct Key {
    pub name: String,
    pub ty: Ty,
}

#[derive(Clone, Debug)]
pub struct TableStructure {
    pub name: String,
    pub key: KeyType,
    pub columns: Vec<Column>,
    pub struct_impl_name: String,
    pub columns_mod_name: String,
}

trait TableMemberTrait {
    fn is_primary(&self) -> bool;
    fn is_key(&self) -> bool;
    fn to_key(&self) -> Key;
}

impl TableMemberTrait for Member {
    fn is_key(&self) -> bool {
        self.has_name_only_attribute("key")
    }

    fn is_primary(&self) -> bool {
        self.ty.is_primary_type()
    }

    fn to_key(&self) -> Key {
        Key {
            name: self.name.clone(),
            ty: self.ty.clone(),
        }
    }
}

fn default_primary_def() -> Primary {
    Primary {
        name: "__id".to_string(),
        member: None,
        attributes: vec![],
        ty: Ty::Item(TyItem {
            name: "felt252".to_string(),
            params: None,
        }),
        type_def: PrimaryTypeDefVariant::Default,
    }
}

impl IExtract<TableStructure> for DefaultIExtractor {
    type SyntaxType = Struct;
    type Error = TableError;
    fn iextract(&self, item: &mut Self::SyntaxType) -> Result<TableStructure, Self::Error> {
        let keys = get_keys(&item.members)?;
        let mut members = item.members.iter_mut();
        let key = match keys {
            None => KeyType::Primary(self.iextract(members.next().unwrap())?),
            Some(keys) if keys.is_empty() => KeyType::None,
            Some(keys) => KeyType::Custom(keys),
        };
        let columns = members
            .map(|m| self.iextract(m))
            .collect::<TableResult<Vec<_>>>()?;
        Ok(TableStructure {
            name: item.name.clone(),
            key,
            columns,
            struct_impl_name: struct_impl_name_tpl(&item.name),
            columns_mod_name: columns_mod_name_tpl(&item.name),
        })
    }
}

fn get_keys(members: &[Member]) -> TableResult<Option<Vec<Key>>> {
    let mut position = 0;
    for (i, member) in members.iter().enumerate() {
        if member.is_key() {
            match position == i {
                true => position += 1,
                false => return Err(TableError::KeysNotFirst),
            }
        }
    }
    match position == 1 && members[0].is_primary() {
        true => Ok(None),
        false => Ok(Some(
            members[..position].iter().map(Member::to_key).collect(),
        )),
    }
}

impl TableStructure {
    pub fn get_structure_impl(&self, i_path: &str) -> String {
        let mut column_defs = Vec::new();
        let mut member_impls = Vec::new();
        let mut column_id_consts = Vec::new();
        let mut tys = Vec::new();
        let mut serialize_member_calls = Vec::new();
        for column in &self.columns {
            column_id_consts.push(column.id_const());
            tys.push(&column.ty);
            column_defs.push(column.as_element_def_with(i_path, &self.columns_mod_name));
            let member_impl_name = column.serialize_member_impl_name(&self.name);
            member_impls.push(member_impl_tpl(
                i_path,
                &member_impl_name,
                &self.struct_impl_name,
                &self.columns_mod_name,
                &column.member,
                &column.ty.as_cairo(),
            ));
            serialize_member_calls
                .push(serialize_member_call_tpl(&member_impl_name, &column.member));
        }
        let (primary, key_impls) = match &self.key {
            KeyType::Primary(p) => (
                p,
                record_id_impl_tpl(
                    i_path,
                    &self.name,
                    &self.struct_impl_name,
                    &p.member.as_ref().unwrap(),
                ),
            ),
            KeyType::Custom(k) => (&default_primary_def(), "".to_string()),
            KeyType::None => (&default_primary_def(), "".to_string()),
        };
        structure_impls_tpl(
            i_path,
            &self.name,
            &self.struct_impl_name,
            &primary.ty.as_cairo(),
            &primary.name.as_cairo_byte_array(),
            &primary.attributes.as_element_defs_span(i_path),
            &primary.type_def.type_def(&primary.ty, i_path),
            &self.columns_mod_name,
            &column_id_consts.join(";\n"),
            &column_defs.as_cairo_span(),
            &tys.collect_child_defs(i_path),
            &member_impls.join("\n"),
            &serialize_member_calls.join("\n"),
        ) + &key_impls
    }
}
