use crate::bytes::IntoByteSource;
use crate::{Bytes31, CairoDeserializer, DecodeResult, EthAddress};
use primitive_types::{U256, U512};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use starknet_types_core::felt::Felt;
use std::cell::RefCell;

pub struct CairoSeFrom<'a, 'de, T, D: CairoDeserializer, C: CairoSerialization> {
    schema: &'a T,
    de: &'de RefCell<&'de mut D>,
    cairo_se: &'a C,
}

pub fn cairo_serialize_from_source<'a, T, D, C, De, S>(
    schema: &'a T,
    to_de: De,
    cairo_se: &'a C,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    C: CairoSerialization,
    S: Serializer,
    De: IntoByteSource,
    De::Source: CairoDeserializer + 'a,
    for<'de> CairoSeFrom<'a, 'de, T, De::Source, C>: Serialize,
{
    let mut de = to_de.into_source();
    let de = RefCell::new(&mut de);
    let se_from = CairoSeFrom::new(schema, &de, cairo_se);
    se_from.serialize(serializer)
}

pub trait CairoDeserialization<D: CairoDeserializer> {
    fn with_de<T, E>(&self, op: impl FnOnce(&mut D) -> Result<T, E>) -> Result<T, E>;
    fn next_felt(&self) -> DecodeResult<Felt> {
        self.with_de(|de| de.next_felt())
    }
    fn next_byte(&self) -> DecodeResult<u8> {
        self.with_de(|de| de.next_byte())
    }
    fn next_bytes<const N: usize>(&self) -> DecodeResult<[u8; N]> {
        self.with_de(|de| de.next_bytes::<N>())
    }
    fn next_felt_bytes(&self) -> DecodeResult<[u8; 32]> {
        self.with_de(|de| de.next_felt_bytes())
    }
    fn next_bytes31(&self) -> DecodeResult<Bytes31> {
        self.with_de(|de| de.next_bytes31())
    }
    fn next_short_string(&self) -> DecodeResult<String> {
        self.with_de(|de| de.next_short_string())
    }
    fn next_bool(&self) -> DecodeResult<bool> {
        self.with_de(|de| de.next_bool())
    }
    fn next_u8(&self) -> DecodeResult<u8> {
        self.with_de(|de| de.next_u8())
    }
    fn next_u16(&self) -> DecodeResult<u16> {
        self.with_de(|de| de.next_u16())
    }
    fn next_u32(&self) -> DecodeResult<u32> {
        self.with_de(|de| de.next_u32())
    }
    fn next_u64(&self) -> DecodeResult<u64> {
        self.with_de(|de| de.next_u64())
    }
    fn next_u128(&self) -> DecodeResult<u128> {
        self.with_de(|de| de.next_u128())
    }
    fn next_limbs(&self) -> DecodeResult<[u64; 2]> {
        self.with_de(|de| de.next_limbs())
    }
    fn next_u256(&self) -> DecodeResult<U256> {
        self.with_de(|de| de.next_u256())
    }
    fn next_u512(&self) -> DecodeResult<U512> {
        self.with_de(|de| de.next_u512())
    }
    fn next_i8(&self) -> DecodeResult<i8> {
        self.with_de(|de| de.next_i8())
    }
    fn next_i16(&self) -> DecodeResult<i16> {
        self.with_de(|de| de.next_i16())
    }
    fn next_i32(&self) -> DecodeResult<i32> {
        self.with_de(|de| de.next_i32())
    }
    fn next_i64(&self) -> DecodeResult<i64> {
        self.with_de(|de| de.next_i64())
    }
    fn next_i128(&self) -> DecodeResult<i128> {
        self.with_de(|de| de.next_i128())
    }
    fn next_eth_address(&self) -> DecodeResult<EthAddress> {
        self.with_de(|de| de.next_eth_address())
    }
    fn next_byte_array_bytes(&self) -> DecodeResult<Vec<u8>>
    where
        Self: Sized,
    {
        self.with_de(|de| de.next_byte_array_bytes())
    }
    fn next_string(&self) -> DecodeResult<String>
    where
        Self: Sized,
    {
        self.with_de(|de| de.next_string())
    }
    fn next_enum_variant(&self) -> DecodeResult<Felt> {
        self.with_de(|de| de.next_enum_variant())
    }
    fn next_option_is_some(&self) -> DecodeResult<bool> {
        self.with_de(|de| de.next_option_is_some())
    }
    fn next_result_is_ok(&self) -> DecodeResult<bool> {
        self.with_de(|de| de.next_result_is_ok())
    }
    fn next_nullable_is_null(&self) -> DecodeResult<bool> {
        self.with_de(|de| de.next_nullable_is_null())
    }
}

