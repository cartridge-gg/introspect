#[derive(PrintStruct, Serde, Introspect)]
#[derive(Drop)]
struct TestStruct<T, S> {
    #[key]
    #[default]
    pub value: Span<T>,
    pub value2: (felt252, felt252),
}
