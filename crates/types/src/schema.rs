use crate::deserialize::{CairoDeserializer, FeltToPrimitive};
use crate::parser::{ParseValues, TypeParserResult};
use crate::{
    Attribute, Attributes, Bytes31EncodedDef, ElementDef, Primary, PrimaryValue, Record,
    ResultInto, TypeDef, felt_to_bytes31_bytes, felt_to_utf8_string,
};
use blake3::Hash;
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
pub struct TableInfo {
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

    pub fn to_record<D: CairoDeserializer>(
        &self,
        primary_id: Felt,
        data: &mut D,
    ) -> TypeParserResult<Record> {
        let primary = self.primary.to_primary(primary_id)?;
        let fields = self.columns.parse_values(data)?;
        Ok(Record {
            table_id: self.id.clone(),
            table_name: self.name.clone(),
            attributes: self.attributes.clone(),
            primary,
            fields,
        })
    }

    pub fn to_schema_info(&self) -> TableInfo {
        TableInfo {
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

// impl TryFrom<TypeDef> for PrimaryTypeDef {
//     type Error = ();

//     fn try_from(value: TypeDef) -> Result<Self, Self::Error> {
//         match value {
//             TypeDef::Felt252 => Ok(PrimaryTypeDef::Felt252),
//             TypeDef::ShortUtf8 => Ok(PrimaryTypeDef::ShortUtf8),
//             TypeDef::Bytes31 => Ok(PrimaryTypeDef::Bytes31),
//             TypeDef::Bytes31Encoded(encoding) => Ok(PrimaryTypeDef::Bytes31Encoded(encoding)),
//             TypeDef::Bool => Ok(PrimaryTypeDef::Bool),
//             TypeDef::U8 => Ok(PrimaryTypeDef::U8),
//             TypeDef::U16 => Ok(PrimaryTypeDef::U16),
//             TypeDef::U32 => Ok(PrimaryTypeDef::U32),
//             TypeDef::U64 => Ok(PrimaryTypeDef::U64),
//             TypeDef::U128 => Ok(PrimaryTypeDef::U128),
//             TypeDef::I8 => Ok(PrimaryTypeDef::I8),
//             TypeDef::I16 => Ok(PrimaryTypeDef::I16),
//             TypeDef::I32 => Ok(PrimaryTypeDef::I32),
//             TypeDef::I64 => Ok(PrimaryTypeDef::I64),
//             TypeDef::I128 => Ok(PrimaryTypeDef::I128),
//             TypeDef::ClassHash => Ok(PrimaryTypeDef::ClassHash),
//             TypeDef::ContractAddress => Ok(PrimaryTypeDef::ContractAddress),
//             TypeDef::EthAddress => Ok(PrimaryTypeDef::EthAddress),
//             TypeDef::StorageAddress => Ok(PrimaryTypeDef::StorageAddress),
//             TypeDef::StorageBaseAddress => Ok(PrimaryTypeDef::StorageBaseAddress),
//             _ => Err(()),
//         }
//     }
// }

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

    pub fn to_primary_value(&self, felt: Felt) -> TypeParserResult<PrimaryValue> {
        match self {
            PrimaryTypeDef::Felt252 => Ok(PrimaryValue::Felt252(felt)),
            PrimaryTypeDef::ShortUtf8 => {
                felt_to_utf8_string(felt).map_into(PrimaryValue::ShortUtf8)
            }
            PrimaryTypeDef::Bytes31 => felt_to_bytes31_bytes(felt).map_into(PrimaryValue::Bytes31),
            PrimaryTypeDef::Bytes31Encoded(e) => e
                .to_encoded_bytes_31(felt)
                .map(PrimaryValue::Bytes31Encoded),
            PrimaryTypeDef::Bool => Ok(PrimaryValue::Bool(!felt.is_zero())),
            PrimaryTypeDef::U8 => felt.to_primitive().map_into(PrimaryValue::U8),
            PrimaryTypeDef::U16 => felt.to_primitive().map_into(PrimaryValue::U16),
            PrimaryTypeDef::U32 => felt.to_primitive().map_into(PrimaryValue::U32),
            PrimaryTypeDef::U64 => felt.to_primitive().map_into(PrimaryValue::U64),
            PrimaryTypeDef::U128 => felt.to_primitive().map_into(PrimaryValue::U128),
            PrimaryTypeDef::I8 => felt.to_primitive().map_into(PrimaryValue::I8),
            PrimaryTypeDef::I16 => felt.to_primitive().map_into(PrimaryValue::I16),
            PrimaryTypeDef::I32 => felt.to_primitive().map_into(PrimaryValue::I32),
            PrimaryTypeDef::I64 => felt.to_primitive().map_into(PrimaryValue::I64),
            PrimaryTypeDef::I128 => felt.to_primitive().map_into(PrimaryValue::I128),
            PrimaryTypeDef::ClassHash => Ok(PrimaryValue::ClassHash(felt)),
            PrimaryTypeDef::ContractAddress => Ok(PrimaryValue::ContractAddress(felt)),
            PrimaryTypeDef::EthAddress => Ok(PrimaryValue::EthAddress(felt)),
            PrimaryTypeDef::StorageAddress => Ok(PrimaryValue::StorageAddress(felt)),
            PrimaryTypeDef::StorageBaseAddress => Ok(PrimaryValue::StorageBaseAddress(felt)),
        }
    }
}

impl PrimaryDef {
    pub fn new(name: String, attributes: Vec<Attribute>, type_def: PrimaryTypeDef) -> Self {
        PrimaryDef {
            name,
            attributes,
            type_def,
        }
    }
    pub fn to_primary(&self, felt: Felt) -> TypeParserResult<Primary> {
        Ok(Primary {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.to_primary_value(felt)?,
        })
    }

    pub fn to_primary_value(&self, felt: Felt) -> TypeParserResult<PrimaryValue> {
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

impl Attributes for TableSchema {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for TableInfo {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for ColumnDef {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for ColumnInfo {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for PrimaryDef {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for PrimaryInfo {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

pub trait FeltIds {
    fn ids(&self) -> Vec<Felt>;
    fn hash(&self) -> Hash {
        let ids = self.ids();
        match ids.len() {
            0 => Hash::from([0; 32]),
            1 => Hash::from(ids[0].to_bytes_be()),
            _ => blake3::hash(
                &self
                    .ids()
                    .into_iter()
                    .flat_map(|id| id.to_bytes_be())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

pub trait FeltId {
    fn id(&self) -> Felt;
}

impl<T: FeltId> FeltIds for Vec<T> {
    fn ids(&self) -> Vec<Felt> {
        self.iter().map(|item| item.id()).collect()
    }
}

impl<T: FeltId> FeltIds for &[T] {
    fn ids(&self) -> Vec<Felt> {
        self.iter().map(|item| item.id()).collect()
    }
}

impl FeltId for Felt {
    fn id(&self) -> Felt {
        *self
    }
}

impl FeltId for ColumnDef {
    fn id(&self) -> Felt {
        self.id.clone()
    }
}

impl FeltId for ColumnInfo {
    fn id(&self) -> Felt {
        self.id.clone()
    }
}
