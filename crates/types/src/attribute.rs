use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Attribute {
    pub id: Felt,
    pub data: Vec<Felt>,
}

impl Attribute {
    pub fn new_empty(id: Felt) -> Attribute {
        Attribute {
            id,
            data: Default::default(),
        }
    }
}
