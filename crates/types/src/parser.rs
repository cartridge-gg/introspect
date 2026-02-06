use crate::deserialize::CairoDeserializer;
use crate::type_def::{
    ByteArrayDeserialization, ByteArrayEncodedDef, FixedArrayDef, MemberDef, StructDef, TypeDef,
};
use crate::value::{Enum, Nullable, Value};
use crate::{
    ArrayDef, Bytes31EncodedDef, CairoOption, CairoResult, ColumnDef, Custom, CustomDef,
    Encoded31Bytes, EncodedBytes, EnumDef, Field, Member, NullableDef, OptionDef, ResultDef,
    Struct, TupleDef, felt_to_bytes31,
};

use starknet_types_core::felt::Felt;

pub struct DefaultParser {
    byte_array_mode: ByteArrayDeserialization,
}

// impl DefaultParser {
//     pub fn read_byte_array(&self, data: &mut FeltIterator) -> Option<Vec<u8>> {
//         match self.byte_array_mode {
//             ByteArrayDeserialization::Serde => deserialize_byte_array(data),
//             ByteArrayDeserialization::ISerde => ideserialize_byte_array(data),
//         }
//     }
//     pub fn read_utf8_array(&self, data: &mut FeltIterator) -> Option<String> {
//         let byte_array = self.read_byte_array(data)?;
//         String::from_utf8_lossy(&byte_array).into_owned().into()
//     }
// }

pub trait TypeParser<D: CairoDeserializer> {
    type Value;
    fn to_values(&self, deserializer: &mut D) -> Option<Vec<Self::Value>>;
}

impl<T: ToValue<D>, D: CairoDeserializer> TypeParser<D> for Vec<T> {
    type Value = <T as ToValue<D>>::Value;
    fn to_values(&self, deserializer: &mut D) -> Option<Vec<Self::Value>> {
        self.iter()
            .map(|item| item.to_value(deserializer))
            .collect::<Option<Vec<Self::Value>>>()
    }
}

pub trait ToValue<D: CairoDeserializer> {
    type Value;
    fn to_value(&self, deserializer: &mut D) -> Option<Self::Value>;
    fn to_value_boxed(&self, deserializer: &mut D) -> Option<Box<Self::Value>> {
        self.to_value(deserializer).map(Box::new)
    }
    fn to_multiple_value(&self, deserializer: &mut D, count: usize) -> Option<Vec<Self::Value>> {
        (0..count)
            .into_iter()
            .map(|_| self.to_value(deserializer))
            .collect()
    }
    // fn to_value_multiple<I: FeltIterator>(
    //     &self,
    //     item: &T,
    //     data: &mut I,
    //     count: usize,
    // ) -> Option<Vec<Self::Value>> {
    //     (0..count)
    //         .into_iter()
    //         .map(|_| self.to_value(item, data))
    //         .collect()
    // }
}

impl<D: CairoDeserializer> ToValue<D> for TypeDef {
    type Value = Value;
    fn to_value(&self, deserializer: &mut D) -> Option<Value> {
        match self {
            TypeDef::None => Some(Value::None),
            TypeDef::Felt252 => deserializer.next_felt().map(Value::Felt252),
            TypeDef::ShortUtf8 => deserializer
                .next_bytes31()
                .map(|b| b.to_string())
                .map(Value::ShortUtf8),
            TypeDef::Bytes31 => deserializer
                .next_bytes31()
                .map(Into::into)
                .map(Value::Bytes31),
            TypeDef::Bytes31Encoded(b) => b.to_value(deserializer).map(Value::Bytes31Encoded),
            TypeDef::Bool => deserializer.next_bool().map(Value::Bool),
            TypeDef::U8 => deserializer.next_u8().map(Value::U8),
            TypeDef::U16 => deserializer.next_u16().map(Value::U16),
            TypeDef::U32 => deserializer.next_u32().map(Value::U32),
            TypeDef::U64 => deserializer.next_u64().map(Value::U64),
            TypeDef::U128 => deserializer.next_u128().map(Value::U128),
            TypeDef::U256 => deserializer.next_u256().map(Value::U256),
            TypeDef::U512 => deserializer.next_u512().map(Value::U512),
            TypeDef::I8 => deserializer.next_i8().map(Value::I8),
            TypeDef::I16 => deserializer.next_i16().map(Value::I16),
            TypeDef::I32 => deserializer.next_i32().map(Value::I32),
            TypeDef::I64 => deserializer.next_i64().map(Value::I64),
            TypeDef::I128 => deserializer.next_i128().map(Value::I128),
            TypeDef::ClassHash => deserializer.next_felt().map(Value::ClassHash),
            TypeDef::ContractAddress => deserializer.next_felt().map(Value::ContractAddress),
            TypeDef::EthAddress => deserializer.next_felt().map(Value::EthAddress),
            TypeDef::StorageAddress => deserializer.next_felt().map(Value::StorageAddress),
            TypeDef::StorageBaseAddress => deserializer.next_felt().map(Value::StorageBaseAddress),
            TypeDef::ByteArray => deserializer.next_byte_array_bytes().map(Value::ByteArray),
            TypeDef::Utf8String => deserializer.next_string().map(Value::Utf8String),
            TypeDef::ByteArrayEncoded(bae) => {
                bae.to_value(deserializer).map(Value::ByteArrayEncoded)
            }
            TypeDef::Tuple(tuple) => tuple.to_value(deserializer).map(Value::Tuple),
            TypeDef::Array(a) => a.to_value(deserializer).map(Value::Array),
            TypeDef::FixedArray(fa) => fa.to_value(deserializer).map(Value::FixedArray),
            TypeDef::Felt252Dict(_ty) => None,
            TypeDef::Struct(s) => s.to_value(deserializer).map(Value::Struct),
            TypeDef::Enum(e) => e.to_value_boxed(deserializer).map(Value::Enum),
            TypeDef::Ref(_) => None,
            TypeDef::Custom(custom) => custom.to_value(deserializer).map(Value::Custom),
            TypeDef::Option(option) => option.to_value_boxed(deserializer).map(Value::Option),
            TypeDef::Result(_r) => None,
            TypeDef::Nullable(_ty) => None,
        }
    }
}

