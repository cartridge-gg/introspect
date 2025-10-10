use introspect_types::FieldInfo;
use introspect_value::{Field, Value};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRecordField {
    pub table_id: Felt,
    pub table_name: String,
    pub attrs: Vec<String>,
    pub field: Field,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRecordFields {
    pub table_id: Felt,
    pub table_name: String,
    pub attrs: Vec<String>,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRecordsField {
    pub table_id: Felt,
    pub table_name: String,
    pub attrs: Vec<String>,
    pub field_info: FieldInfo,
    pub values: Vec<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRecordsFields {
    pub table_id: Felt,
    pub table_name: String,
    pub attrs: Vec<String>,
    pub field_infos: Vec<FieldInfo>,
    pub values: Vec<Vec<Value>>, //row-major
}
