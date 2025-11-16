use crate::Attribute;
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
    pub encoding: Felt,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Record {
    pub table_id: Felt,
    pub table_name: String,
    pub attributes: Vec<Attribute>,
    pub primary: Primary,
    pub fields: Vec<Field>,
}

// pub trait ToPrimitiveString {
//     fn to_primitive_string(&self) -> Option<String>;
// }

// impl ToPrimitiveString for Enum {
//     fn to_primitive_string(&self) -> Option<String> {
//         let value = self.value.to_primitive_string()?.to_case(Case::Snake);
//         Some(format!("{}-{}", self.variant, value))
//     }
// }

// impl ToPrimitiveString for CairoOption<Value> {
//     fn to_primitive_string(&self) -> Option<String> {
//         match self {
//             CairoOption::Some(v) => Some(format!("some-{}", v.to_primitive_string()?)),
//             CairoOption::None => Some("none".to_string()),
//         }
//     }
// }

// impl ToPrimitiveString for CairoResult<Value, Value> {
//     fn to_primitive_string(&self) -> Option<String> {
//         match self {
//             CairoResult::Ok(v) => Some(format!("ok-{}", v.to_primitive_string()?)),
//             CairoResult::Err(e) => Some(format!("err-{}", e.to_primitive_string()?)),
//         }
//     }
// }

// impl ToPrimitiveString for Nullable {
//     fn to_primitive_string(&self) -> Option<String> {
//         match self {
//             Nullable::Null => Some("null".to_string()),
//             Nullable::NotNull(v) => Some(format!("not_null-{}", v.to_primitive_string()?)),
//         }
//     }
// }

// impl ToPrimitiveString for Value {
//     fn to_primitive_string(&self) -> Option<String> {
//         match self {
//             Value::Felt252(value)
//             | Value::ClassHash(value)
//             | Value::ContractAddress(value)
//             | Value::EthAddress(value)
//             | Value::StorageAddress(value)
//             | Value::StorageBaseAddress(value) => Some(felt_to_hex_string(value)),
//             Value::ShortUtf8(value) | Value::Utf8Array(value) => Some(value.clone()),
//             Value::Bytes31(value) => Some(bytes31_to_hex_string(value)),
//             Value::Bool(value) => Some(value.to_string()),
//             Value::U8(value) => Some(value.to_string()),
//             Value::U16(value) => Some(value.to_string()),
//             Value::U32(value) => Some(value.to_string()),
//             Value::U64(value) => Some(value.to_string()),
//             Value::U128(value) => Some(value.to_string()),
//             Value::U256(value) => Some(value.to_string()),
//             Value::U512(value) => Some(value.to_string()),
//             Value::I8(value) => Some(value.to_string()),
//             Value::I16(value) => Some(value.to_string()),
//             Value::I32(value) => Some(value.to_string()),
//             Value::I64(value) => Some(value.to_string()),
//             Value::I128(value) => Some(value.to_string()),
//             Value::Enum(v) => v.to_primitive_string(),
//             Value::Option(v) => v.to_primitive_string(),
//             Value::Result(v) => v.to_primitive_string(),
//             Value::Nullable(v) => v.to_primitive_string(),
//             _ => None,
//         }
//     }
// }
