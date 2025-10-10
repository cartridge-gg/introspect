use introspect_types::{FieldDef, FieldDefVec};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

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
