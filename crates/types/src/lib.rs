pub mod attribute;
pub mod deserialize;
pub mod parser;
pub mod schema;
pub mod type_def;
pub mod utils;
pub mod value;
pub use attribute::Attribute;
pub use deserialize::CairoDeserialize;
pub use parser::ToValue;
pub use schema::{ColumnDef, ColumnInfo, Field};
pub use type_def::{EnumDef, FixedArrayDef, MemberDef, ResultDef, StructDef, TypeDef, VariantDef};
pub use utils::{
    FeltIterator, deserialize_byte_array, deserialize_byte_array_string, pop_primitive,
    pop_short_utf8, pop_u256, pop_u512, read_serialized_felt_array,
};
pub use value::{Custom, EncodedBytes, Enum, Member, Nullable, Struct, Value};
// use cainome_cairo_serde::{ByteArray, Bytes31};
// use introspect_value::{Custom, Enum, FeltIterator, Field, Struct, ToValue, Value};
// use num_traits::Zero;
// use primitive_types::U256;
// use serde::{Deserialize, Serialize};
// use starknet_types_core::felt::Felt;
// use std::collections::HashMap;

// #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
// pub enum TypeDef {
//     #[default]
//     None,
//     Felt252,
//     Bytes31,
//     Bool,
//     U8,
//     U16,
//     U32,
//     U64,
//     U128,
//     U256,
//     U521,
//     I8,
//     I16,
//     I32,
//     I64,
//     I128,
//     ClassHash,
//     ContractAddress,
//     EthAddress,
//     StorageAddress,
//     StorageBaseAddress,
//     ByteArray,
//     ShortString,
//     Tuple(Vec<TypeDef>),
//     Array(Box<TypeDef>),
//     FixedArray(Box<FixedArrayDef>),
//     Felt252Dict(Box<TypeDef>),
//     Struct(StructDef),
//     Enum(EnumDef),
//     Option(Box<TypeDef>),
//     Result(ResultDef),
//     Nullable(Box<TypeDef>),
//     Ref(Felt),
//     Custom(Felt),
// }

// pub trait TypeName {
//     fn type_name(&self) -> String;
// }

// impl TypeName for TypeDef {
//     fn type_name(&self) -> String {
//         match self {
//             TypeDef::None => "None".to_string(),
//             TypeDef::Felt252 => "Felt252".to_string(),
//             TypeDef::Bytes31 => "bytes31".to_string(),
//             TypeDef::Bool => "bool".to_string(),
//             TypeDef::U8 => "u8".to_string(),
//             TypeDef::U16 => "u16".to_string(),
//             TypeDef::U32 => "u32".to_string(),
//             TypeDef::U64 => "u64".to_string(),
//             TypeDef::U128 => "u128".to_string(),
//             TypeDef::U256 => "u256".to_string(),
//             TypeDef::U521 => "u521".to_string(),
//             TypeDef::I8 => "i8".to_string(),
//             TypeDef::I16 => "i16".to_string(),
//             TypeDef::I32 => "i32".to_string(),
//             TypeDef::I64 => "i64".to_string(),
//             TypeDef::I128 => "i128".to_string(),
//             TypeDef::ShortString => "ShortString".to_string(),
//             TypeDef::ClassHash => "ClassHash".to_string(),
//             TypeDef::ContractAddress => "ContractAddress".to_string(),
//             TypeDef::EthAddress => "EthAddress".to_string(),
//             TypeDef::StorageAddress => "StorageAddress".to_string(),
//             TypeDef::StorageBaseAddress => "StorageBaseAddress".to_string(),
//             TypeDef::ByteArray => "ByteArray".to_string(),
//             TypeDef::Tuple(inner) => format!(
//                 "({})",
//                 inner
//                     .iter()
//                     .map(|e| e.type_name())
//                     .collect::<Vec<String>>()
//                     .join(", ")
//             ),
//             TypeDef::Array(inner) => format!("Vec<{}>", inner.type_name()),
//             TypeDef::FixedArray(inner) => {
//                 format!("[{}; {}]", inner.type_def.type_name(), inner.size)
//             }
//             TypeDef::Felt252Dict(inner) => format!("Felt252Dict<{}>", inner.type_name()),
//             TypeDef::Struct(s) => s.name.clone(),
//             TypeDef::Enum(e) => e.name.clone(),

//             TypeDef::Option(inner) => format!("Option<{}>", inner.type_name()),
//             TypeDef::Result(inner) => format!(
//                 "Result<{}, {}>",
//                 inner.ok.type_name(),
//                 inner.err.type_name()
//             ),
//             TypeDef::Nullable(inner) => format!("Nullable<{}>", inner.type_name()),
//             TypeDef::Ref(name) => name.to_hex_string(),
//             TypeDef::Custom(name) => name.to_hex_string(),
//         }
//     }
// }
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct Attribute {
//     pub id: Felt,
//     pub data: Vec<Felt>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct ColumnDef {
//     pub id: Felt,
//     pub name: String,
//     pub attributes: Vec<Attribute>,
//     pub type_def: TypeDef,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct FieldInfo {
//     pub id: Felt,
//     pub name: String,
//     pub attributes: Vec<Attribute>,
// }

// impl From<&ColumnDef> for FieldInfo {
//     fn from(field_def: &ColumnDef) -> Self {
//         FieldInfo {
//             id: field_def.id.clone(),
//             name: field_def.name.clone(),
//             attributes: field_def.attributes.clone(),
//         }
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct VariantDef {
//     pub name: String,
//     pub attributes: Vec<Attribute>,
//     pub type_def: TypeDef,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct StructDef {
//     pub name: String,
//     pub attributes: Vec<Attribute>,
//     pub fields: Vec<FieldDef>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct EnumDef {
//     pub name: String,
//     pub attributes: Vec<Attribute>,
//     pub variants: HashMap<Felt, VariantDef>,
//     pub order: Vec<Felt>,
// }

// impl PartialEq for EnumDef {
//     fn eq(&self, other: &Self) -> bool {
//         let is_eq = self.name == other.name && self.attributes == other.attributes;
//         if !is_eq {
//             return false;
//         }

//         self.variants
//             .iter()
//             .all(|(k, v)| other.variants.get(k).map(|ov| v == ov).unwrap_or(false))
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct FixedArrayDef {
//     pub type_def: TypeDef,
//     pub size: u32,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct FieldDef {
//     pub name: String,
//     pub attributes: Vec<Attribute>,
//     pub type_def: TypeDef,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct ResultDef {
//     pub ok: TypeDef,
//     pub err: TypeDef,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct TypeDefVec(pub Vec<TypeDef>);

// impl std::ops::Deref for TypeDefVec {
//     type Target = Vec<TypeDef>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl std::ops::DerefMut for TypeDefVec {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
