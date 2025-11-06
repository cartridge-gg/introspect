#[derive(Introspect)]
struct TestStruct<T, S> {
    #[key]
    #[default]
    pub value: Span<T>,
    pub value2: (felt252, S),
}


#[derive(Introspect)]
enum TestEnum {
    Variant1,
    Variant2: TestStruct<felt252, felt252>,
}
