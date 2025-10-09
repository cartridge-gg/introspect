use starknet_types_core::{felt::Felt, short_string::ShortString};
use std::collections::VecDeque;

pub enum Value {
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
    ShortString(ShortString),
    ClassHash(Felt),
    ContractAddress(Felt),
    EthAddress(Felt),
    ByteArray(String),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
    FixedArray(Vec<Value>),
    Felt252Dict(Vec<(Felt, Value)>),
    Struct(Struct),
    Enum(Box<Enum>),
    Schema(String),
    Custom(Custom),
    Option(Box<Option<Value>>),
    Result(Box<Result<Value, Value>>),
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
    pub value: Value,
}

pub struct Struct {
    pub name: String,
    pub attrs: Vec<String>,
    pub children: Vec<Member>,
}
pub struct Enum {
    pub name: String,
    pub attrs: Vec<String>,
    pub variant: String,
    pub variant_attrs: Vec<String>,
    pub value: Value,
}

pub struct Field {
    pub selector: Felt,
    pub name: String,
    pub attrs: Vec<String>,
    pub value: Value,
}

pub enum Nullable {
    Null,
    NotNull(Value),
}

pub struct Encoded {
    pub encoding: String,
    pub value: Vec<u8>,
}

pub struct Custom {
    pub name: String,
    pub values: Vec<Felt>,
}

pub trait ToValue {
    type Value;
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Self::Value>;
    fn to_value_multiple(
        &self,
        data: &mut VecDeque<Felt>,
        count: usize,
    ) -> Option<Vec<Self::Value>> {
        (0..count)
            .into_iter()
            .map(|_| self.to_value(data))
            .collect()
    }
}
