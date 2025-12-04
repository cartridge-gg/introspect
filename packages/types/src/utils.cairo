pub impl SpanDefault<T, +Drop<T>> of Default<Span<T>> {
    fn default() -> Span<T> {
        array![].span()
    }
}
