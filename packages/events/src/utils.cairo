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
