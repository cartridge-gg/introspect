use starknet::Event;
use crate::emit_event_impl;
use crate::utils::{DrainSpanTrait, VerifyEventDeserializeTrait};

pub mod selectors {
    pub const MultiPartEventBegin: felt252 = selector!("MultiPartEventBegin");
    pub const MultiPartEvent: felt252 = selector!("MultiPartEvent");
    pub const MultiPartEventEnd: felt252 = selector!("MultiPartEventEnd");
}

#[derive(Drop, Serde)]
pub struct MultiPartEventBegin {
    pub event_selector: felt252,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde)]
pub struct MultiPartEvent {
    pub data: Span<felt252>,
}

#[derive(Drop, Serde)]
pub struct MultiPartEventEnd {
    pub data: Span<felt252>,
}

impl CreateFieldSetEvent of Event<MultiPartEventBegin> {
    fn append_keys_and_data(
        self: @MultiPartEventBegin, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        data.append(*self.event_selector);
        data.append_span(*self.data)
    }

    fn deserialize(
        ref keys: Span<felt252>, ref data: Span<felt252>,
    ) -> Option<MultiPartEventBegin> {
        let event_selector = *data.pop_front()?;
        let message_data = data.drain();
        MultiPartEventBegin { event_selector, data: message_data }.verify_keys(ref keys)
    }
}

impl EmitMultiPartEventBegin =
    emit_event_impl::EmitEventImpl<MultiPartEventBegin, selectors::MultiPartEventBegin>;
