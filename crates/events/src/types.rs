use introspect_types::ColumnDef;
use introspect_value::{FeltIterator, ToValue};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

use crate::value::UpdateRecordFields;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub table_id: Felt,
    pub table_name: String,
    pub attributes: Vec<String>,
    pub fields: Vec<ColumnDef>,
}

impl ToValue for TableSchema {
    type Value = UpdateRecordFields;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Self::Value> {
        Some(UpdateRecordFields {
            table_id: self.table_id,
            table_name: self.table_name.clone(),
            attributes: self.attributes.clone(),
            fields: self
                .fields
                .iter()
                .map(|f| f.to_value(data))
                .collect::<Option<Vec<_>>>()?,
        })
    }
}
