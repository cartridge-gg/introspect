use crate::type_def::{
    ByteArrayDeserialization, ByteArrayEDef, FixedArrayDef, MemberDef, StructDef, TypeDef,
};
use crate::utils::ideserialize_byte_array;
use crate::value::{Enum, Nullable, Value};
use crate::{
    ArrayDef, Bytes31EDef, CairoOption, CairoResult, ColumnDef, Custom, CustomDef, EncodedBytes,
    EnumDef, FeltIterator, Field, ISerde, Member, NullableDef, OptionDef, ResultDef, Struct,
    TupleDef, deserialize_byte_array, pop_bytes31, pop_primitive, pop_short_utf8, pop_u256,
    pop_u512,
};
use num_traits::Zero;
use starknet_types_core::felt::Felt;
use std::sync::Arc;

pub struct DefaultParser {
    byte_array_mode: ByteArrayDeserialization,
}

impl DefaultParser {
    pub fn read_byte_array(&self, data: &mut FeltIterator) -> Option<Vec<u8>> {
        match self.byte_array_mode {
            ByteArrayDeserialization::Serde => deserialize_byte_array(data),
            ByteArrayDeserialization::ISerde => ideserialize_byte_array(data),
        }
    }
    pub fn read_utf8_array(&self, data: &mut FeltIterator) -> Option<String> {
        let byte_array = self.read_byte_array(data)?;
        String::from_utf8_lossy(&byte_array).into_owned().into()
    }
}

pub trait ToValue<T> {
    type Value;
    fn to_value(&self, item: &T, data: &mut FeltIterator) -> Option<Self::Value>;
    fn to_value_multiple(
        &self,
        item: &T,
        data: &mut FeltIterator,
        count: usize,
    ) -> Option<Vec<Self::Value>> {
        (0..count)
            .into_iter()
            .map(|_| self.to_value(item, data))
            .collect()
    }
}

impl<T> ToValue<Vec<T>> for DefaultParser
where
    Self: ToValue<T>,
{
    type Value = Vec<<Self as ToValue<T>>::Value>;
    fn to_value(
        &self,
        item: &Vec<T>,
        data: &mut FeltIterator,
    ) -> Option<Vec<<Self as ToValue<T>>::Value>> {
        item.iter()
            .map(|item| self.to_value(item, data))
            .collect::<Option<Vec<<Self as ToValue<T>>::Value>>>()
    }
}

impl<T> ToValue<Box<T>> for DefaultParser
where
    Self: ToValue<T>,
{
    type Value = <Self as ToValue<T>>::Value;
    fn to_value(
        &self,
        item: &Box<T>,
        data: &mut FeltIterator,
    ) -> Option<<Self as ToValue<T>>::Value> {
        self.to_value(item.as_ref(), data)
    }
}

// pub trait ToItemValue<T>
// where
//     Self: ToValue<T>,
//     Self::Value: ItemDefTrait,
// {
//     fn to_item_value(&self, item: &T, data: &mut FeltIterator) -> Option<Value> {
//         self.to_value(item, data).map(Self::Value::wrap_to_type_def)
//     }
// }

impl<T> ToValue<Arc<T>> for DefaultParser
where
    Self: ToValue<T>,
{
    type Value = <Self as ToValue<T>>::Value;
    fn to_value(
        &self,
        item: &Arc<T>,
        data: &mut FeltIterator,
    ) -> Option<<Self as ToValue<T>>::Value> {
        self.to_value(item.as_ref(), data)
    }
}

impl ToValue<TypeDef> for DefaultParser {
    type Value = Value;
    fn to_value(&self, item: &TypeDef, data: &mut FeltIterator) -> Option<Value> {
        match item {
            TypeDef::None => Some(Value::None),
            TypeDef::Felt252 => pop_primitive(data).map(Value::Felt252),
            TypeDef::ShortUtf8 => pop_short_utf8(data).map(Value::ShortUtf8),
            TypeDef::Bytes31 => pop_bytes31(data).map(Value::Bytes31),
            TypeDef::Bytes31E(b) => self.to_value(b, data).map(Value::Bytes31E),
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
            TypeDef::ByteArray => self.read_byte_array(data).map(Value::ByteArray),
            TypeDef::Utf8String => self.read_utf8_array(data).map(Value::Utf8String),
            TypeDef::ByteArrayE(bae) => self.to_value(bae, data).map(Value::ByteArrayE),
            TypeDef::Tuple(tuple) => self.to_value(tuple, data).map(Value::Tuple),
            TypeDef::Array(a) => self.to_value(a, data).map(Value::Array),
            TypeDef::FixedArray(fa) => self.to_value(fa, data).map(Value::FixedArray),
            TypeDef::Felt252Dict(_ty) => None,
            TypeDef::Struct(s) => self.to_value(s, data).map(Value::Struct),
            TypeDef::Enum(e) => self.to_value(e, data).map(Box::new).map(Value::Enum),
            TypeDef::Ref(_) => None,
            TypeDef::Custom(custom) => self.to_value(custom, data).map(Value::Custom),
            TypeDef::Option(option) => self.to_value(option, data).map(Box::new).map(Value::Option),
            TypeDef::Result(_r) => None,
            TypeDef::Nullable(_ty) => None,
        }
    }
}

impl ToValue<MemberDef> for DefaultParser {
    type Value = Member;
    fn to_value(&self, item: &MemberDef, data: &mut FeltIterator) -> Option<Member> {
        Some(Member {
            name: item.name.clone(),
            attributes: item.attributes.clone(),
            value: self.to_value(&item.type_def, data)?,
        })
    }
}

