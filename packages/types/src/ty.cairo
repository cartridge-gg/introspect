use wrapper::Wrapper;


#[derive(Drop, Copy, Serde, PartialEq)]
pub struct Struct {
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub children: Span<Member>,
}

#[derive(Drop, Copy, Serde, PartialEq)]
pub struct Enum {
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub children: Span<(felt252, Ty)>,
}

#[derive(Drop, Copy, Serde, PartialEq)]
pub struct Member {
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub ty: Ty,
}

#[derive(Drop, Copy, Serde, PartialEq, Default)]
pub enum Ty {
    #[default]
    None,
    Felt252,
    Bool,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Uint256,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    ShortString,
    ClassHash,
    ContractAddress,
    EthAddress,
    ByteArray,
    Tuple: Span<Ty>,
    Array: Wrapper<Ty>,
    FixedArray: (Wrapper<Ty>, u32),
    Custom: felt252,
    Struct: Struct,
    Enum: Enum,
}
