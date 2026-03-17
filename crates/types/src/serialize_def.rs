use crate::serialize::{CairoSeFrom, ToCairoDeSeFrom};
use crate::{CairoDeserializer, EnumDef, PrimaryTypeDef, ResultDef, TupleDef, TypeDef, VariantDef};
use primitive_types::{U256, U512};
use serde::ser::{Error as SerError, SerializeMap, SerializeSeq, SerializeTuple};
use serde::{Serialize, Serializer};
use starknet_types_core::felt::Felt;

pub trait CairoTypeSerialization: Sized {
    fn serialize_byte_array<S: Serializer>(
        &self,
        serializer: S,
        value: &[u8],
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(value)
    }
    fn serialize_string<S: Serializer>(
        &self,
        serializer: S,
        value: &str,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(value)
    }
    fn serialize_felt<S: Serializer>(
        &self,
        serializer: S,
        value: &[u8; 32],
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(value)
    }
    fn serialize_eth_address<S: Serializer>(
        &self,
        serializer: S,
        value: &[u8; 20],
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(value)
    }
    fn serialize_u256<S: Serializer>(&self, serializer: S, value: U256) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }
    fn serialize_u512<S: Serializer>(&self, serializer: S, value: U512) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }
    fn serialize_tuple<'a, S: Serializer>(
        &'a self,
        data: &mut impl CairoDeserializer,
        serializer: S,
        tuple: &'a TupleDef,
    ) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(tuple.elements.len())?;
        for element in &tuple.elements {
            seq.serialize_element(&element.to_de_se(data, self))?;
        }
        seq.end()
    }

    fn serialize_enum<'a, S: Serializer>(
        &'a self,
        data: &mut impl CairoDeserializer,
        serializer: S,
        enum_def: &'a EnumDef,
        variant: Felt,
    ) -> Result<S::Ok, S::Error> {
        let VariantDef { name, type_def, .. } =
            enum_def.get_variant(&variant).map_err(S::Error::custom)?;
        self.serialize_variant(data, serializer, name, type_def)
    }

    fn serialize_variant<'a, S: Serializer>(
        &'a self,
        data: &mut impl CairoDeserializer,
        serializer: S,
        name: &str,
        type_def: &'a TypeDef,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&name, &type_def.to_de_se(data, self))?;
        map.end()
    }

    fn serialize_result<'a, S: Serializer>(
        &'a self,
        data: &mut impl CairoDeserializer,
        serializer: S,
        result: &'a ResultDef,
        is_ok: bool,
    ) -> Result<S::Ok, S::Error> {
        match is_ok {
            true => self.serialize_variant(data, serializer, "Ok", &result.ok),
            false => self.serialize_variant(data, serializer, "Err", &result.err),
        }
    }
}

pub trait CairoSerialize<'a, C> {
    fn serialize_cairo<S: Serializer>(
        &self,
        data: &mut impl CairoDeserializer,
        cairo_se: &C,
        serializer: S,
    ) -> Result<S::Ok, S::Error>;
}

impl<'a, T: CairoSerialize<'a, C>, D: CairoDeserializer, C> Serialize for CairoSeFrom<'a, T, D, C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let data = unsafe { &mut *self.de };
        self.schema.serialize_cairo(data, self.cairo_se, serializer)
    }
}

