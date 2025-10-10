use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;
use std::collections::VecDeque;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum Value {
    #[default]
    None,
    Felt252(Felt),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256(U256),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    USize(u64),
    ShortString(String),
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct U256 {
    pub low: u128,
    pub high: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub attrs: Vec<String>,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub attrs: Vec<String>,
    pub members: Vec<Member>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub attrs: Vec<String>,
    pub variant: String,
    pub variant_attrs: Vec<String>,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    pub selector: Felt,
    pub name: String,
    pub attrs: Vec<String>,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Nullable {
    Null,
    NotNull(Value),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Encoded {
    pub encoding: String,
    pub value: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
