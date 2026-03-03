use crate::serialize::{CairoDeserialization, CairoSeFrom, CairoSerialization};
use crate::{CairoDeserializer, TypeDef, VariantDef};
use serde::ser::{Error as SerError, SerializeMap, SerializeSeq, SerializeTuple};
use serde::{Serialize, Serializer};

impl<'a, D: CairoDeserializer, C: CairoSerialization> Serialize for CairoSeFrom<'a, D, C, TypeDef> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.schema() {
            TypeDef::None => serializer.serialize_unit(),
            TypeDef::Felt252
            | TypeDef::ClassHash
            | TypeDef::ContractAddress
            | TypeDef::StorageAddress
            | TypeDef::StorageBaseAddress => {
                let value = self.next_felt_bytes().map_err(S::Error::custom)?;
                self.serialize_felt(serializer, &value)
            }
            TypeDef::ShortUtf8 => {
                let value = self.next_short_string().map_err(S::Error::custom)?;
                serializer.serialize_str(&value)
            }
            TypeDef::Bytes31 | TypeDef::Bytes31Encoded(_) => {
                let value = self.next_bytes::<31>().map_err(S::Error::custom)?;
                serializer.serialize_bytes(&value)
            }
            TypeDef::Bool => {
                let value = self.next_bool().map_err(S::Error::custom)?;
                serializer.serialize_bool(value)
            }
            TypeDef::U8 => {
                let value = self.next_u8().map_err(S::Error::custom)?;
                serializer.serialize_u8(value)
            }
            TypeDef::U16 => {
                let value = self.next_u16().map_err(S::Error::custom)?;
                serializer.serialize_u16(value)
            }
            TypeDef::U32 => {
                let value = self.next_u32().map_err(S::Error::custom)?;
                serializer.serialize_u32(value)
            }
            TypeDef::U64 => {
                let value = self.next_u64().map_err(S::Error::custom)?;
                serializer.serialize_u64(value)
            }
            TypeDef::U128 => {
                let value = self.next_u128().map_err(S::Error::custom)?;
                serializer.serialize_u128(value)
            }
            TypeDef::U256 => {
                let value = self.next_u256().map_err(S::Error::custom)?;
                self.serialize_u256(serializer, value)
            }
            TypeDef::U512 => {
                let value = self.next_u512().map_err(S::Error::custom)?;
                self.serialize_u512(serializer, value)
            }
            TypeDef::I8 => {
                let value = self.next_i8().map_err(S::Error::custom)?;
                serializer.serialize_i8(value)
            }
            TypeDef::I16 => {
                let value = self.next_i16().map_err(S::Error::custom)?;
                serializer.serialize_i16(value)
            }
            TypeDef::I32 => {
                let value = self.next_i32().map_err(S::Error::custom)?;
                serializer.serialize_i32(value)
            }
            TypeDef::I64 => {
                let value = self.next_i64().map_err(S::Error::custom)?;
                serializer.serialize_i64(value)
            }
            TypeDef::I128 => {
                let value = self.next_i128().map_err(S::Error::custom)?;
                serializer.serialize_i128(value)
            }
            TypeDef::EthAddress => {
                let value = self.next_eth_address().map_err(S::Error::custom)?;
                self.serialize_eth_address(serializer, &value.0)
            }
            TypeDef::ByteArray | TypeDef::ByteArrayEncoded(_) | TypeDef::Custom(_) => {
                let value = self.next_byte_array_bytes().map_err(S::Error::custom)?;
                self.serialize_byte_array(serializer, &value)
            }
            TypeDef::Utf8String => {
                let value = self.next_string().map_err(S::Error::custom)?;
                serializer.serialize_str(&value)
            }
            TypeDef::Tuple(tuple) => {
                let mut seq = serializer.serialize_tuple(tuple.elements.len())?;
                for element in &tuple.elements {
                    seq.serialize_element(&self.to_schema(element))?;
                }
                seq.end()
            }
            TypeDef::Array(array_def) => {
                let len = self.next_u32().map_err(S::Error::custom)? as usize;
                let mut seq = serializer.serialize_seq(Some(len))?;
                let inner = self.to_schema(&array_def.type_def);
                for _ in 0..len {
                    seq.serialize_element(&inner)?;
                }
                seq.end()
            }
            TypeDef::FixedArray(fixed_array_def) => {
                let len = fixed_array_def.size as usize;
                let mut seq = serializer.serialize_seq(Some(len))?;
                for _ in 0..len {
                    seq.serialize_element(&self.to_schema(&fixed_array_def.type_def))?;
                }
                seq.end()
            }
            TypeDef::Felt252Dict(_) => Err(S::Error::custom(
                "Felt252Dict transcoding is not implemented",
            )),
            TypeDef::Struct(struct_def) => {
                let mut map = serializer.serialize_map(Some(struct_def.members.len()))?;
                for member in &struct_def.members {
                    map.serialize_entry(&member.name, &self.to_schema(&member.type_def))?;
                }
                map.end()
            }
            TypeDef::Enum(enum_def) => {
                let selector = self.next_enum_variant().map_err(S::Error::custom)?;
                let VariantDef { name, type_def, .. } =
                    enum_def.get_variant(&selector).map_err(S::Error::custom)?;

                self.serialize_enum(serializer, name, type_def)
            }
            TypeDef::Ref(_) => Err(S::Error::custom(
                "TypeDef Ref needs to be expanded before serializing",
            )),
            TypeDef::Option(option_def) => {
                match self.next_option_is_some().map_err(S::Error::custom)? {
                    true => serializer.serialize_some(&self.to_schema(&option_def.type_def)),
                    false => serializer.serialize_none(),
                }
            }
            TypeDef::Result(result_def) => {
                let is_ok = self.next_result_is_ok().map_err(S::Error::custom)?;
                match is_ok {
                    true => self.serialize_result_ok(serializer, &self.to_schema(&result_def.ok)),
                    false => {
                        self.serialize_result_err(serializer, &self.to_schema(&result_def.err))
                    }
                }
            }
            TypeDef::Nullable(nullable_def) => {
                let is_null = self
                    .with_de(|de| de.next_nullable_is_null())
                    .map_err(S::Error::custom)?;
                match is_null {
                    true => serializer.serialize_none(),
                    false => serializer.serialize_some(&self.to_schema(&nullable_def.type_def)),
                }
            }
        }
    }
}
