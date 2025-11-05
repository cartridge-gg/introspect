#[introspect]
#[derive(PrintAll)]
struct TestStruct {
    #[encoding("utf-8")]
    pub value: felt252,
}