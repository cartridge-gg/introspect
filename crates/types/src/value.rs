use crate::{Attribute, bytes31_to_hex_string, felt_to_hex_string};
use primitive_types::{U256, U512};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum Value {
    #[default]
    None,
    Felt252(Felt),
    ShortUtf8(String),
    Bytes31([u8; 31]),
    Bytes31E(EncodedBytes),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256(U256),
    U512(U512),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ClassHash(Felt),
    ContractAddress(Felt),
    EthAddress(Felt),
    StorageAddress(Felt),
    StorageBaseAddress(Felt),
    ByteArray(Vec<u8>),
    Utf8String(String),
    ByteArrayE(EncodedBytes),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
    FixedArray(Vec<Value>),
    Felt252Dict(Vec<(Felt, Value)>),
    Struct(Struct),
    Enum(Box<Enum>),
    Custom(Custom),
    Option(Box<CairoOption<Value>>),
    Result(Box<CairoResult<Value, Value>>),
    Nullable(Box<Nullable>),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PrimaryValue {
    Felt252(Felt),
    ShortUtf8(String),
    Bytes31([u8; 31]),
    Bytes31E(EncodedBytes),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ClassHash(Felt),
    ContractAddress(Felt),
    EthAddress(Felt),
    StorageAddress(Felt),
    StorageBaseAddress(Felt),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Struct {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub members: Vec<Member>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Member {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Enum {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub variant: String,
    pub variant_attributes: Vec<Attribute>,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CairoOption<T> {
    Some(T),
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CairoResult<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Nullable {
    Null,
    NotNull(Value),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Custom {
    pub id: Felt,
    pub values: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EncodedBytes {
    pub encoding: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Field {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub value: Value,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Primary {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub value: PrimaryValue,
}

impl ToString for PrimaryValue {
    fn to_string(&self) -> String {
        match self {
            PrimaryValue::Felt252(value)
            | PrimaryValue::ClassHash(value)
            | PrimaryValue::ContractAddress(value)
            | PrimaryValue::EthAddress(value)
            | PrimaryValue::StorageAddress(value)
            | PrimaryValue::StorageBaseAddress(value) => felt_to_hex_string(value),
            PrimaryValue::Bytes31(value) => bytes31_to_hex_string(value),
            PrimaryValue::Bytes31E(value) => bytes31_to_hex_string(&value.bytes),
            PrimaryValue::ShortUtf8(value) => value.clone(),
            PrimaryValue::Bool(value) => value.to_string(),
            PrimaryValue::U8(value) => value.to_string(),
            PrimaryValue::U16(value) => value.to_string(),
            PrimaryValue::U32(value) => value.to_string(),
            PrimaryValue::U64(value) => value.to_string(),
            PrimaryValue::U128(value) => value.to_string(),
            PrimaryValue::I8(value) => value.to_string(),
            PrimaryValue::I16(value) => value.to_string(),
            PrimaryValue::I32(value) => value.to_string(),
            PrimaryValue::I64(value) => value.to_string(),
            PrimaryValue::I128(value) => value.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Record {
    pub table_id: Felt,
    pub table_name: String,
    pub attributes: Vec<Attribute>,
    pub primary: Primary,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RecordValues {
    pub primary: PrimaryValue,
    pub fields: Vec<Value>,
}
