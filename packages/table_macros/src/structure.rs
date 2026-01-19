use crate::primary::Primary;
use crate::templates::{
    columns_mod_name_tpl, keyed_impls_tpl, record_id_impl_tpl, single_key_impls_tpl,
    snappable_key_tpl, struct_impl_name_tpl, structure_impls_tpl,
};
use crate::{Column, TableError, TableResult};
use introspect_macros::i_type::extraction::IExtractablesContext;
use introspect_macros::i_type::{IExtract, ITys};
use introspect_macros::table::primary::PrimaryTypeDefVariant;
use introspect_macros::ty::TyItem;
use introspect_macros::type_def::CairoElementDefWith;
use introspect_macros::{
    AsCairo, AsCairoBytes, CairoElementDefs, CollectionsAsCairo, IAttribute, Member, Struct, Ty,
};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub enum KeyType {
    Primary(Primary),
    Custom(usize),
}

#[derive(Clone, Debug)]
pub struct TableStructure {
    pub name: String,
    pub key: KeyType,
    pub columns: Vec<Column>,
    pub attributes: Vec<IAttribute>,
    pub impl_name: String,
    pub columns_mod_name: String,
}

trait TableMemberTrait {
    fn is_primary(&self) -> bool;
}

impl TableMemberTrait for Member {
    fn is_primary(&self) -> bool {
        self.ty.is_primary_type()
    }
}

fn default_primary_def() -> Primary {
    Primary {
        name: "__id".to_string(),
        member: String::new(),
        attributes: vec![],
        ty: Ty::Item(TyItem {
            name: "felt252".to_string(),
            params: None,
        }),
        type_def: PrimaryTypeDefVariant::Default,
    }
}

pub fn get_keys_index(columns: &[Column]) -> TableResult<usize> {
    let mut position = 0;
    for (i, column) in columns.iter().enumerate() {
        if column.key {
            match position == i {
                true => position += 1,
                false => return Err(TableError::KeysNotFirst),
            }
        }
    }
    Ok(position)
}

impl IExtract for TableStructure {
    type SyntaxType = Struct;
    type Error = TableError;
    fn iextract(item: &mut Self::SyntaxType) -> Result<TableStructure, Self::Error> {
        let mut columns = item.members.iextracts_with(&item.name)?;
        let keys_index = get_keys_index(&columns)?;
        let key = if keys_index == 1 && item.members[0].is_primary() {
            KeyType::Primary(columns.remove(0).try_into()?)
        } else {
            KeyType::Custom(keys_index)
        };
        Ok(TableStructure {
            name: item.name.clone(),
            key,
            attributes: vec![],
            columns,
            impl_name: struct_impl_name_tpl(&item.name),
            columns_mod_name: columns_mod_name_tpl(&item.name),
        })
    }
}

impl TableStructure {
    pub fn get_structure_impl(&self, i_path: &str, i_table_path: &str) -> String {
        let mut column_defs = Vec::new();
        let mut member_impls = Vec::new();
        let mut column_id_consts = Vec::new();
        let mut tys = Vec::new();
        let mut serialize_member_calls = Vec::new();
        for column in &self.columns {
            column_id_consts.push(column.id_const());
            tys.push(&column.ty);
            column_defs.push(column.as_element_def_with(i_path, &self.columns_mod_name));
            member_impls.push(column.member_impl(
                i_table_path,
                &self.impl_name,
                &self.columns_mod_name,
            ));
            if !column.key {
                serialize_member_calls.push(column.serialize_member_call::<true>());
            }
        }
        let (primary, key_impls) = match &self.key {
            KeyType::Primary(p) => (
                p,
                record_id_impl_tpl(i_path, i_table_path, &self.name, &self.impl_name, &p.member),
            ),
            KeyType::Custom(k) => {
                let key_impls = match *k {
                    0 => "".to_string(),
                    1 => self.get_single_key_impls(i_table_path),
                    _ => self.get_keyed_impls(i_table_path),
                };
                (&default_primary_def(), key_impls)
            }
        };
        structure_impls_tpl(
            i_path,
            i_table_path,
            &self.name,
            &self.impl_name,
            &primary.ty.as_cairo(),
            &primary.name.as_cairo_byte_array(),
            &primary.attributes.as_element_defs_span(i_path),
            &primary.type_def.type_def(&primary.ty, i_path),
            &self.columns_mod_name,
            &column_id_consts.join("\n"),
            &column_defs.as_cairo_span(),
            &tys.collect_child_defs(i_path),
            &member_impls.join("\n"),
            &serialize_member_calls.join("\n"),
        ) + &key_impls
    }

    pub fn get_keyed_impls(&self, i_table_path: &str) -> String {
        let keys = self.columns.iter().filter(|c| c.key).collect::<Vec<_>>();
        let key_types: Vec<_> = keys.iter().map(|c| c.ty.as_cairo()).collect();
        let key_types_ss = key_types.iter().map(|k| format!("@{k}")).join(",");
        let serialize_calls = keys
            .iter()
            .map(|c| c.serialize_member_call::<false>())
            .join("\n");
        let key_members = keys.iter().map(|c| &c.member).join(",");
        let self_key_members = keys.iter().map(|c| format!("self.{}", c.member)).join(",");
        let generics = (0..keys.len()).map(|i| format!("K{i}")).join(", ");
        let snappables = key_types
            .iter()
            .enumerate()
            .map(|(i, c)| snappable_key_tpl(i_table_path, i, c))
            .join(",");
        keyed_impls_tpl(
            i_table_path,
            &self.name,
            &self.impl_name,
            &key_types.join(","),
            &key_types_ss,
            &serialize_calls,
            &key_members,
            &self_key_members,
            &generics,
            &snappables,
        )
    }

    pub fn get_single_key_impls(&self, i_table_path: &str) -> String {
        let key = self.columns.iter().find(|c| c.key).unwrap();
        single_key_impls_tpl(
            i_table_path,
            &self.name,
            &self.impl_name,
            &key.ty.as_cairo(),
            &key.member,
            &key.member_impl_name,
        )
    }
}
