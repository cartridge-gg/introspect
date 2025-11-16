use crate::{Attribute, PrimaryDef};
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
    Utf8Array(String),
    ByteArrayE(EncodedBytes),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
    FixedArray(Vec<Value>),
    Felt252Dict(Vec<(Felt, Value)>),
    Struct(Struct),
    Enum(Box<Enum>),
    Custom(Custom),
    Option(Box<Option<Value>>),
    Result(Box<Result<Value, Value>>),
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
pub enum Nullable {
    Null,
    NotNull(Value),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Custom {
    pub name: Felt,
    pub values: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EncodedBytes {
    pub encoding: Felt,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Field {
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Record {
    pub table_id: Felt,
    pub table_name: String,
    pub attributes: Vec<Attribute>,
    pub primary: PrimaryDef,
    pub fields: Vec<Field>,
}
