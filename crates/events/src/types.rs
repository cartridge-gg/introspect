use introspect_types::{CairoDeserialize, TypeDef};
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

    fn deserialize_event(keys: Vec<Felt>, data: Vec<Felt>) -> Option<Self> {
        let mut keys_iter = keys.into_iter();
        let mut data_iter = data.into_iter();

        let id = keys_iter.next()?;
        let type_def = TypeDef::c_deserialize(&mut data_iter)?;

        DeclareType { id, type_def }.verify(&mut keys_iter, &mut data_iter)
    }
}
