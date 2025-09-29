use starknet::core::types::Felt;

struct U256 {
    low: u128,
    high: u128,
}

pub struct Member {
    name: String,
    attrs: Vec<String>,
    value: Parsed,
}

pub struct Struct {
    name: String,
    attrs: Vec<String>,
    children: Vec<Member>,
}
pub struct Enum {
    name: String,
    attrs: Vec<String>,
    children: Vec<Field>,
}

pub struct Field {
    pub selector: Felt,
    pub name: String,
    pub attrs: Vec<String>,
    pub ty: Parsed,
}

pub struct OptionParsed {
    is_some: bool,
    value: Box<Parsed>,
}

pub struct ParsedParsed {
    is_ok: bool,
    value: Box<Parsed>,
}
pub enum Nullable {
    Null,
    NotNull(Box<Parsed>),
}

pub struct EncodingParsed {
    encoding: String,
    value: Vec<String>,
}

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
    Custom(String),
    Option(Option<Box<Parsed>>),
    Result(Result<Box<Parsed>, Box<Parsed>>),
    Nullable(Nullable),
    Function(Function),
    Encoding(EncodingParsed),
}
