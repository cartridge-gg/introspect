use std::collections::VecDeque;

use introspect_types::{FieldDef, FieldDefVec};
use introspect_value::ToValue;
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

use crate::value::{UpdateRecordField, UpdateRecordFields};

pub struct TableFieldDef {
    pub id: Felt,
    pub name: String,
    pub attrs: Vec<String>,
    pub field: FieldDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableFieldsDef {
    pub id: Felt,
    pub name: String,
    pub attrs: Vec<String>,
    pub fields: FieldDefVec,
}

impl ToValue for TableFieldDef {
    type Value = UpdateRecordField;
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Self::Value> {
        Some(UpdateRecordField {
            table_id: self.id,
            table_name: self.name.clone(),
            attrs: self.attrs.clone(),
            field: self.field.to_value(data)?,
        })
    }
}

impl ToValue for TableFieldsDef {
    type Value = UpdateRecordFields;
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Self::Value> {
        Some(UpdateRecordFields {
            table_id: self.id,
            table_name: self.name.clone(),
            attrs: self.attrs.clone(),
            fields: self.fields.to_value(data)?,
        })
    }
}
