use introspect_types::ColumnDefVec;
use introspect_value::{FeltIterator, ToValue};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

use crate::value::UpdateRecordFields;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub table_id: Felt,
    pub table_name: String,
    pub attrs: Vec<String>,
    pub fields: ColumnDefVec,
}

impl ToValue for TableSchema {
    type Value = UpdateRecordFields;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Self::Value> {
        Some(UpdateRecordFields {
            table_id: self.table_id,
            table_name: self.table_name.clone(),
            attrs: self.attrs.clone(),
            fields: self.fields.to_value(data)?,
        })
    }
}
