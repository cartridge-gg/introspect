use introspect_types::{ISerde, TypeDef};
use starknet::Event;
use crate::utils::VerifyEventDeserializeTrait;

pub mod selectors {
    pub const DeclareType: felt252 = selector!("DeclareType");
}

/// Declares a reusable schema layout.
///
/// Fields:
/// - `id`: Deterministic schema ID (e.g., hash of fields).
/// - `fields`: Field members (selector + layout + attributes)

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeclareType {
    #[key]
    pub id: felt252,
    pub type_def: TypeDef,
}


impl DeclareTypeEvent of Event<DeclareType> {
    fn append_keys_and_data(
        self: @DeclareType, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
        self.type_def.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeclareType> {
        DeclareType { id: *keys.pop_front()?, type_def: ISerde::ideserialize(ref data)? }
            .verify(ref keys, ref data)
    }
}
