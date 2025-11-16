use introspect_types::ColumnDef;
use introspect_types::schema::PrimaryTypeDef;
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<String>,
    pub primary: PrimaryTypeDef,
    pub columns: Vec<ColumnDef>,
}
