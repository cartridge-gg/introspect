use introspect_types::ISerde;



#[generate_trait]
pub impl DrainSpanImpl<T, +Drop<T>> of DrainSpanTrait<T> {
    fn drain(ref self: Span<T>) -> Span<T> {
        let data = self;
        self = array![].span();
        data
    }
}

#[generate_trait]
pub impl VerifyEventDeserializeImpl<T, +Drop<T>> of VerifyEventDeserializeTrait<T> {
    fn verify(self: T, ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<T> {
        match (keys.pop_front(), data.pop_front()) {
            (None, None) => { Some(self) },
            _ => None,
        }
    }

    fn verify_keys(self: T, ref keys: Span<felt252>) -> Option<T> {
        match keys.pop_front() {
            None => { Some(self) },
            _ => None,
        }
    }
}


pub trait ISerdeEnd<T> {
    fn ideserialize_end(ref serialised: Span<felt252>) -> Option<Span<T>>;
    fn iserialize_end(self: @Span<T>, ref output: Array<felt252>);
}

impl DrainItemImpl<T, +ISerde<T>, +Drop<T>> of ISerdeEnd<T> {
    fn ideserialize_end(ref serialised: Span<felt252>) -> Option<Span<T>> {
        let mut items: Array<T> = Default::default();
        while !serialised.is_empty() {
            items.append(ISerde::ideserialize(ref serialised)?);
        }
        Some(items.span())
    }

    fn iserialize_end(self: @Span<T>, ref output: Array<felt252>) {
        for item in *self {
            item.iserialize(ref output);
        }
    }
}