impl<'a, D: CairoDeserializer> CairoDeserialization<D> for RefCell<&'a mut D> {
    fn with_de<T, E>(&self, op: impl FnOnce(&mut D) -> Result<T, E>) -> Result<T, E> {
        let mut de = self.borrow_mut();
        op(&mut *de)
    }
}

impl<'a, 'de, S, D: CairoDeserializer, C: CairoSerialization> CairoDeserialization<D>
    for CairoSeFrom<'a, 'de, S, D, C>
{
    fn with_de<T, E>(&self, op: impl FnOnce(&mut D) -> Result<T, E>) -> Result<T, E> {
        let mut de = self.de.borrow_mut();
        op(&mut *de)
    }
}

impl<'a, 'de, T, D: CairoDeserializer, C: CairoSerialization> CairoSeFrom<'a, 'de, T, D, C> {
    pub fn new(schema: &'a T, de: &'de RefCell<&'de mut D>, cairo_se: &'a C) -> Self {
        Self {
            de,
            cairo_se,
            schema,
        }
    }

    pub fn to_schema<S>(&self, schema: &'a S) -> CairoSeFrom<'a, 'de, S, D, C> {
        CairoSeFrom {
            de: self.de,
            cairo_se: self.cairo_se,
            schema,
        }
    }

    pub fn schema(&self) -> &'a T {
        self.schema
    }
}

pub trait CairoSerialization {
    fn serialize_byte_array<S: Serializer>(
        &self,
        serializer: S,
        value: &[u8],
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(value)
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
    fn serialize_enum<T, S: Serializer>(
        &self,
        serializer: S,
        variant_name: &str,
        value: &T,
    ) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(variant_name, value)?;
        map.end()
    }
    fn serialize_result_ok<T: Serialize, S: Serializer>(
        &self,
        serializer: S,
        value: &T,
    ) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
    {
        self.serialize_enum(serializer, "Ok", value)
    }
    fn serialize_result_err<T: Serialize, S: Serializer>(
        &self,
        serializer: S,
        value: &T,
    ) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
    {
        self.serialize_enum(serializer, "Err", value)
    }
}

impl<'a, 'de, T, D: CairoDeserializer, C: CairoSerialization> CairoSerialization
    for CairoSeFrom<'a, 'de, T, D, C>
{
    fn serialize_byte_array<S: Serializer>(
        &self,
        serializer: S,
        value: &[u8],
    ) -> Result<S::Ok, S::Error> {
        self.cairo_se.serialize_byte_array(serializer, value)
    }

    fn serialize_felt<S: Serializer>(
        &self,
        serializer: S,
        value: &[u8; 32],
    ) -> Result<S::Ok, S::Error> {
        self.cairo_se.serialize_felt(serializer, value)
    }

    fn serialize_eth_address<S: Serializer>(
        &self,
        serializer: S,
        value: &[u8; 20],
    ) -> Result<S::Ok, S::Error> {
        self.cairo_se.serialize_eth_address(serializer, value)
    }

    fn serialize_u256<S: Serializer>(&self, serializer: S, value: U256) -> Result<S::Ok, S::Error> {
        self.cairo_se.serialize_u256(serializer, value)
    }

    fn serialize_u512<S: Serializer>(&self, serializer: S, value: U512) -> Result<S::Ok, S::Error> {
        self.cairo_se.serialize_u512(serializer, value)
    }

    fn serialize_enum<V: Serialize, S: Serializer>(
        &self,
        serializer: S,
        variant_name: &str,
        value: &V,
    ) -> Result<S::Ok, S::Error> {
        self.cairo_se
            .serialize_enum(serializer, variant_name, value)
    }

    fn serialize_result_ok<V: Serialize, S: Serializer>(
        &self,
        serializer: S,
        value: &V,
    ) -> Result<S::Ok, S::Error> {
        self.cairo_se.serialize_result_ok(serializer, value)
    }
    fn serialize_result_err<V: Serialize, S: Serializer>(
        &self,
        serializer: S,
        value: &V,
    ) -> Result<S::Ok, S::Error> {
        self.cairo_se.serialize_result_err(serializer, value)
    }
}
