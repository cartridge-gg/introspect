use starknet::core::types::Felt;
pub enum Parsed {
    None,
    Felt252(Felt),
    Bool(bool),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Uint256(U256),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    USize(u64),
    ShortString(String),
    ClassHash(Felt),
    ContractAddress(Felt),
    EthAddress(Felt),
    ByteArray(String),
    Tuple(Vec<Parsed>),
    Array(Box<Parsed>),
    FixedArray(Vec<Parsed>),
    Felt252Dict(Vec<(Felt, Parsed)>),
    Struct(Struct),
    Enum(Enum),
    Schema(String),
    Custom(Vec<Felt>),
    Option(Box<ParsedOption>),
    Result(Box<ParsedResult>),
    Nullable(Box<Nullable>),
    Encoding(Encoded),
    DynamicEncoding(Encoded),
}

pub struct U256 {
    pub low: u128,
    pub high: u128,
}

pub struct Member {
    pub name: String,
    pub attrs: Vec<String>,
    pub value: Parsed,
}

pub struct Struct {
    pub name: String,
    pub attrs: Vec<String>,
    pub children: Vec<Member>,
}
pub struct Enum {
    pub name: String,
    pub attrs: Vec<String>,
    pub children: Vec<Field>,
}

pub struct Field {
    pub selector: Felt,
    pub name: String,
    pub attrs: Vec<String>,
    pub ty: Parsed,
}

pub type ParsedOption = Option<Parsed>;
pub type ParsedResult = Result<Parsed, Parsed>;

pub enum Nullable {
    Null,
    NotNull(Parsed),
}

pub struct Encoded {
    pub encoding: String,
    pub value: Vec<u8>,
}
