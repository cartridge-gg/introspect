use crate::utils::felt_to_utf8_string;
use crate::{
    Attribute, EncodedBytes, FeltIterator, Primary, PrimaryValue, Record, RecordValues, ToValue,
    TypeDef, felt_to_bytes31,
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
    pub fn new(
        id: Felt,
        name: String,
        attributes: Vec<Attribute>,
        primary: PrimaryDef,
        columns: Vec<ColumnDef>,
    ) -> Self {
        TableSchema {
            id,
            name,
            attributes,
            primary,
            columns,
        }
    }
    pub fn to_record(&self, primary: Felt, data: &mut FeltIterator) -> Option<Record> {
        Some(Record {
            table_id: self.id.clone(),
            table_name: self.name.clone(),
            attributes: self.attributes.clone(),
            primary: self.primary.to_primary(primary)?,
            fields: self.columns.to_value(data)?,
        })
    }

    pub fn to_record_values(&self, primary: Felt, data: &mut FeltIterator) -> Option<RecordValues> {
        Some(RecordValues {
            primary: self.primary.type_def.to_primary_value(primary)?,
            fields: self
                .columns
                .iter()
                .map(|col| col.type_def.to_value(data))
                .collect::<Option<Vec<_>>>()?,
        })
    }

    pub fn to_schema_info(&self) -> SchemaInfo {
        SchemaInfo {
            table_id: self.id.clone(),
            table_name: self.name.clone(),
            attributes: self.attributes.clone(),
            primary: PrimaryInfo {
                name: self.primary.name.clone(),
                attributes: self.primary.attributes.clone(),
            },
            columns: self
                .columns
                .iter()
                .map(ColumnInfo::from)
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SchemaInfo {
    pub table_id: Felt,
    pub table_name: String,
    pub attributes: Vec<Attribute>,
    pub primary: PrimaryInfo,
    pub columns: Vec<ColumnInfo>,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrimaryInfo {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum PrimaryTypeDef {
    #[default]
    Felt252,
    ShortUtf8,
    Bytes31,
    Bytes31E(String),
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
    fn from(column_def: &ColumnDef) -> Self {
        ColumnInfo {
            id: column_def.id.clone(),
            name: column_def.name.clone(),
            attributes: column_def.attributes.clone(),
        }
    }
}

impl<T: AsRef<ColumnDef>> From<T> for ColumnInfo {
    fn from(column_def: T) -> Self {
        let column_def = column_def.as_ref();
        ColumnInfo {
            id: column_def.id.clone(),
            name: column_def.name.clone(),
            attributes: column_def.attributes.clone(),
        }
    }
}

impl PrimaryTypeDef {
    pub fn to_primary_value(&self, felt: Felt) -> Option<PrimaryValue> {
        match self {
            PrimaryTypeDef::Felt252 => Some(PrimaryValue::Felt252(felt)),
            PrimaryTypeDef::ShortUtf8 => felt_to_utf8_string(felt).map(PrimaryValue::ShortUtf8),
            PrimaryTypeDef::Bytes31 => felt_to_bytes31(felt).map(PrimaryValue::Bytes31),
            PrimaryTypeDef::Bytes31E(encoding) => felt_to_bytes31(felt).map(|bytes| {
                PrimaryValue::Bytes31E(EncodedBytes {
                    encoding: encoding.clone(),
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
    pub fn to_primary(&self, felt: Felt) -> Option<Primary> {
        Some(Primary {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.to_primary_value(felt)?,
        })
    }

    pub fn to_primary_value(&self, felt: Felt) -> Option<PrimaryValue> {
        self.type_def.to_primary_value(felt)
    }

    pub fn to_primary_info(&self) -> PrimaryInfo {
        PrimaryInfo {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
        }
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
