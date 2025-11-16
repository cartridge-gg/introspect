use crate::utils::felt_to_utf8_string;
use crate::{
    Attribute, EncodedBytes, FeltIterator, Primary, PrimaryValue, Record, ToValue, TypeDef,
    felt_to_bytes31,
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

impl TableSchema {
    pub fn to_record(&self, primary: Felt, data: &mut FeltIterator) -> Option<Record> {
        Some(Record {
            table_id: self.id.clone(),
            table_name: self.name.clone(),
            attributes: self.attributes.clone(),
            primary: self.primary.to_primary(primary)?,
            fields: self.columns.to_value(data)?,
        })
    }
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

impl From<&ColumnDef> for ColumnInfo {
    fn from(field_def: &ColumnDef) -> Self {
        ColumnInfo {
            id: field_def.id.clone(),
            name: field_def.name.clone(),
            attributes: field_def.attributes.clone(),
        }
    }
}

impl PrimaryTypeDef {
    fn to_value(&self, felt: Felt) -> Option<PrimaryValue> {
        match self {
            PrimaryTypeDef::Felt252 => Some(PrimaryValue::Felt252(felt)),
            PrimaryTypeDef::ShortUtf8 => felt_to_utf8_string(felt).map(PrimaryValue::ShortUtf8),
            PrimaryTypeDef::Bytes31 => felt_to_bytes31(felt).map(PrimaryValue::Bytes31),
            PrimaryTypeDef::Bytes31E(encoding) => felt_to_bytes31(felt).map(|bytes| {
                PrimaryValue::Bytes31E(EncodedBytes {
                    encoding: *encoding,
                    bytes: bytes.into(),
                })
            }),
            PrimaryTypeDef::Bool => Some(PrimaryValue::Bool(!felt.is_zero())),
            PrimaryTypeDef::U8 => felt.try_into().ok().map(PrimaryValue::U8),
            PrimaryTypeDef::U16 => felt.try_into().ok().map(PrimaryValue::U16),
            PrimaryTypeDef::U32 => felt.try_into().ok().map(PrimaryValue::U32),
            PrimaryTypeDef::U64 => felt.try_into().ok().map(PrimaryValue::U64),
            PrimaryTypeDef::U128 => felt.try_into().ok().map(PrimaryValue::U128),
            PrimaryTypeDef::I8 => felt.try_into().ok().map(PrimaryValue::I8),
            PrimaryTypeDef::I16 => felt.try_into().ok().map(PrimaryValue::I16),
            PrimaryTypeDef::I32 => felt.try_into().ok().map(PrimaryValue::I32),
            PrimaryTypeDef::I64 => felt.try_into().ok().map(PrimaryValue::I64),
            PrimaryTypeDef::I128 => felt.try_into().ok().map(PrimaryValue::I128),
            PrimaryTypeDef::ClassHash => Some(PrimaryValue::ClassHash(felt)),
            PrimaryTypeDef::ContractAddress => Some(PrimaryValue::ContractAddress(felt)),
            PrimaryTypeDef::EthAddress => Some(PrimaryValue::EthAddress(felt)),
            PrimaryTypeDef::StorageAddress => Some(PrimaryValue::StorageAddress(felt)),
            PrimaryTypeDef::StorageBaseAddress => Some(PrimaryValue::StorageBaseAddress(felt)),
        }
    }
}

impl PrimaryDef {
    fn to_primary(&self, felt: Felt) -> Option<Primary> {
        Some(Primary {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.to_value(felt)?,
        })
    }
}

impl ColumnDef {
    pub fn new(id: Felt, name: String, attributes: Vec<Attribute>, type_def: TypeDef) -> Self {
        ColumnDef {
            id,
            name,
            attributes,
            type_def,
        }
    }
}
