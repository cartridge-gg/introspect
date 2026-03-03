use crate::{CairoDeserializer, ResultDef, TypeDef, VariantDef};
use primitive_types::{U256, U512};
use serde::de::value;
use serde::ser::{
    Error as SerError, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};
use std::cell::RefCell;
#[cfg(not(any(feature = "std", feature = "alloc")))]
use std::fmt::Display;

pub struct SchemaSerialize<'a, D, C: CairoSerializer> {
    type_def: &'a TypeDef,
    de: &'a RefCell<&'a mut D>,
    cairo_serializer: &'a C,
}

pub struct CairoSerialization<'a, D, C: CairoSerializer> {
    de: &'a RefCell<&'a mut D>,
    cairo_se: &'a C,
}

impl<'a, D: CairoDeserializer, C: CairoSerializer> SchemaSerialize<'a, D, C> {
    pub fn new(type_def: &'a TypeDef, de: &'a RefCell<&'a mut D>, cairo_serializer: &'a C) -> Self {
        Self {
            type_def,
            de,
            cairo_serializer,
        }
    }
    pub fn to_type(&self, type_def: &'a TypeDef) -> SchemaSerialize<'a, D, C> {
        SchemaSerialize {
            type_def,
            de: self.de,
            cairo_serializer: self.cairo_serializer,
        }
    }

    fn with_de<T, E>(&self, op: impl FnOnce(&mut D) -> Result<T, E>) -> Result<T, E> {
        let mut de = self.de.borrow_mut();
        op(&mut *de)
    }

    fn serialize_byte_array<S: Serializer>(
        &self,
        serializer: S,
        value: &[u8],
    ) -> Result<S::Ok, S::Error> {
        self.cairo_serializer
            .serialize_byte_array(serializer, value)
    }

    fn serialize_felt<S: Serializer>(
        &self,
        serializer: S,
        value: &[u8; 32],
    ) -> Result<S::Ok, S::Error> {
        self.cairo_serializer.serialize_felt(serializer, value)
    }

    fn serialize_eth_address<S: Serializer>(
        &self,
        serializer: S,
        value: &[u8; 20],
    ) -> Result<S::Ok, S::Error> {
        self.cairo_serializer
            .serialize_eth_address(serializer, value)
    }

    fn serialize_u256<S: Serializer>(&self, serializer: S, value: U256) -> Result<S::Ok, S::Error> {
        self.cairo_serializer.serialize_u256(serializer, value)
    }

    fn serialize_u512<S: Serializer>(&self, serializer: S, value: U512) -> Result<S::Ok, S::Error> {
        self.cairo_serializer.serialize_u512(serializer, value)
    }

    fn serialize_enum<S: Serializer>(
        &self,
        serializer: S,
        variant_name: &str,
        type_def: &'a TypeDef,
    ) -> Result<S::Ok, S::Error> {
        self.cairo_serializer
            .serialize_enum(serializer, variant_name, type_def, self.de)
    }
}

pub trait CairoSerializer: Serializer {
    fn serialize_byte_array(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(value)
    }
    fn serialize_felt(self, value: &[u8; 32]) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(value)
    }
    fn serialize_eth_address(self, value: &[u8; 20]) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(value)
    }
    fn serialize_u256(self, value: U256) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&value.to_string())
    }
    fn serialize_u512(self, value: U512) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&value.to_string())
    }
    fn serialize_enum<T>(self, variant_name: &str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut map = self.serialize_map(Some(1))?;
        map.serialize_entry(variant_name, value)?;
        map.end()
    }
    fn serialize_result<T: Serialize>(
        self,
        is_okay: bool,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        match is_okay {
            true => self.serialize_enum("Ok", value),
            false => self.serialize_enum("Err", value),
        }
    }
}

pub struct SchemaSerializer<'a, D, T> {
    data: &'a mut D,
    schema: &'a T,
}

impl<'a, D, T> Serialize for SchemaSerializer<'a, D, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.schema
            .serialize_from(self.data, self)
            .map_err(S::Error::custom)
    }
}

pub struct CairoSe<T: Serializer>(pub T);
pub struct CairoVal<'a, T: Sized>(pub &'a T);