impl ToValue<StructDef> for DefaultParser {
    type Value = Struct;
    fn to_value(&self, item: &StructDef, data: &mut FeltIterator) -> Option<Struct> {
        Some(Struct {
            name: item.name.clone(),
            attributes: item.attributes.clone(),
            members: item
                .members
                .iter()
                .map(|child| self.to_value(child, data))
                .collect::<Option<Vec<Member>>>()?,
        })
    }
}

impl ToValue<ArrayDef> for DefaultParser {
    type Value = Vec<Value>;
    fn to_value(&self, item: &ArrayDef, data: &mut FeltIterator) -> Option<Vec<Value>> {
        let count = pop_primitive::<usize>(data)?;
        self.to_value_multiple(&item.type_def, data, count)
    }
}

impl ToValue<FixedArrayDef> for DefaultParser {
    type Value = Vec<Value>;
    fn to_value(&self, item: &FixedArrayDef, data: &mut FeltIterator) -> Option<Vec<Value>> {
        self.to_value_multiple(&item.type_def, data, item.size as usize)
    }
}

impl ToValue<TupleDef> for DefaultParser {
    type Value = Vec<Value>;
    fn to_value(&self, item: &TupleDef, data: &mut FeltIterator) -> Option<Vec<Value>> {
        self.to_value(&item.elements, data)
    }
}

impl ToValue<EnumDef> for DefaultParser {
    type Value = Enum;
    fn to_value(&self, item: &EnumDef, data: &mut FeltIterator) -> Option<Enum> {
        let selector = pop_primitive::<Felt>(data)?;
        let field = item.variants.get(&selector)?;

        Some(Enum {
            name: item.name.clone(),
            attributes: item.attributes.clone(),
            variant: field.name.clone(),
            variant_attributes: field.attributes.clone(),
            value: self.to_value(&field.type_def, data)?,
        })
    }
}

impl ToValue<OptionDef> for DefaultParser {
    type Value = CairoOption<Value>;
    fn to_value(&self, item: &OptionDef, data: &mut FeltIterator) -> Option<CairoOption<Value>> {
        let is_some = data.next()?.is_zero();
        match is_some {
            true => self.to_value(&item.type_def, data).map(CairoOption::Some),
            false => Some(CairoOption::None),
        }
    }
}

impl ToValue<ResultDef> for DefaultParser {
    type Value = CairoResult<Value, Value>;
    fn to_value(
        &self,
        item: &ResultDef,
        data: &mut FeltIterator,
    ) -> Option<CairoResult<Value, Value>> {
        let is_ok = data.next()?.is_zero();
        match is_ok {
            true => self.to_value(&item.ok, data).map(CairoResult::Ok),
            false => self.to_value(&item.err, data).map(CairoResult::Err),
        }
    }
}

impl ToValue<NullableDef> for DefaultParser {
    type Value = Nullable;
    fn to_value(&self, item: &NullableDef, data: &mut FeltIterator) -> Option<Nullable> {
        let is_null = data.next()?.is_zero();
        match is_null {
            false => self.to_value(&item.type_def, data).map(Nullable::NotNull),
            true => Some(Nullable::Null),
        }
    }
}

impl ToValue<CustomDef> for DefaultParser {
    type Value = Custom;
    fn to_value(&self, item: &CustomDef, data: &mut FeltIterator) -> Option<Custom> {
        Some(Custom {
            encoding: item.encoding.clone(),
            values: Vec::<Felt>::ideserialize(data)?,
        })
    }
}

impl ToValue<ColumnDef> for DefaultParser {
    type Value = Field;
    fn to_value(&self, item: &ColumnDef, data: &mut FeltIterator) -> Option<Field> {
        Some(Field {
            id: item.id.clone(),
            name: item.name.clone(),
            attributes: item.attributes.clone(),
            value: self.to_value(&item.type_def, data)?,
        })
    }
}

impl ToValue<ByteArrayEDef> for DefaultParser {
    type Value = EncodedBytes;
    fn to_value(&self, item: &ByteArrayEDef, data: &mut FeltIterator) -> Option<Self::Value> {
        self.read_byte_array(data).map(|bytes| EncodedBytes {
            encoding: item.encoding.clone(),
            bytes,
        })
    }
}

impl ToValue<Bytes31EDef> for DefaultParser {
    type Value = EncodedBytes;
    fn to_value(&self, item: &Bytes31EDef, data: &mut FeltIterator) -> Option<Self::Value> {
        pop_bytes31_encoded(item.encoding.clone(), data)
    }
}

pub fn pop_bytes31_encoded(encoding: String, data: &mut FeltIterator) -> Option<EncodedBytes> {
    Some(EncodedBytes {
        bytes: pop_bytes31(data)?.into(),
        encoding,
    })
}

pub fn read_byte_array(mode: ByteArrayDeserialization, data: &mut FeltIterator) -> Option<Vec<u8>> {
    match mode {
        ByteArrayDeserialization::Serde => deserialize_byte_array(data),
        ByteArrayDeserialization::ISerde => ideserialize_byte_array(data),
    }
}

pub fn pop_byte_array_encoded(encoding: String, data: &mut FeltIterator) -> Option<EncodedBytes> {
    let bytes = ideserialize_byte_array(data)?;
    Some(EncodedBytes { bytes, encoding })
}

pub fn read_utf8_array(mode: ByteArrayDeserialization, data: &mut FeltIterator) -> Option<String> {
    let byte_array = read_byte_array(mode, data)?;
    String::from_utf8_lossy(&byte_array).into_owned().into()
}
