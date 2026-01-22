use crate::parser::DefaultParser;
use crate::utils::felt_to_utf8_string;
use crate::{
    Attribute, Bytes31EncodedDef, ElementDef, FeltIterator, Primary, PrimaryValue, Record,
    RecordValues, ToValue, TypeDef, felt_to_bytes31,
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

pub trait RecordParser {
    fn to_record(
        &self,
        schema: &TableSchema,
        primary: Felt,
        data: &mut FeltIterator,
    ) -> Option<Record>;
    fn to_record_values(
        &self,
        schema: &TableSchema,
        primary: Felt,
        data: &mut FeltIterator,
    ) -> Option<RecordValues>;
}

impl RecordParser for DefaultParser {
    fn to_record(
        &self,
        schema: &TableSchema,
        primary: Felt,
        data: &mut FeltIterator,
    ) -> Option<Record> {
        Some(Record {
            table_id: schema.id.clone(),
            table_name: schema.name.clone(),
            attributes: schema.attributes.clone(),
            primary: schema.primary.to_primary(primary)?,
            fields: self.to_value(&schema.columns, data)?,
        })
    }

    fn to_record_values(
        &self,
        schema: &TableSchema,
        primary: Felt,
        data: &mut FeltIterator,
    ) -> Option<RecordValues> {
        Some(RecordValues {
            primary: schema.primary.type_def.to_primary_value(primary)?,
            fields: schema
                .columns
                .iter()
                .map(|col| self.to_value(&col.type_def, data))
                .collect::<Option<Vec<_>>>()?,
        })
    }
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
    Bytes31Encoded(Bytes31EncodedDef),
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

impl ElementDef for PrimaryTypeDef {}
impl ElementDef for PrimaryDef {}

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

impl TryFrom<TypeDef> for PrimaryTypeDef {
    type Error = ();

    fn try_from(value: TypeDef) -> Result<Self, Self::Error> {
        match value {
            TypeDef::Felt252 => Ok(PrimaryTypeDef::Felt252),
            TypeDef::ShortUtf8 => Ok(PrimaryTypeDef::ShortUtf8),
            TypeDef::Bytes31 => Ok(PrimaryTypeDef::Bytes31),
            TypeDef::Bytes31Encoded(encoding) => Ok(PrimaryTypeDef::Bytes31Encoded(encoding)),
            TypeDef::Bool => Ok(PrimaryTypeDef::Bool),
            TypeDef::U8 => Ok(PrimaryTypeDef::U8),
            TypeDef::U16 => Ok(PrimaryTypeDef::U16),
            TypeDef::U32 => Ok(PrimaryTypeDef::U32),
            TypeDef::U64 => Ok(PrimaryTypeDef::U64),
            TypeDef::U128 => Ok(PrimaryTypeDef::U128),
            TypeDef::I8 => Ok(PrimaryTypeDef::I8),
            TypeDef::I16 => Ok(PrimaryTypeDef::I16),
            TypeDef::I32 => Ok(PrimaryTypeDef::I32),
            TypeDef::I64 => Ok(PrimaryTypeDef::I64),
            TypeDef::I128 => Ok(PrimaryTypeDef::I128),
            TypeDef::ClassHash => Ok(PrimaryTypeDef::ClassHash),
            TypeDef::ContractAddress => Ok(PrimaryTypeDef::ContractAddress),
            TypeDef::EthAddress => Ok(PrimaryTypeDef::EthAddress),
            TypeDef::StorageAddress => Ok(PrimaryTypeDef::StorageAddress),
            TypeDef::StorageBaseAddress => Ok(PrimaryTypeDef::StorageBaseAddress),
            _ => Err(()),
        }
    }
}

impl PrimaryTypeDef {
    pub fn item_name(&self) -> &str {
        match self {
            PrimaryTypeDef::Felt252 => "Felt252",
            PrimaryTypeDef::ShortUtf8 => "ShortUtf8",
            PrimaryTypeDef::Bytes31 => "Bytes31",
            PrimaryTypeDef::Bytes31Encoded(_) => "Bytes31Encoded",
            PrimaryTypeDef::Bool => "Bool",
            PrimaryTypeDef::U8 => "U8",
            PrimaryTypeDef::U16 => "U16",
            PrimaryTypeDef::U32 => "U32",
            PrimaryTypeDef::U64 => "U64",
            PrimaryTypeDef::U128 => "U128",
            PrimaryTypeDef::I8 => "I8",
            PrimaryTypeDef::I16 => "I16",
            PrimaryTypeDef::I32 => "I32",
            PrimaryTypeDef::I64 => "I64",
            PrimaryTypeDef::I128 => "I128",
            PrimaryTypeDef::ClassHash => "ClassHash",
            PrimaryTypeDef::ContractAddress => "ContractAddress",
            PrimaryTypeDef::EthAddress => "EthAddress",
            PrimaryTypeDef::StorageAddress => "StorageAddress",
            PrimaryTypeDef::StorageBaseAddress => "StorageBaseAddress",
        }
    }

    pub fn to_primary_value(&self, felt: Felt) -> Option<PrimaryValue> {
        match self {
            PrimaryTypeDef::Felt252 => Some(PrimaryValue::Felt252(felt)),
            PrimaryTypeDef::ShortUtf8 => felt_to_utf8_string(felt).map(PrimaryValue::ShortUtf8),
            PrimaryTypeDef::Bytes31 => felt_to_bytes31(felt).map(PrimaryValue::Bytes31),
            PrimaryTypeDef::Bytes31Encoded(e) => e
                .to_encoded_bytes_31(felt)
                .map(PrimaryValue::Bytes31Encoded),
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
