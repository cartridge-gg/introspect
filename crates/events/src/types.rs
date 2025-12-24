use introspect_types::{FeltIterator, ISerde, TypeDef, ascii_str_to_limbs};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

use crate::event::EventTrait;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeclareType {
    pub id: Felt,
    pub type_def: TypeDef,
}

impl EventTrait for DeclareType {
    const SELECTOR_RAW: [u64; 4] = ascii_str_to_limbs("DeclareType");

    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        let id = keys.next()?;
        let type_def = TypeDef::ideserialize(data)?;

        DeclareType { id, type_def }.verify(keys, data)
    }
}
