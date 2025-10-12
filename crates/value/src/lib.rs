use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

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

impl ToString for U256 {
    fn to_string(&self) -> String {
        format!("0x{:016x}{:016x}", self.high, self.low)
    }
}

pub fn felt_to_string(value: &Felt) -> String {
    format!("0x{:016x}", value)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub attrs: Vec<String>,
    pub fields: Vec<Field>,
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

pub type FeltIterator = dyn Iterator<Item = Felt>;

pub trait ToValue {
    type Value;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Self::Value>;

    fn to_value_multiple(&self, data: &mut FeltIterator, count: usize) -> Option<Vec<Self::Value>> {
        (0..count)
            .into_iter()
            .map(|_| self.to_value(data))
            .collect()
    }
}

pub trait ToPrimitiveString {
    fn to_primitive_string(&self) -> Option<String>;
}

impl ToPrimitiveString for Enum {
    fn to_primitive_string(&self) -> Option<String> {
        let value = self.value.to_primitive_string()?.to_case(Case::Snake);
        Some(format!("{}-{}", self.variant, value))
    }
}

impl ToPrimitiveString for Option<Value> {
    fn to_primitive_string(&self) -> Option<String> {
        match self {
            Some(v) => Some(format!("some-{}", v.to_primitive_string()?)),
            None => Some("none".to_string()),
        }
    }
}

impl ToPrimitiveString for Nullable {
    fn to_primitive_string(&self) -> Option<String> {
        match self {
            Nullable::Null => Some("null".to_string()),
            Nullable::NotNull(v) => Some(format!("not_null-{}", v.to_primitive_string()?)),
        }
    }
}

impl ToPrimitiveString for Value {
    fn to_primitive_string(&self) -> Option<String> {
        match self {
            Value::Felt252(value)
            | Value::ClassHash(value)
            | Value::ContractAddress(value)
            | Value::EthAddress(value) => Some(felt_to_string(value)),
            Value::Bool(value) => Some(value.to_string()),
            Value::U8(value) => Some(value.to_string()),
            Value::U16(value) => Some(value.to_string()),
            Value::U32(value) => Some(value.to_string()),
            Value::U64(value) => Some(value.to_string()),
            Value::U128(value) => Some(value.to_string()),
            Value::U256(value) => Some(value.to_string()),
            Value::I8(value) => Some(value.to_string()),
            Value::I16(value) => Some(value.to_string()),
            Value::I32(value) => Some(value.to_string()),
            Value::I64(value) => Some(value.to_string()),
            Value::I128(value) => Some(value.to_string()),
            Value::ShortString(s) => Some(s.clone()),
            Value::ByteArray(s) => Some(s.clone()),
            Value::Enum(v) => v.to_primitive_string(),
            Value::Option(v) => v.to_primitive_string(),
            Value::Nullable(v) => v.to_primitive_string(),
            _ => None,
        }
    }
}
