use introspect_types::{CairoDeserialize, FeltIterator, TypeDef};
use serde::{Deserialize, Serialize};
use starknet::macros::selector;
use starknet_types_core::felt::Felt;

use crate::event::EventTrait;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeclareType {
    pub id: Felt,
    pub type_def: TypeDef,
}

impl EventTrait for DeclareType {
    const SELECTOR: Felt = selector!("DeclareType");

    fn deserialize_event(keys: &mut FeltIterator, data: &mut FeltIterator) -> Option<Self> {
        let id = keys.next()?;
        let type_def = TypeDef::c_deserialize(data)?;

        DeclareType { id, type_def }.verify(keys, data)
    }
}
