use crate::deserialize::CairoDeserialize;
use crate::parser::pop_bytes31_encoded;
use crate::type_def::selectors;
use crate::utils::pop_bytes31;
use crate::{
    Attribute, EncodedBytes, FeltIterator, ToValue, TypeDef, Value, deserialize_byte_array_string,
    pop_primitive, pop_short_utf8,
};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub primary: PrimaryDef,
    pub columns: Vec<ColumnDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ColumnDef {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ColumnInfo {
    pub id: Felt,
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrimaryDef {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: PrimaryTypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum PrimaryTypeDef {
    #[default]
    Felt252,
    ShortUtf8,
    Bytes31,
    Bytes31E(Felt),
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    ClassHash,
    ContractAddress,
    EthAddress,
    StorageAddress,
    StorageBaseAddress,
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
pub struct Field {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub value: Value,
}

impl From<&ColumnDef> for ColumnInfo {
    fn from(field_def: &ColumnDef) -> Self {
        ColumnInfo {
            id: field_def.id.clone(),
            name: field_def.name.clone(),
            attributes: field_def.attributes.clone(),
        }
    }
}

impl ToValue for PrimaryTypeDef {
    type Value = PrimaryValue;
    fn to_value(&self, data: &mut crate::FeltIterator) -> Option<Self::Value> {
        match self {
            PrimaryTypeDef::Felt252 => pop_primitive(data).map(PrimaryValue::Felt252),
            PrimaryTypeDef::ShortUtf8 => pop_short_utf8(data).map(PrimaryValue::ShortUtf8),
            PrimaryTypeDef::Bytes31 => pop_bytes31(data).map(PrimaryValue::Bytes31),
            PrimaryTypeDef::Bytes31E(encoding) => {
                pop_bytes31_encoded(*encoding, data).map(PrimaryValue::Bytes31E)
            }
            PrimaryTypeDef::Bool => data.next().map(|v| PrimaryValue::Bool(!v.is_zero())),
            PrimaryTypeDef::U8 => pop_primitive(data).map(PrimaryValue::U8),
            PrimaryTypeDef::U16 => pop_primitive(data).map(PrimaryValue::U16),
            PrimaryTypeDef::U32 => pop_primitive(data).map(PrimaryValue::U32),
            PrimaryTypeDef::U64 => pop_primitive(data).map(PrimaryValue::U64),
            PrimaryTypeDef::U128 => pop_primitive(data).map(PrimaryValue::U128),
            PrimaryTypeDef::I8 => pop_primitive(data).map(PrimaryValue::I8),
            PrimaryTypeDef::I16 => pop_primitive(data).map(PrimaryValue::I16),
            PrimaryTypeDef::I32 => pop_primitive(data).map(PrimaryValue::I32),
            PrimaryTypeDef::I64 => pop_primitive(data).map(PrimaryValue::I64),
            PrimaryTypeDef::I128 => pop_primitive(data).map(PrimaryValue::I128),
            PrimaryTypeDef::ClassHash => pop_primitive(data).map(PrimaryValue::ClassHash),
            PrimaryTypeDef::ContractAddress => data.next().map(PrimaryValue::ContractAddress),
            PrimaryTypeDef::EthAddress => data.next().map(PrimaryValue::EthAddress),
            PrimaryTypeDef::StorageAddress => pop_primitive(data).map(PrimaryValue::StorageAddress),
            PrimaryTypeDef::StorageBaseAddress => {
                pop_primitive(data).map(PrimaryValue::StorageBaseAddress)
            }
        }
    }
}

impl CairoDeserialize for ColumnDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let id = pop_primitive(data)?;
        let name = deserialize_byte_array_string(data)?;
        let attributes: Vec<Attribute> = Vec::<Attribute>::c_deserialize(data)?;
        let type_def: TypeDef = TypeDef::c_deserialize(data)?;
        Some(ColumnDef {
            id,
            name,
            attributes,
            type_def,
        })
    }
}

impl CairoDeserialize for PrimaryDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let name = deserialize_byte_array_string(data)?;
        let attributes: Vec<Attribute> = Vec::<Attribute>::c_deserialize(data)?;
        let type_def: PrimaryTypeDef = PrimaryTypeDef::c_deserialize(data)?;
        Some(PrimaryDef {
            name,
            attributes,
            type_def,
        })
    }
}

impl CairoDeserialize for PrimaryTypeDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let selector = data.next()?.to_raw();
        match selector {
            selectors::Felt252 => Some(PrimaryTypeDef::Felt252),
            selectors::ShortUtf8 => Some(PrimaryTypeDef::ShortUtf8),
            selectors::Bytes31 => Some(PrimaryTypeDef::Bytes31),
            selectors::Bytes31E => Some(PrimaryTypeDef::Bytes31E(data.next()?)),
            selectors::Bool => Some(PrimaryTypeDef::Bool),
            selectors::U8 => Some(PrimaryTypeDef::U8),
            selectors::U16 => Some(PrimaryTypeDef::U16),
            selectors::U32 => Some(PrimaryTypeDef::U32),
            selectors::U64 => Some(PrimaryTypeDef::U64),
            selectors::U128 => Some(PrimaryTypeDef::U128),
            selectors::I8 => Some(PrimaryTypeDef::I8),
            selectors::I16 => Some(PrimaryTypeDef::I16),
            selectors::I32 => Some(PrimaryTypeDef::I32),
            selectors::I64 => Some(PrimaryTypeDef::I64),
            selectors::I128 => Some(PrimaryTypeDef::I128),
            selectors::ClassHash => Some(PrimaryTypeDef::ClassHash),
            selectors::ContractAddress => Some(PrimaryTypeDef::ContractAddress),
            selectors::EthAddress => Some(PrimaryTypeDef::EthAddress),
            selectors::StorageAddress => Some(PrimaryTypeDef::StorageAddress),
            selectors::StorageBaseAddress => Some(PrimaryTypeDef::StorageBaseAddress),
            _ => None,
        }
    }
}
