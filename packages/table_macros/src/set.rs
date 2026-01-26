use introspect_macros::i_type::IExtract;
use introspect_macros::i_type::extraction::IExtractablesContext;
use introspect_macros::{AsCairo, Struct};
use itertools::Itertools;

use crate::structure::{KeyType, get_keys_index};
use crate::templates::{
    column_set_impl_name_tpl, column_set_item_impl_tpl, column_set_member_impl_tpl,
    column_set_value_impl_tpl,
};
use crate::{Column, TableError};

pub enum ColumnSetKey {
    None,
    Single,
    Multiple,
}

pub struct ColumnSet {
    pub name: String,
    pub keys: KeyType,
    pub columns: Vec<Column>,
    pub impl_name: String,
}

impl IExtract for ColumnSet {
    type SyntaxType = Struct;
    type Error = TableError;
    fn iextract(item: &mut Self::SyntaxType) -> Result<ColumnSet, Self::Error> {
        let mut columns = item.members.iextracts_with(&item.name)?;
        let keys_index = get_keys_index(&columns)?;
        let keys = if keys_index == 1 && item.members[0].ty.is_primary_type() {
            KeyType::Primary(columns.remove(0).try_into()?)
        } else {
            KeyType::Custom(keys_index)
        };
        Ok(ColumnSet {
            name: item.name.clone(),
            keys,
            columns,
            impl_name: column_set_impl_name_tpl(&item.name),
        })
    }
}

impl ColumnSet {
    pub fn column_set_impl(&self, i_table_path: &str) -> String {
        match &self.keys {
            KeyType::Custom(k) if *k == 0 => self.column_set_value_impl(i_table_path),
            _ => self.column_set_item_impl(i_table_path),
        }
    }
    pub fn column_set_value_impl(&self, i_table_path: &str) -> String {
        let column_ids = self.columns.iter().map(|c| &c.id).join(",");
        let member_impls = self
            .columns
            .iter()
            .map(|c| c.set_member_impl(i_table_path))
            .join(",");
        let serialize_member_calls = self
            .columns
            .iter()
            .map(|c| c.serialize_member_call::<true>())
            .join("\n");
        column_set_value_impl_tpl(
            i_table_path,
            &self.name,
            &self.impl_name,
            &self.columns.len().to_string(),
            &member_impls,
            &column_ids,
            &serialize_member_calls,
        )
    }

    pub fn column_set_item_impl(&self, i_table_path: &str) -> String {
        let (snapped_key, self_keys) = match &self.keys {
            KeyType::Primary(p) => (
                format!("@{}", p.ty.as_cairo()),
                format!("self.{}", p.member,),
            ),
            KeyType::Custom(k) if *k == 1 => (
                format!("@{}", self.columns[0].ty.as_cairo()),
                format!("self.{}", self.columns[0].member),
            ),
            KeyType::Custom(s) => make_compound_key(&self.columns[..*s]),
        };
        let column_ids = self.columns.iter().map(|c| &c.id).join(",");
        let member_impls = self
            .columns
            .iter()
            .map(|c| c.set_member_impl(i_table_path))
            .join(",");
        let serialize_member_calls = self
            .columns
            .iter()
            .filter(|c| !c.key)
            .map(|c| c.serialize_member_call::<true>())
            .join("\n");
        column_set_item_impl_tpl(
            i_table_path,
            &self.name,
            &self.impl_name,
            &self.columns.len().to_string(),
            &member_impls,
            &column_ids,
            &self_keys,
            &snapped_key,
            &serialize_member_calls,
        )
    }
}

fn make_compound_key(columns: &[Column]) -> (String, String) {
    let key_types = columns
        .iter()
        .map(|c| format!("@{}", c.ty.as_cairo()))
        .join(",");
    let self_keys = columns
        .iter()
        .map(|c| format!("self.{}", c.member))
        .join(",");
    (format!("({key_types})"), format!("({self_keys})"))
}

impl Column {
    fn set_member_impl(&self, i_table_path: &str) -> String {
        column_set_member_impl_tpl(
            i_table_path,
            &self.member_impl_name,
            &self.id,
            &self.ty.as_cairo(),
        )
    }
}