impl<D: CairoDeserializer> ToValue<D> for MemberDef {
    type Value = Member;
    fn to_value(&self, deserializer: &mut D) -> Option<Member> {
        Some(Member {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.to_value(deserializer)?,
        })
    }
}

impl<D: CairoDeserializer> ToValue<D> for StructDef {
    type Value = Struct;
    fn to_value(&self, deserializer: &mut D) -> Option<Struct> {
        Some(Struct {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            members: self.members.to_values(deserializer)?,
        })
    }
}

impl<D: CairoDeserializer> ToValue<D> for ArrayDef {
    type Value = Vec<Value>;
    fn to_value(&self, deserializer: &mut D) -> Option<Vec<Value>> {
        let count = deserializer.next_usize()?;
        self.type_def.to_multiple_value(deserializer, count)
    }
}

impl<D: CairoDeserializer> ToValue<D> for FixedArrayDef {
    type Value = Vec<Value>;
    fn to_value(&self, deserializer: &mut D) -> Option<Vec<Value>> {
        self.type_def
            .to_multiple_value(deserializer, self.size as usize)
    }
}

impl<D: CairoDeserializer> ToValue<D> for TupleDef {
    type Value = Vec<Value>;
    fn to_value(&self, deserializer: &mut D) -> Option<Vec<Value>> {
        self.elements.to_values(deserializer)
    }
}

impl<D: CairoDeserializer> ToValue<D> for EnumDef {
    type Value = Enum;
    fn to_value(&self, deserializer: &mut D) -> Option<Enum> {
        let selector = deserializer.next_felt()?;
        let field = self.variants.get(&selector)?;

        Some(Enum {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            variant: field.name.clone(),
            variant_attributes: field.attributes.clone(),
            value: field.type_def.to_value(deserializer)?,
        })
    }
}

impl<D: CairoDeserializer> ToValue<D> for OptionDef {
    type Value = CairoOption<Value>;
    fn to_value(&self, deserializer: &mut D) -> Option<CairoOption<Value>> {
        match deserializer.next_option_is_some()? {
            true => self.type_def.to_value(deserializer).map(CairoOption::Some),
            false => Some(CairoOption::None),
        }
    }
}

impl<D: CairoDeserializer> ToValue<D> for ResultDef {
    type Value = CairoResult<Value, Value>;
    fn to_value(&self, deserializer: &mut D) -> Option<CairoResult<Value, Value>> {
        match deserializer.next_result_is_ok()? {
            true => self.ok.to_value(deserializer).map(CairoResult::Ok),
            false => self.err.to_value(deserializer).map(CairoResult::Err),
        }
    }
}

impl<D: CairoDeserializer> ToValue<D> for NullableDef {
    type Value = Nullable;
    fn to_value(&self, deserializer: &mut D) -> Option<Nullable> {
        match deserializer.next_nullable_is_null()? {
            false => self.type_def.to_value(deserializer).map(Nullable::NotNull),
            true => Some(Nullable::Null),
        }
    }
}

impl<D: CairoDeserializer> ToValue<D> for CustomDef {
    type Value = Custom;
    fn to_value(&self, deserializer: &mut D) -> Option<Custom> {
        Some(Custom {
            encoding: self.encoding.clone(),
            values: deserializer.next_array()?,
        })
    }
}

impl<D: CairoDeserializer> ToValue<D> for ColumnDef {
    type Value = Field;
    fn to_value(&self, deserializer: &mut D) -> Option<Field> {
        Some(Field {
            id: self.id.clone(),
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.to_value(deserializer)?,
        })
    }
}

impl<D: CairoDeserializer> ToValue<D> for ByteArrayEncodedDef {
    type Value = EncodedBytes;
    fn to_value(&self, deserializer: &mut D) -> Option<Self::Value> {
        deserializer
            .next_byte_array_bytes()
            .map(|bytes| EncodedBytes {
                encoding: self.encoding.clone(),
                bytes,
            })
    }
}

impl<D: CairoDeserializer> ToValue<D> for Bytes31EncodedDef {
    type Value = Encoded31Bytes;
    fn to_value(&self, deserializer: &mut D) -> Option<Self::Value> {
        deserializer.next_bytes31().map(|bytes| Encoded31Bytes {
            encoding: self.encoding.clone(),
            bytes: bytes.into(),
        })
    }
}

impl Bytes31EncodedDef {
    pub fn to_encoded_bytes_31(&self, felt: Felt) -> Option<Encoded31Bytes> {
        Some(Encoded31Bytes {
            encoding: self.encoding.clone(),
            bytes: felt_to_bytes31(felt)?,
        })
    }
}
