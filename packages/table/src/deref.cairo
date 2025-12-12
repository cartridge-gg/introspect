pub trait Snapable<T, S> {
    fn snapshot(self: T) -> @S;
}


impl TSnapable<T, +Drop<T>> of Snapable<T, T> {
    fn snapshot(self: T) -> @T {
        @self
    }
}

impl TSSSnapable<T> of Snapable<@T, T> {
    fn snapshot(self: @T) -> @T {
        self
    }
}

impl TSSSSnapable<T, impl SS: Snapable<@T, T>> of Snapable<@@T, T> {
    fn snapshot(self: @@T) -> @T {
        SS::snapshot(*self)
    }
}


pub trait Spannable<C, T> {
    fn to_span(self: C) -> Span<T>;
}


impl SpannableImpl<C, T, +ToSpanTrait<C, T>, +Drop<C>> of Spannable<C, T> {
    fn to_span(self: C) -> Span<T> {
        self.span()
    }
}

impl SpannableSpan<T> of Spannable<Span<T>, T> {
    fn to_span(self: Span<T>) -> Span<T> {
        self
    }
}

impl AFixedArrayToSpan<
    T, const N: usize, impl TS: ToSpanTrait<[T; N], T>,
> of Spannable<@[T; N], T> {
    fn to_span(self: @[T; N]) -> Span<T> {
        TS::span(self)
    }
}
