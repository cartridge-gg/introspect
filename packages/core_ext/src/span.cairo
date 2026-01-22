pub trait ToSpan<From, Inner> {
    fn to_span(self: From) -> Span<Inner>;
}


impl OwnedToSpan<From, Inner, +ToSpanTrait<From, Inner>, +Drop<From>> of ToSpan<From, Inner> {
    fn to_span(self: From) -> Span<Inner> {
        self.span()
    }
}

impl SnapshotToSpan<From, Inner, +ToSpanTrait<From, Inner>> of ToSpan<@From, Inner> {
    fn to_span(self: @From) -> Span<Inner> {
        self.span()
    }
}


impl SpanToSpan<T> of ToSpan<Span<T>, T> {
    fn to_span(self: Span<T>) -> Span<T> {
        self
    }
}

impl SpanSnapshotToSpan<T> of ToSpan<@Span<T>, T> {
    fn to_span(self: @Span<T>) -> Span<T> {
        *self
    }
}
impl Span2SnapshotToSpan<T, S, impl TS: ToSpan<@T, S>> of ToSpan<@@T, S> {
    fn to_span(self: @@T) -> Span<S> {
        TS::to_span(*self)
    }
}

