

// This the top-level type for a schema 
enum SchemaTy {
    Struct: Struct,
    Enum: Enum,
}

pub struct Struct {
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub children: Span<Member>,
}

pub struct Enum {
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub children: Span<(felt252, Ty)>,
}

pub struct Member {
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub ty: Ty,
}

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
    Schema: felt252,
    Tuple: Span<Ty>,
    Array: Wrapper<Ty>,
    FixedArray: (Wrapper<Ty>, u32),

}