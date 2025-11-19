use crate::type_def::{
    ByteArrayDeserialization, ByteArrayEDef, FixedArrayDef, MemberDef, StructDef, TypeDef,
};
use crate::value::{Enum, Nullable, Value};
use crate::{
    ArrayDef, CairoDeserialize, CairoOption, CairoResult, ColumnDef, Custom, CustomDef,
    EncodedBytes, EnumDef, FeltIterator, Field, Member, NullableDef, OptionDef, ResultDef, Struct,
    TupleDef, deserialize_byte_array, pop_bytes31, pop_primitive, pop_short_utf8, pop_u256,
    pop_u512,
};
use num_traits::Zero;
use starknet_types_core::felt::Felt;
use std::sync::Arc;

impl<T> ToValue for Vec<T>
where
    T: ToValue,
{
    type Value = Vec<T::Value>;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Vec<T::Value>> {
        self.iter()
            .map(|item| item.to_value(data))
            .collect::<Option<Vec<T::Value>>>()
    }
}

impl<T: ToValue> ToValue for Arc<T> {
    type Value = T::Value;
    fn to_value(&self, data: &mut FeltIterator) -> Option<T::Value> {
        self.as_ref().to_value(data)
    }
}

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
            TypeDef::ByteArray(mode) => read_byte_array(mode.clone(), data).map(Value::ByteArray),
            TypeDef::Utf8String(mode) => read_utf8_array(mode.clone(), data).map(Value::Utf8String),
            TypeDef::ByteArrayE(bae) => bae.to_value(data).map(Value::ByteArrayE),
            TypeDef::Tuple(tuple) => tuple.to_value(data).map(Value::Tuple),
            TypeDef::Array(a) => a.to_value(data).map(Value::Array),
            TypeDef::FixedArray(fa) => fa.to_value(data).map(Value::FixedArray),
            TypeDef::Felt252Dict(_ty) => None,
            TypeDef::Struct(s) => s.to_value(data).map(Value::Struct),
            TypeDef::Enum(e) => e.to_value(data).map(Box::new).map(Value::Enum),
            TypeDef::Ref(_) => None,
            TypeDef::Custom(custom) => custom.to_value(data).map(Value::Custom),
            TypeDef::Option(option) => option.to_value(data).map(Box::new).map(Value::Option),
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

impl ToValue for ArrayDef {
    type Value = Vec<Value>;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Vec<Value>> {
        let count = pop_primitive::<usize>(data)?;
        self.type_def.to_value_multiple(data, count)
    }
}

impl ToValue for FixedArrayDef {
    type Value = Vec<Value>;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Vec<Value>> {
        self.type_def.to_value_multiple(data, self.size as usize)
    }
}

impl ToValue for TupleDef {
    type Value = Vec<Value>;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Vec<Value>> {
        self.elements.to_value(data)
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
}

impl ToValue for OptionDef {
    type Value = CairoOption<Value>;
    fn to_value(&self, data: &mut FeltIterator) -> Option<CairoOption<Value>> {
        let is_some = data.next()?.is_zero();
        match is_some {
            true => self.type_def.to_value(data).map(CairoOption::Some),
            false => Some(CairoOption::None),
        }
    }
}

impl ToValue for ResultDef {
    type Value = CairoResult<Value, Value>;
    fn to_value(&self, data: &mut FeltIterator) -> Option<CairoResult<Value, Value>> {
        let is_ok = data.next()?.is_zero();
        match is_ok {
            true => self.ok.to_value(data).map(CairoResult::Ok),
            false => self.err.to_value(data).map(CairoResult::Err),
        }
    }
}

impl ToValue for NullableDef {
    type Value = Nullable;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Nullable> {
        let is_null = data.next()?.is_zero();
        match is_null {
            false => self.type_def.to_value(data).map(Nullable::NotNull),
            true => Some(Nullable::Null),
        }
    }
}

impl ToValue for CustomDef {
    type Value = Custom;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Custom> {
        Some(Custom {
            id: self.id.clone(),
            values: Vec::<Felt>::c_deserialize(data)?,
        })
    }
}

impl ToValue for ColumnDef {
    type Value = Field;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Field> {
        Some(Field {
            id: self.id.clone(),
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.to_value(data)?,
        })
    }
}

impl ToValue for ByteArrayEDef {
    type Value = EncodedBytes;
    fn to_value(&self, data: &mut FeltIterator) -> Option<Self::Value> {
        read_byte_array(self.mode, data).map(|bytes| EncodedBytes {
            encoding: self.encoding,
            bytes,
        })
    }
}

pub fn pop_bytes31_encoded(encoding: Felt, data: &mut FeltIterator) -> Option<EncodedBytes> {
    Some(EncodedBytes {
        bytes: pop_bytes31(data)?.into(),
        encoding,
    })
}

pub fn read_byte_array(mode: ByteArrayDeserialization, data: &mut FeltIterator) -> Option<Vec<u8>> {
    match mode {
        ByteArrayDeserialization::Serde => deserialize_byte_array(data),
        ByteArrayDeserialization::ISerde => i_deserialize_byte_array(data),
    }
}

pub fn i_deserialize_byte_array(data: &mut FeltIterator) -> Option<Vec<u8>> {
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
    let bytes = i_deserialize_byte_array(data)?;
    Some(EncodedBytes { bytes, encoding })
}

pub fn read_utf8_array(mode: ByteArrayDeserialization, data: &mut FeltIterator) -> Option<String> {
    let byte_array = read_byte_array(mode, data)?;
    String::from_utf8_lossy(&byte_array).into_owned().into()
}