// impl<'a, T: Serialize> CairoVal<'a, T> {
//     pub fn cairo_serialize<S: Serializer>(
//         &self,
//         serializer: S,
//     ) -> Result<<CairoSe<S> as Serializer>::Ok, <CairoSe<S> as Serializer>::Error>
//     where
//         CairoSe<S>: Serializer<Ok = S::Ok, Error = S::Error>,
//     {
//         self.serialize(CairoSe(serializer))
//     }
// }

impl<'a, T> Serialize for CairoVal<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.cairo_serialize(CairoSe(serializer))
    }
}

pub trait CairoSerializeFrom<D, S: Serializer> {
    fn serialize_from(&self, data: &mut D, serializer: S) -> Result<S::Ok, S::Error>;
}

impl<'a, D: CairoDeserializer, S: CairoSerializer> CairoSerializeFrom<D, S> for TypeDef {
    fn serialize_from(&self, data: &mut D, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TypeDef::None => serializer.serialize_unit(),
            TypeDef::Felt252
            | TypeDef::ClassHash
            | TypeDef::ContractAddress
            | TypeDef::StorageAddress
            | TypeDef::StorageBaseAddress => {
                let value = data.next_felt_bytes().map_err(S::Error::custom)?;
                serializer.serialize_felt(&value)
            }
            TypeDef::ShortUtf8 => {
                let value = data.next_short_string().map_err(S::Error::custom)?;
                serializer.serialize_str(&value)
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
                serializer.serialize_u256(value)
            }
            TypeDef::U512 => {
                let value = data.next_u512().map_err(S::Error::custom)?;
                serializer.serialize_u512(value)
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
                serializer.serialize_eth_address(&value.0)
            }
            TypeDef::ByteArray | TypeDef::ByteArrayEncoded(_) | TypeDef::Custom(_) => {
                let value = data.next_byte_array_bytes().map_err(S::Error::custom)?;
                serializer.serialize_byte_array(&value)
            }
            TypeDef::Utf8String => {
                let value = data.next_string().map_err(S::Error::custom)?;
                serializer.serialize_str(&value)
            }
            TypeDef::Tuple(tuple) => {
                let mut seq = serializer.serialize_tuple(tuple.elements.len())?;
                for element in &tuple.elements {
                    seq.serialize_element(&self.to_type(element))?;
                }
                seq.end()
            }
            TypeDef::Array(array_def) => {
                let len = self.with_de(|de| de.next_u32()).map_err(S::Error::custom)? as usize;
                let mut seq = serializer.serialize_seq(Some(len))?;
                let inner = self.to_type(&array_def.type_def);
                for _ in 0..len {
                    seq.serialize_element(&inner)?;
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
                let VariantDef { name, type_def, .. } =
                    enum_def.get_variant(&selector).map_err(S::Error::custom)?;

                self.serialize_enum(serializer, name, type_def)
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
                    map.serialize_result
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

impl<'a, D: CairoDeserializer, C: CairoSerializer> Serialize for SchemaSerialize<'a, D, C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
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
                self.serialize_felt(serializer, &value)
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
                self.serialize_u256(serializer, value)
            }
            TypeDef::U512 => {
                let value = self
                    .with_de(|de| de.next_u512())
                    .map_err(S::Error::custom)?;
                self.serialize_u512(serializer, value)
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
                self.serialize_eth_address(serializer, &value.0)
            }
            TypeDef::ByteArray | TypeDef::ByteArrayEncoded(_) | TypeDef::Custom(_) => {
                let value = self
                    .with_de(|de| de.next_byte_array_bytes())
                    .map_err(S::Error::custom)?;
                self.serialize_byte_array(serializer, &value)
            }
            TypeDef::Utf8String => {
                let value = self
                    .with_de(|de| de.next_string())
                    .map_err(S::Error::custom)?;
                serializer.serialize_str(&value)
            }
            TypeDef::Tuple(tuple) => {
                let mut seq = serializer.serialize_tuple(tuple.elements.len())?;
                for element in &tuple.elements {
                    seq.serialize_element(&self.to_type(element))?;
                }
                seq.end()
            }
            TypeDef::Array(array_def) => {
                let len = self.with_de(|de| de.next_u32()).map_err(S::Error::custom)? as usize;
                let mut seq = serializer.serialize_seq(Some(len))?;
                let inner = self.to_type(&array_def.type_def);
                for _ in 0..len {
                    seq.serialize_element(&inner)?;
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
                let VariantDef { name, type_def, .. } =
                    enum_def.get_variant(&selector).map_err(S::Error::custom)?;

                self.serialize_enum(serializer, name, type_def)
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

impl<S: Serializer + CairoSerializer> Serializer for CairoSe<S>
where
    S::SerializeSeq: SerializeSeq + CairoSerializer,
    S::SerializeTuple: SerializeTuple + CairoSerializer,
    S::SerializeTupleStruct: SerializeTupleStruct + CairoSerializer,
    S::SerializeTupleVariant: SerializeTupleVariant + CairoSerializer,
    S::SerializeMap: SerializeMap + CairoSerializer,
    S::SerializeStruct: SerializeStruct + CairoSerializer,
    S::SerializeStructVariant: SerializeStructVariant + CairoSerializer,
    // CairoSe<S::SerializeSeq>:
    //     SerializeSeq<Ok = <S as Serializer>::Ok, Error = <S as Serializer>::Error>,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = CairoSe<S::SerializeSeq>;
    type SerializeTuple = CairoSe<S::SerializeTuple>;
    type SerializeTupleStruct = CairoSe<S::SerializeTupleStruct>;
    type SerializeTupleVariant = CairoSe<S::SerializeTupleVariant>;
    type SerializeMap = CairoSe<S::SerializeMap>;
    type SerializeStruct = CairoSe<S::SerializeStruct>;
    type SerializeStructVariant = CairoSe<S::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i64(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i128(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u128(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_some(value)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_newtype_struct(name, &CairoVal(value))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.serialize_seq(len).map(CairoSe)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.0.serialize_tuple(len).map(CairoSe)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.0.serialize_tuple_struct(name, len).map(CairoSe)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.0
            .serialize_tuple_variant(name, variant_index, variant, len)
            .map(CairoSe)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.0.serialize_map(len).map(CairoSe)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.0.serialize_struct(name, len).map(CairoSe)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.0
            .serialize_struct_variant(name, variant_index, variant, len)
            .map(CairoSe)
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        self.0.collect_seq(iter)
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        self.0.collect_map(iter)
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        self.0.collect_str(value)
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        self.0.collect_str(value)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.0.is_human_readable()
    }
}

impl<S: Serializer + CairoSerializer + SerializeSeq> SerializeSeq for CairoSe<S> {
    type Ok = <S as SerializeSeq>::Ok;
    type Error = <S as SerializeSeq>::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.end()
    }
}

impl<S: Serializer + CairoSerializer + SerializeTuple> SerializeTuple for CairoSe<S> {
    type Ok = <S as SerializeTuple>::Ok;
    type Error = <S as SerializeTuple>::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.end()
    }
}

impl<S: Serializer + CairoSerializer + SerializeTupleStruct> SerializeTupleStruct for CairoSe<S> {
    type Ok = <S as SerializeTupleStruct>::Ok;
    type Error = <S as SerializeTupleStruct>::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_field(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.end()
    }
}

impl<S: Serializer + CairoSerializer + SerializeTupleVariant> SerializeTupleVariant for CairoSe<S> {
    type Ok = <S as SerializeTupleVariant>::Ok;
    type Error = <S as SerializeTupleVariant>::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_field(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.end()
    }
}

impl<S: Serializer + CairoSerializer + SerializeMap> SerializeMap for CairoSe<S> {
    type Ok = <S as SerializeMap>::Ok;
    type Error = <S as SerializeMap>::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_key(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_value(value)
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        self.0.serialize_entry(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.end()
    }
}

impl<S: Serializer + CairoSerializer + SerializeStruct> SerializeStruct for CairoSe<S> {
    type Ok = <S as SerializeStruct>::Ok;
    type Error = <S as SerializeStruct>::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_field(key, value)
    }

    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.0.skip_field(key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.end()
    }
}

impl<S: Serializer + CairoSerializer + SerializeStructVariant> SerializeStructVariant
    for CairoSe<S>
{
    type Ok = <S as SerializeStructVariant>::Ok;
    type Error = <S as SerializeStructVariant>::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_field(key, value)
    }

    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.0.skip_field(key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.end()
    }
}