impl<'a, C: CairoTypeSerialization> CairoSerialize<'a, C> for TypeDef {
    fn serialize_cairo<S: Serializer>(
        &self,
        data: &mut impl CairoDeserializer,
        cairo_se: &C,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            TypeDef::None => serializer.serialize_unit(),
            TypeDef::Felt252
            | TypeDef::ClassHash
            | TypeDef::ContractAddress
            | TypeDef::StorageAddress
            | TypeDef::StorageBaseAddress => {
                let value = data.next_felt_bytes().map_err(S::Error::custom)?;
                cairo_se.serialize_felt(serializer, &value)
            }
            TypeDef::ShortUtf8 => {
                let value = data.next_short_string().map_err(S::Error::custom)?;
                cairo_se.serialize_string(serializer, &value)
            }
            TypeDef::Bytes31 | TypeDef::Bytes31Encoded(_) => {
                let value = data.next_bytes::<31>().map_err(S::Error::custom)?;
                serializer.serialize_bytes(&value)
            }
            TypeDef::Bool => {
                let value = data.next_bool().map_err(S::Error::custom)?;
                serializer.serialize_bool(value)
            }
            TypeDef::U8 => {
                let value = data.next_u8().map_err(S::Error::custom)?;
                serializer.serialize_u8(value)
            }
            TypeDef::U16 => {
                let value = data.next_u16().map_err(S::Error::custom)?;
                serializer.serialize_u16(value)
            }
            TypeDef::U32 => {
                let value = data.next_u32().map_err(S::Error::custom)?;
                serializer.serialize_u32(value)
            }
            TypeDef::U64 => {
                let value = data.next_u64().map_err(S::Error::custom)?;
                serializer.serialize_u64(value)
            }
            TypeDef::U128 => {
                let value = data.next_u128().map_err(S::Error::custom)?;
                serializer.serialize_u128(value)
            }
            TypeDef::U256 => {
                let value = data.next_u256().map_err(S::Error::custom)?;
                cairo_se.serialize_u256(serializer, value)
            }
            TypeDef::U512 => {
                let value = data.next_u512().map_err(S::Error::custom)?;
                cairo_se.serialize_u512(serializer, value)
            }
            TypeDef::I8 => {
                let value = data.next_i8().map_err(S::Error::custom)?;
                serializer.serialize_i8(value)
            }
            TypeDef::I16 => {
                let value = data.next_i16().map_err(S::Error::custom)?;
                serializer.serialize_i16(value)
            }
            TypeDef::I32 => {
                let value = data.next_i32().map_err(S::Error::custom)?;
                serializer.serialize_i32(value)
            }
            TypeDef::I64 => {
                let value = data.next_i64().map_err(S::Error::custom)?;
                serializer.serialize_i64(value)
            }
            TypeDef::I128 => {
                let value = data.next_i128().map_err(S::Error::custom)?;
                serializer.serialize_i128(value)
            }
            TypeDef::EthAddress => {
                let value = data.next_eth_address().map_err(S::Error::custom)?;
                cairo_se.serialize_eth_address(serializer, &value.0)
            }
            TypeDef::ByteArray | TypeDef::ByteArrayEncoded(_) | TypeDef::Custom(_) => {
                let value = data.next_byte_array_bytes().map_err(S::Error::custom)?;
                cairo_se.serialize_byte_array(serializer, &value)
            }
            TypeDef::Utf8String => {
                let value = data.next_string().map_err(S::Error::custom)?;
                cairo_se.serialize_string(serializer, &value)
            }
            TypeDef::Tuple(tuple) => cairo_se.serialize_tuple(data, serializer, tuple),
            TypeDef::Array(array_def) => {
                let len = data.next_u32().map_err(S::Error::custom)? as usize;
                let mut seq = serializer.serialize_seq(Some(len))?;
                let inner = array_def.type_def.to_de_se(data, cairo_se);
                for _ in 0..len {
                    seq.serialize_element(&inner)?;
                }
                seq.end()
            }
            TypeDef::FixedArray(fixed_array_def) => {
                let len = fixed_array_def.size as usize;
                let mut seq = serializer.serialize_seq(Some(len))?;
                let inner = fixed_array_def.type_def.to_de_se(data, cairo_se);
                for _ in 0..len {
                    seq.serialize_element(&inner)?;
                }
                seq.end()
            }
            TypeDef::Felt252Dict(_) => Err(S::Error::custom(
                "Felt252Dict transcoding is not implemented",
            )),
            TypeDef::Struct(struct_def) => {
                let mut map = serializer.serialize_map(Some(struct_def.members.len()))?;
                for member in &struct_def.members {
                    map.serialize_entry(&member.name, &member.type_def.to_de_se(data, cairo_se))?;
                }
                map.end()
            }
            TypeDef::Enum(enum_def) => {
                let selector = data.next_enum_variant().map_err(S::Error::custom)?;
                cairo_se.serialize_enum(data, serializer, enum_def, selector)
            }
            TypeDef::Ref(_) => Err(S::Error::custom(
                "TypeDef Ref needs to be expanded before serializing",
            )),
            TypeDef::Option(option_def) => {
                match data.next_option_is_some().map_err(S::Error::custom)? {
                    true => {
                        serializer.serialize_some(&option_def.type_def.to_de_se(data, cairo_se))
                    }
                    false => serializer.serialize_none(),
                }
            }
            TypeDef::Result(result_def) => {
                let is_ok = data.next_result_is_ok().map_err(S::Error::custom)?;
                cairo_se.serialize_result(data, serializer, result_def, is_ok)
            }
            TypeDef::Nullable(nullable_def) => {
                let is_null = data.next_nullable_is_null().map_err(S::Error::custom)?;
                match is_null {
                    true => serializer.serialize_none(),
                    false => {
                        serializer.serialize_some(&nullable_def.type_def.to_de_se(data, cairo_se))
                    }
                }
            }
        }
    }
}

