pub trait Spannable<From, Inner> {
    fn to_span(self: From) -> Span<Inner>;
}


impl OwnedToSpan<From, Inner, +ToSpanTrait<From, Inner>, +Drop<From>> of Spannable<From, Inner> {
    fn to_span(self: From) -> Span<Inner> {
        self.span()
    }
}

impl SnapshotToSpan<From, Inner, +ToSpanTrait<From, Inner>> of Spannable<@From, Inner> {
    fn to_span(self: @From) -> Span<Inner> {
        self.span()
    }
}


impl SpanToSpan<T> of Spannable<Span<T>, T> {
    fn to_span(self: Span<T>) -> Span<T> {
        self
    }
}

impl SpanSnapshotToSpan<T> of Spannable<@Span<T>, T> {
    fn to_span(self: @Span<T>) -> Span<T> {
        *self
    }
}
