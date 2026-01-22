use introspect_types::{ChildDef, ISerde, TypeDef};
use starknet::Event;
use crate::emit_event_impl;
use crate::utils::{DrainSpanTrait, VerifyEventDeserializeTrait};

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
    pub id: felt252,
    pub type_def: TypeDef,
}


impl DeclareTypeEvent of Event<DeclareType> {
    fn append_keys_and_data(
        self: @DeclareType, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.id);
        self.type_def.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeclareType> {
        DeclareType { id: *data.pop_front()?, type_def: ISerde::ideserialize(ref data)? }
            .verify(ref keys, ref data)
    }
}

impl DeclareTypeEventSerialized of Event<ChildDef> {
    fn append_keys_and_data(self: @ChildDef, ref keys: Array<felt252>, ref data: Array<felt252>) {
        data.append(*self.id);
        data.append_span(*self.type_def)
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<ChildDef> {
        ChildDef { id: *data.pop_front()?, type_def: data.drain() }.verify(ref keys, ref data)
    }
}

impl EmitDeclareType = emit_event_impl::EmitEventImpl<DeclareType, selectors::DeclareType>;
impl EmitDeclareTypeSerialized = emit_event_impl::EmitEventImpl<ChildDef, selectors::DeclareType>;
