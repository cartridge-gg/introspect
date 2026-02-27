use std::cell::RefCell;

use serde::ser::{Error as SerError, SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};

use crate::transcode::CairoSerializer;
use crate::{CairoDeserializer, EnumDef, TypeDef};

pub struct SchemaSerialize<'a, D> {
    type_def: &'a TypeDef,
    de: &'a RefCell<&'a mut D>,
}

impl<'a, D: CairoDeserializer> SchemaSerialize<'a, D> {
    pub fn new(type_def: &'a TypeDef, de: &'a RefCell<&'a mut D>) -> Self {
        Self { type_def, de }
    }

    pub fn to_type(&self, type_def: &'a TypeDef) -> SchemaSerialize<'a, D> {
        SchemaSerialize {
            type_def,
            de: self.de,
        }
    }

    fn with_de<T, E>(&self, op: impl FnOnce(&mut D) -> Result<T, E>) -> Result<T, E> {
        let mut de = self.de.borrow_mut();
        op(&mut *de)
    }

    fn serialize_enum<S>(&self, serializer: S, enum_def: &'a EnumDef) -> Result<S::Ok, S::Error>
    where
        S: CairoSerializer,
    {
        let selector = self
            .with_de(|de| de.next_enum_variant())
            .map_err(S::Error::custom)?;
        let variant = enum_def.get_variant(&selector).map_err(S::Error::custom)?;

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&variant.name, &self.to_type(&variant.type_def))?;
        map.end()
    }

    pub fn serialize_with<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: CairoSerializer,
    {
        match self.type_def {
            TypeDef::None => serializer.serialize_unit(),
            TypeDef::Felt252
            | TypeDef::ClassHash
            | TypeDef::ContractAddress
            | TypeDef::StorageAddress
            | TypeDef::StorageBaseAddress => {
                let value = self
                    .with_de(|de| de.next_felt_bytes())
                    .map_err(S::Error::custom)?;
                serializer.serialize_felt(value)
            }
            TypeDef::ShortUtf8 => {
                let value = self
                    .with_de(|de| de.next_short_string())
                    .map_err(S::Error::custom)?;
                serializer.serialize_str(&value)
            }
            TypeDef::Bytes31 | TypeDef::Bytes31Encoded(_) => {
                let value = self
                    .with_de(|de| de.next_bytes::<31>())
                    .map_err(S::Error::custom)?;
                serializer.serialize_byte_string(&value)
            }
            TypeDef::Bool => {
                let value = self
                    .with_de(|de| de.next_bool())
                    .map_err(S::Error::custom)?;
                serializer.serialize_bool(value)
            }
            TypeDef::U8 => {
                let value = self.with_de(|de| de.next_u8()).map_err(S::Error::custom)?;
                serializer.serialize_u8(value)
            }
            TypeDef::U16 => {
                let value = self.with_de(|de| de.next_u16()).map_err(S::Error::custom)?;
                serializer.serialize_u16(value)
            }
            TypeDef::U32 => {
                let value = self.with_de(|de| de.next_u32()).map_err(S::Error::custom)?;
                serializer.serialize_u32(value)
            }
            TypeDef::U64 => {
                let value = self.with_de(|de| de.next_u64()).map_err(S::Error::custom)?;
                serializer.serialize_u64(value)
            }
            TypeDef::U128 => {
                let value = self
                    .with_de(|de| de.next_u128())
                    .map_err(S::Error::custom)?;
                serializer.serialize_u128(value)
            }
            TypeDef::U256 => {
                let value = self
                    .with_de(|de| de.next_u256())
                    .map_err(S::Error::custom)?;
                serializer.serialize_u256(value)
            }
            TypeDef::U512 => {
                let value = self
                    .with_de(|de| de.next_u512())
                    .map_err(S::Error::custom)?;
                serializer.serialize_u512(value)
            }
            TypeDef::I8 => {
                let value = self.with_de(|de| de.next_i8()).map_err(S::Error::custom)?;
                serializer.serialize_i8(value)
            }
            TypeDef::I16 => {
                let value = self.with_de(|de| de.next_i16()).map_err(S::Error::custom)?;
                serializer.serialize_i16(value)
            }
            TypeDef::I32 => {
                let value = self.with_de(|de| de.next_i32()).map_err(S::Error::custom)?;
                serializer.serialize_i32(value)
            }
            TypeDef::I64 => {
                let value = self.with_de(|de| de.next_i64()).map_err(S::Error::custom)?;
                serializer.serialize_i64(value)
            }
            TypeDef::I128 => {
                let value = self
                    .with_de(|de| de.next_i128())
                    .map_err(S::Error::custom)?;
                serializer.serialize_i128(value)
            }
            TypeDef::EthAddress => {
                let value = self
                    .with_de(|de| de.next_eth_address())
                    .map_err(S::Error::custom)?;
                serializer.serialize_eth_address(value.0)
            }
            TypeDef::ByteArray | TypeDef::ByteArrayEncoded(_) | TypeDef::Custom(_) => {
                let value = self
                    .with_de(|de| de.next_byte_array_bytes())
                    .map_err(S::Error::custom)?;
                serializer.serialize_byte_string(&value)
            }
            TypeDef::Utf8String => {
                let value = self
                    .with_de(|de| de.next_string())
                    .map_err(S::Error::custom)?;
                serializer.serialize_str(&value)
            }
            TypeDef::Tuple(tuple) => {
                let mut seq = serializer.serialize_seq(Some(tuple.elements.len()))?;
                for element in &tuple.elements {
                    seq.serialize_element(&self.to_type(element))?;
                }
                seq.end()
            }
            TypeDef::Array(array_def) => {
                let len = self.with_de(|de| de.next_u32()).map_err(S::Error::custom)? as usize;
                let mut seq = serializer.serialize_seq(Some(len))?;
                for _ in 0..len {
                    seq.serialize_element(&self.to_type(&array_def.type_def))?;
                }
                seq.end()
            }
            TypeDef::FixedArray(fixed_array_def) => {
                let len = fixed_array_def.size as usize;
                let mut seq = serializer.serialize_seq(Some(len))?;
                for _ in 0..len {
                    seq.serialize_element(&self.to_type(&fixed_array_def.type_def))?;
                }
                seq.end()
            }
            TypeDef::Felt252Dict(_) => Err(S::Error::custom(
                "Felt252Dict transcoding is not implemented",
            )),
            TypeDef::Struct(struct_def) => {
                let mut map = serializer.serialize_map(Some(struct_def.members.len()))?;
                for member in &struct_def.members {
                    map.serialize_entry(&member.name, &self.to_type(&member.type_def))?;
                }
                map.end()
            }
            TypeDef::Enum(enum_def) => self.serialize_enum(serializer, enum_def),
            TypeDef::Ref(_) => Err(S::Error::custom(
                "TypeDef Ref needs to be expanded before serializing",
            )),
            TypeDef::Option(option_def) => {
                let is_some = self
                    .with_de(|de| de.next_option_is_some())
                    .map_err(S::Error::custom)?;
                if is_some {
                    serializer.serialize_some(&self.to_type(&option_def.type_def))
                } else {
                    serializer.serialize_none()
                }
            }
            TypeDef::Result(result_def) => {
                let is_ok = self
                    .with_de(|de| de.next_result_is_ok())
                    .map_err(S::Error::custom)?;
                let mut map = serializer.serialize_map(Some(1))?;
                if is_ok {
                    map.serialize_entry("Ok", &self.to_type(&result_def.ok))?;
                } else {
                    map.serialize_entry("Err", &self.to_type(&result_def.err))?;
                }
                map.end()
            }
            TypeDef::Nullable(nullable_def) => {
                let is_null = self
                    .with_de(|de| de.next_nullable_is_null())
                    .map_err(S::Error::custom)?;
                if is_null {
                    serializer.serialize_none()
                } else {
                    serializer.serialize_some(&self.to_type(&nullable_def.type_def))
                }
            }
        }
    }

    pub fn serialize_fallback<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.type_def {
            TypeDef::None => serializer.serialize_unit(),
            TypeDef::Felt252
            | TypeDef::ClassHash
            | TypeDef::ContractAddress
            | TypeDef::StorageAddress
            | TypeDef::StorageBaseAddress => {
                let value = self
                    .with_de(|de| de.next_felt_bytes())
                    .map_err(S::Error::custom)?;
                serializer.serialize_bytes(&value)
            }
            TypeDef::ShortUtf8 => {
                let value = self
                    .with_de(|de| de.next_short_string())
                    .map_err(S::Error::custom)?;
                serializer.serialize_str(&value)
            }
            TypeDef::Bytes31 | TypeDef::Bytes31Encoded(_) => {
                let value = self
                    .with_de(|de| de.next_bytes::<31>())
                    .map_err(S::Error::custom)?;
                serializer.serialize_bytes(&value)
            }
            TypeDef::Bool => {
                let value = self
                    .with_de(|de| de.next_bool())
                    .map_err(S::Error::custom)?;
                serializer.serialize_bool(value)
            }
            TypeDef::U8 => {
                let value = self.with_de(|de| de.next_u8()).map_err(S::Error::custom)?;
                serializer.serialize_u8(value)
            }
            TypeDef::U16 => {
                let value = self.with_de(|de| de.next_u16()).map_err(S::Error::custom)?;
                serializer.serialize_u16(value)
            }
            TypeDef::U32 => {
                let value = self.with_de(|de| de.next_u32()).map_err(S::Error::custom)?;
                serializer.serialize_u32(value)
            }
            TypeDef::U64 => {
                let value = self.with_de(|de| de.next_u64()).map_err(S::Error::custom)?;
                serializer.serialize_u64(value)
            }
            TypeDef::U128 => {
                let value = self
                    .with_de(|de| de.next_u128())
                    .map_err(S::Error::custom)?;
                serializer.serialize_u128(value)
            }
            TypeDef::U256 => {
                let value = self
                    .with_de(|de| de.next_u256())
                    .map_err(S::Error::custom)?;
                serializer.collect_str(&value)
            }
            TypeDef::U512 => {
                let value = self
                    .with_de(|de| de.next_u512())
                    .map_err(S::Error::custom)?;
                serializer.collect_str(&value)
            }
            TypeDef::I8 => {
                let value = self.with_de(|de| de.next_i8()).map_err(S::Error::custom)?;
                serializer.serialize_i8(value)
            }
            TypeDef::I16 => {
                let value = self.with_de(|de| de.next_i16()).map_err(S::Error::custom)?;
                serializer.serialize_i16(value)
            }
            TypeDef::I32 => {
                let value = self.with_de(|de| de.next_i32()).map_err(S::Error::custom)?;
                serializer.serialize_i32(value)
            }
            TypeDef::I64 => {
                let value = self.with_de(|de| de.next_i64()).map_err(S::Error::custom)?;
                serializer.serialize_i64(value)
            }
            TypeDef::I128 => {
                let value = self
                    .with_de(|de| de.next_i128())
                    .map_err(S::Error::custom)?;
                serializer.serialize_i128(value)
            }
            TypeDef::EthAddress => {
                let value = self
                    .with_de(|de| de.next_eth_address())
                    .map_err(S::Error::custom)?;
                serializer.serialize_bytes(&value.0)
            }
            TypeDef::ByteArray | TypeDef::ByteArrayEncoded(_) | TypeDef::Custom(_) => {
                let value = self
                    .with_de(|de| de.next_byte_array_bytes())
                    .map_err(S::Error::custom)?;
                serializer.serialize_bytes(&value)
            }
            TypeDef::Utf8String => {
                let value = self
                    .with_de(|de| de.next_string())
                    .map_err(S::Error::custom)?;
                serializer.serialize_str(&value)
            }
            TypeDef::Tuple(tuple) => {
                let mut seq = serializer.serialize_seq(Some(tuple.elements.len()))?;
                for element in &tuple.elements {
                    seq.serialize_element(&self.to_type(element))?;
                }
                seq.end()
            }
            TypeDef::Array(array_def) => {
                let len = self.with_de(|de| de.next_u32()).map_err(S::Error::custom)? as usize;
                let mut seq = serializer.serialize_seq(Some(len))?;
                for _ in 0..len {
                    seq.serialize_element(&self.to_type(&array_def.type_def))?;
                }
                seq.end()
            }
            TypeDef::FixedArray(fixed_array_def) => {
                let len = fixed_array_def.size as usize;
                let mut seq = serializer.serialize_seq(Some(len))?;
                for _ in 0..len {
                    seq.serialize_element(&self.to_type(&fixed_array_def.type_def))?;
                }
                seq.end()
            }
            TypeDef::Felt252Dict(_) => Err(S::Error::custom(
                "Felt252Dict transcoding is not implemented",
            )),
            TypeDef::Struct(struct_def) => {
                let mut map = serializer.serialize_map(Some(struct_def.members.len()))?;
                for member in &struct_def.members {
                    map.serialize_entry(&member.name, &self.to_type(&member.type_def))?;
                }
                map.end()
            }
            TypeDef::Enum(enum_def) => {
                let selector = self
                    .with_de(|de| de.next_enum_variant())
                    .map_err(S::Error::custom)?;
                let variant = enum_def.get_variant(&selector).map_err(S::Error::custom)?;

                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&variant.name, &self.to_type(&variant.type_def))?;
                map.end()
            }
            TypeDef::Ref(_) => Err(S::Error::custom(
                "TypeDef Ref needs to be expanded before serializing",
            )),
            TypeDef::Option(option_def) => {
                let is_some = self
                    .with_de(|de| de.next_option_is_some())
                    .map_err(S::Error::custom)?;
                if is_some {
                    serializer.serialize_some(&self.to_type(&option_def.type_def))
                } else {
                    serializer.serialize_none()
                }
            }
            TypeDef::Result(result_def) => {
                let is_ok = self
                    .with_de(|de| de.next_result_is_ok())
                    .map_err(S::Error::custom)?;
                let mut map = serializer.serialize_map(Some(1))?;
                if is_ok {
                    map.serialize_entry("Ok", &self.to_type(&result_def.ok))?;
                } else {
                    map.serialize_entry("Err", &self.to_type(&result_def.err))?;
                }
                map.end()
            }
            TypeDef::Nullable(nullable_def) => {
                let is_null = self
                    .with_de(|de| de.next_nullable_is_null())
                    .map_err(S::Error::custom)?;
                if is_null {
                    serializer.serialize_none()
                } else {
                    serializer.serialize_some(&self.to_type(&nullable_def.type_def))
                }
            }
        }
    }
}

impl<'a, D: CairoDeserializer> Serialize for SchemaSerialize<'a, D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.serialize_fallback(serializer)
    }
}

pub fn serialize_with_type_def<D, S>(
    type_def: &TypeDef,
    input: &mut D,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    D: CairoDeserializer,
    S: CairoSerializer,
{
    let de = RefCell::new(input);
    SchemaSerialize::new(type_def, &de).serialize_with(serializer)
}
