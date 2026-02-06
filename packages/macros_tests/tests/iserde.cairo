#[derive(Drop, ISerde, TypeDef)]
struct MyStruct<T> {
    a: Array<Span<felt252>>,
    b: Option<felt252>,
    c: ByteArray,
    d: (u8, u16, T),
}

#[derive(Drop, ISerde, TypeDef)]
struct MyStruct0 {}

#[derive(Drop, ISerde, TypeDef)]
struct MyStruct1 {
    a: Array<Span<felt252>>,
}
#[derive(Drop, ISerde, TypeDef)]
struct MyStruct2 {
    a: Array<Span<felt252>>,
    b: Option<felt252>,
}


#[derive(Drop, ISerde, TypeDefRef)]
enum MyEnum<T, S> {
    Variant1,
    Variant2: T,
    Variant3: (felt252, felt252, Option<S>),
}

#[derive(Drop, ISerde, TypeDefRef)]
enum MyEnum1 {
    Variant1,
}

#[derive(Drop, ISerde, TypeDefRef)]
enum MyEnum2 {
    Variant1,
    Variant2,
}