impl<'a, C: CairoTypeSerialization> CairoSerialize<'a, C> for PrimaryTypeDef {
    fn serialize_cairo<S: Serializer>(
        &self,
        data: &mut impl CairoDeserializer,
        cairo_se: &C,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            PrimaryTypeDef::Felt252
            | PrimaryTypeDef::ClassHash
            | PrimaryTypeDef::ContractAddress
            | PrimaryTypeDef::StorageAddress
            | PrimaryTypeDef::StorageBaseAddress => {
                let value = data.next_felt_bytes().map_err(S::Error::custom)?;
                cairo_se.serialize_felt(serializer, &value)
            }
            PrimaryTypeDef::ShortUtf8 => {
                let value = data.next_short_string().map_err(S::Error::custom)?;
                serializer.serialize_str(&value)
            }
            PrimaryTypeDef::Bytes31 | PrimaryTypeDef::Bytes31Encoded(_) => {
                let value = data.next_bytes::<31>().map_err(S::Error::custom)?;
                serializer.serialize_bytes(&value)
            }
            PrimaryTypeDef::Bool => {
                let value = data.next_bool().map_err(S::Error::custom)?;
                serializer.serialize_bool(value)
            }
            PrimaryTypeDef::U8 => {
                let value = data.next_u8().map_err(S::Error::custom)?;
                serializer.serialize_u8(value)
            }
            PrimaryTypeDef::U16 => {
                let value = data.next_u16().map_err(S::Error::custom)?;
                serializer.serialize_u16(value)
            }
            PrimaryTypeDef::U32 => {
                let value = data.next_u32().map_err(S::Error::custom)?;
                serializer.serialize_u32(value)
            }
            PrimaryTypeDef::U64 => {
                let value = data.next_u64().map_err(S::Error::custom)?;
                serializer.serialize_u64(value)
            }
            PrimaryTypeDef::U128 => {
                let value = data.next_u128().map_err(S::Error::custom)?;
                serializer.serialize_u128(value)
            }
            PrimaryTypeDef::I8 => {
                let value = data.next_i8().map_err(S::Error::custom)?;
                serializer.serialize_i8(value)
            }
            PrimaryTypeDef::I16 => {
                let value = data.next_i16().map_err(S::Error::custom)?;
                serializer.serialize_i16(value)
            }
            PrimaryTypeDef::I32 => {
                let value = data.next_i32().map_err(S::Error::custom)?;
                serializer.serialize_i32(value)
            }
            PrimaryTypeDef::I64 => {
                let value = data.next_i64().map_err(S::Error::custom)?;
                serializer.serialize_i64(value)
            }
            PrimaryTypeDef::I128 => {
                let value = data.next_i128().map_err(S::Error::custom)?;
                serializer.serialize_i128(value)
            }
            PrimaryTypeDef::EthAddress => {
                let value = data.next_eth_address().map_err(S::Error::custom)?;
                cairo_se.serialize_eth_address(serializer, &value.0)
            }
        }
    }
}
