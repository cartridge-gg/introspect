use crate::type_def::{FixedArrayDef, MemberDef, StructDef, TypeDef};
use crate::utils::{felt_to_string, pop_bytes31};
use crate::value::{Enum, Nullable, Value};
use crate::{
    ColumnDef, Custom, EncodedBytes, EnumDef, FeltIterator, Field, Member, Struct, pop_primitive,
    pop_short_utf8, pop_u256, pop_u512, read_serialized_felt_array,
};
use convert_case::{Case, Casing};
use num_traits::Zero;
use starknet_types_core::felt::Felt;

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
            Value::ShortUtf8(value) | Value::Utf8Array(value) => Some(value.clone()),
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
            Value::ByteArray(s) => Some(String::from_utf8_lossy(s).to_string()),
            Value::Enum(v) => v.to_primitive_string(),
            Value::Option(v) => v.to_primitive_string(),
            Value::Nullable(v) => v.to_primitive_string(),
            _ => None,
        }
    }
}

impl ToValue for TypeDef {
    type Value = Value;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Value> {
        match self {
            TypeDef::None => Some(Value::None),
            TypeDef::Felt252 => pop_primitive(data).map(Value::Felt252),
            TypeDef::ShortUtf8 => pop_short_utf8(data).map(Value::ShortUtf8),
            TypeDef::Bytes31 => pop_bytes31(data).map(Value::Bytes31),
            TypeDef::Bytes31E(encoding) => {
                pop_bytes31_encoded(*encoding, data).map(Value::Bytes31E)
            }
            TypeDef::Bool => data.next().map(|v| Value::Bool(!v.is_zero())),
            TypeDef::U8 => pop_primitive(data).map(Value::U8),
            TypeDef::U16 => pop_primitive(data).map(Value::U16),
            TypeDef::U32 => pop_primitive(data).map(Value::U32),
            TypeDef::U64 => pop_primitive(data).map(Value::U64),
            TypeDef::U128 => pop_primitive(data).map(Value::U128),
            TypeDef::U256 => pop_u256(data).map(Value::U256),
            TypeDef::U512 => pop_u512(data).map(Value::U512),
            TypeDef::I8 => pop_primitive(data).map(Value::I8),
            TypeDef::I16 => pop_primitive(data).map(Value::I16),
            TypeDef::I32 => pop_primitive(data).map(Value::I32),
            TypeDef::I64 => pop_primitive(data).map(Value::I64),
            TypeDef::I128 => pop_primitive(data).map(Value::I128),
            TypeDef::ClassHash => pop_primitive(data).map(Value::ClassHash),
            TypeDef::ContractAddress => data.next().map(Value::ContractAddress),
            TypeDef::EthAddress => data.next().map(Value::EthAddress),
            TypeDef::StorageAddress => pop_primitive(data).map(Value::StorageAddress),
            TypeDef::StorageBaseAddress => pop_primitive(data).map(Value::StorageBaseAddress),
            TypeDef::ByteArray => pop_byte_array(data).map(Value::ByteArray),
            TypeDef::Utf8Array => pop_utf8_array(data).map(Value::Utf8Array),
            TypeDef::ByteArrayE(encoding) => {
                pop_byte_array_encoded(*encoding, data).map(Value::ByteArrayE)
            }
            TypeDef::Tuple(type_defs) => parse_tuple_to_value(type_defs, data).map(Value::Tuple),
            TypeDef::Array(type_def) => {
                let size = pop_primitive(data)?;
                type_def.to_value_multiple(data, size).map(Value::Array)
            }
            TypeDef::FixedArray(fa) => fa.to_value(data).map(Value::FixedArray),
            TypeDef::Felt252Dict(_ty) => None,
            TypeDef::Struct(s) => s.to_value(data).map(Value::Struct),
            TypeDef::Enum(e) => e.to_value(data).map(Box::new).map(Value::Enum),
            TypeDef::Ref(_) => None,
            TypeDef::Custom(name) => to_custom_value(name.clone(), data).map(Value::Custom),
            TypeDef::Option(type_def) => to_option_value(type_def, data)
                .map(Box::new)
                .map(Value::Option),
            TypeDef::Result(_r) => None,
            TypeDef::Nullable(_ty) => None,
        }
    }
}

impl ToValue for MemberDef {
    type Value = Member;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Member> {
        Some(Member {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.to_value(data)?,
        })
    }
}

impl ToValue for StructDef {
    type Value = Struct;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Struct> {
        Some(Struct {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            members: self
                .members
                .iter()
                .map(|child| child.to_value(data))
                .collect::<Option<Vec<Member>>>()?,
        })
    }
}

impl ToValue for FixedArrayDef {
    type Value = Vec<Value>;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Vec<Value>> {
        self.type_def.to_value_multiple(data, self.size as usize)
    }
}

impl ToValue for EnumDef {
    type Value = Enum;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Enum> {
        let selector = pop_primitive::<Felt>(data)?;
        let field = self.variants.get(&selector)?;

        Some(Enum {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            variant: field.name.clone(),
            variant_attributes: field.attributes.clone(),
            value: field.type_def.to_value(data)?,
        })
    }

    fn to_value_multiple(&self, data: &mut FeltIterator, count: usize) -> Option<Vec<Self::Value>> {
        (0..count)
            .into_iter()
            .map(|_| self.to_value(data))
            .collect()
    }
}

impl ToValue for ColumnDef {
    type Value = Field;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Field> {
        Some(Field {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.to_value(data)?,
        })
    }
}

pub fn pop_bytes31_encoded(encoding: Felt, data: &mut FeltIterator) -> Option<EncodedBytes> {
    let bytes = data.next()?.to_bytes_be();

    Some(EncodedBytes {
        bytes: bytes[1..32].try_into().ok()?,
        encoding,
    })
}

pub fn pop_byte_array(data: &mut FeltIterator) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    loop {
        let felt_bytes = data.next()?.to_bytes_be();
        let info = felt_bytes[0];
        bytes.extend_from_slice(match info & 2 {
            0 => &felt_bytes[1..32],
            _ => &felt_bytes[(32 - felt_bytes[1] as usize)..32],
        });

        if info & 1 == 1 {
            return Some(bytes);
        }
    }
}

pub fn pop_byte_array_encoded(encoding: Felt, data: &mut FeltIterator) -> Option<EncodedBytes> {
    let bytes = pop_byte_array(data)?;
    Some(EncodedBytes { bytes, encoding })
}

pub fn pop_utf8_array(data: &mut FeltIterator) -> Option<String> {
    let byte_array = pop_byte_array(data)?;
    String::from_utf8_lossy(&byte_array).into_owned().into()
}

fn parse_tuple_to_value(type_defs: &Vec<TypeDef>, data: &mut FeltIterator) -> Option<Vec<Value>> {
    type_defs
        .iter()
        .map(|type_def| type_def.to_value(data))
        .collect::<Option<Vec<Value>>>()
}

fn to_custom_value(name: Felt, data: &mut FeltIterator) -> Option<Custom> {
    Some(Custom {
        name: name,
        values: read_serialized_felt_array(data)?,
    })
}

fn to_option_value(type_def: &TypeDef, data: &mut FeltIterator) -> Option<Option<Value>> {
    let is_some = data.next()?.is_zero();
    match is_some {
        true => type_def.to_value(data).map(Some),
        false => Some(None),
    }
}
