use crate::decode_error::DecodeResultTrait;
use crate::{ByteArray, Bytes31, DecodeError, DecodeResult, EthAddress};
use primitive_types::{U256, U512};
use starknet_types_core::felt::{Felt, PrimitiveFromFeltError};

pub trait PrimitiveFromFelt: Sized {
    fn primitive_from_felt(felt: Felt) -> DecodeResult<Self>;
}

pub trait FeltToPrimitive<T>: Sized {
    fn to_primitive(self) -> DecodeResult<T>;
}

impl<T> PrimitiveFromFelt for T
where
    T: TryFrom<Felt, Error = PrimitiveFromFeltError>,
{
    fn primitive_from_felt(felt: Felt) -> DecodeResult<Self> {
        T::try_from(felt).map_err(|_| DecodeError::PrimitiveFromFelt {
            what: core::any::type_name::<T>(),
        })
    }
}
impl<T> FeltToPrimitive<T> for Felt
where
    T: TryFrom<Felt, Error = PrimitiveFromFeltError>,
{
    fn to_primitive(self) -> DecodeResult<T> {
        T::try_from(self).map_err(|_| DecodeError::PrimitiveFromFelt {
            what: core::any::type_name::<T>(),
        })
    }
}

pub trait CairoDeserialize<D>
where
    Self: Sized,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self>;
    fn deserialize_multiple(deserializer: &mut D, count: usize) -> DecodeResult<Vec<Self>> {
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(Self::deserialize(deserializer)?);
        }
        Ok(items)
    }
}

impl<D: CairoDeserializer, T: CairoDeserialize<D>> CairoDeserialize<D> for (Felt, T) {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        Ok((deserializer.next_felt()?, T::deserialize(deserializer)?))
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for Felt {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_felt()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for Bytes31 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_bytes31()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for bool {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_bool()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for u8 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_u8()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for u16 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_u16()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for u32 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_u32()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for u64 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_u64()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for u128 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_u128()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for U256 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_u256()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for U512 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_u512()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for i8 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_i8()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for i16 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_i16()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for i32 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_i32()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for i64 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_i64()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for i128 {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_i128()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for EthAddress {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_eth_address()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for ByteArray {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_byte_array()
    }
}

impl<D: CairoDeserializer, T> CairoDeserialize<D> for Vec<T>
where
    T: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        deserializer.next_array()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for String {
    fn deserialize(deserializer: &mut T) -> DecodeResult<Self> {
        deserializer.next_string()
    }
}

pub trait CairoDeserializer {
    fn next_felt(&mut self) -> DecodeResult<Felt>;
    fn next_byte(&mut self) -> DecodeResult<u8> {
        self.next_bytes::<1>().map(|b| b[0])
    }
    fn next_bytes<const N: usize>(&mut self) -> DecodeResult<[u8; N]> {
        const {
            assert!(
                N <= 32,
                "next_bytes: N must be at most 32 since a felt is 32 bytes"
            );
        }
        let felt = self.next_felt()?;
        let bytes = felt.to_bytes_be();
        if bytes[..32 - N].iter().all(|&b| b == 0) {
            Ok(bytes[32 - N..].try_into().unwrap())
        } else {
            Err(DecodeError::NonZeroBytesInFelt(felt))
        }
    }
    fn next_value<T: CairoDeserialize<Self>>(&mut self) -> DecodeResult<T>
    where
        Self: Sized,
    {
        T::deserialize(self)
    }
    fn drain(&mut self) -> DecodeResult<Vec<Felt>> {
        let mut felts = Vec::new();
        loop {
            match self.next_felt() {
                Ok(felt) => felts.push(felt),
                Err(DecodeError::Eof) => break Ok(felts),
                Err(err) => break Err(err),
            }
        }
    }
    fn drain_values<T: CairoDeserialize<Self>>(&mut self) -> DecodeResult<Vec<T>>
    where
        Self: Sized,
    {
        let mut items = Vec::new();
        loop {
            match T::deserialize(self) {
                Ok(item) => items.push(item),
                Err(DecodeError::Eof) => break Ok(items),
                Err(err) => break Err(err),
            }
        }
    }
    fn next_bool_tag(&mut self, what: &'static str) -> DecodeResult<bool> {
        match self.next_byte() {
            Ok(0) => Ok(false),
            Ok(1) => Ok(true),
            Ok(other) => Err(DecodeError::invalid_tag(what, other)),
            Err(DecodeError::NonZeroBytesInFelt(felt)) => Err(DecodeError::invalid_tag(what, felt)),
            Err(err) => Err(err),
        }
    }
    fn next_felt_bytes(&mut self) -> DecodeResult<[u8; 32]> {
        self.next_bytes::<32>()
    }
    fn next_bytes31(&mut self) -> DecodeResult<Bytes31> {
        self.next_bytes::<31>().map(Bytes31)
    }
    fn next_short_string(&mut self) -> DecodeResult<String> {
        self.next_bytes31().map(|b| b.to_string())
    }
    fn next_bool(&mut self) -> DecodeResult<bool> {
        self.next_bool_tag("bool")
    }
    fn next_u8(&mut self) -> DecodeResult<u8> {
        self.next_bytes::<1>().map(|b| b[0])
    }
    fn next_u16(&mut self) -> DecodeResult<u16> {
        self.next_bytes::<2>().map(|b| u16::from_be_bytes(b))
    }
    fn next_u32(&mut self) -> DecodeResult<u32> {
        self.next_bytes::<4>().map(|b| u32::from_be_bytes(b))
    }
    fn next_u64(&mut self) -> DecodeResult<u64> {
        self.next_bytes::<8>().map(|b| u64::from_be_bytes(b))
    }
    fn next_u128(&mut self) -> DecodeResult<u128> {
        self.next_bytes::<16>().map(|b| u128::from_be_bytes(b))
    }
    fn next_limbs(&mut self) -> DecodeResult<[u64; 2]> {
        let bytes = self.next_bytes::<16>()?;
        let &[high, low] = (unsafe { bytes.as_chunks_unchecked::<8>() }) else {
            unreachable!()
        };
        Ok([u64::from_be_bytes(low), u64::from_be_bytes(high)])
    }
    fn next_u256(&mut self) -> DecodeResult<U256> {
        let [l1, l2] = self.next_limbs()?;
        let [h1, h2] = self
            .next_limbs()
            .map_eof(|| DecodeError::InvalidEncoding { what: "u256" })?;
        Ok(U256([h2, h1, l2, l1]))
    }
    fn next_u512(&mut self) -> DecodeResult<U512> {
        let [l1, l2] = self.next_limbs()?;
        let [l3, l4] = self
            .next_limbs()
            .map_eof(|| DecodeError::InvalidEncoding { what: "u512" })?;
        let [l5, l6] = self
            .next_limbs()
            .map_eof(|| DecodeError::InvalidEncoding { what: "u512" })?;
        let [l7, l8] = self
            .next_limbs()
            .map_eof(|| DecodeError::InvalidEncoding { what: "u512" })?;
        Ok(U512([l8, l7, l6, l5, l4, l3, l2, l1]))
    }
    fn next_i8(&mut self) -> DecodeResult<i8> {
        self.next_felt()?
            .try_into()
            .map_err(|_| DecodeError::PrimitiveFromFelt { what: "i8" })
    }
    fn next_i16(&mut self) -> DecodeResult<i16> {
        self.next_felt()?
            .try_into()
            .map_err(|_| DecodeError::PrimitiveFromFelt { what: "i16" })
    }
    fn next_i32(&mut self) -> DecodeResult<i32> {
        self.next_felt()?
            .try_into()
            .map_err(|_| DecodeError::PrimitiveFromFelt { what: "i32" })
    }
    fn next_i64(&mut self) -> DecodeResult<i64> {
        self.next_felt()?
            .try_into()
            .map_err(|_| DecodeError::PrimitiveFromFelt { what: "i64" })
    }
    fn next_i128(&mut self) -> DecodeResult<i128> {
        self.next_felt()?
            .try_into()
            .map_err(|_| DecodeError::PrimitiveFromFelt { what: "i128" })
    }
    fn next_eth_address(&mut self) -> DecodeResult<EthAddress> {
        self.next_bytes::<20>().map(Into::into)
    }
    fn next_byte_array(&mut self) -> DecodeResult<ByteArray>
    where
        Self: Sized,
    {
        Ok(ByteArray::new_from_parts(
            self.next_array::<Bytes31>()?,
            self.next_bytes31().raise_eof()?.into(),
            self.next_u8().raise_eof()?,
        ))
    }
    fn next_byte_array_bytes(&mut self) -> DecodeResult<Vec<u8>>
    where
        Self: Sized,
    {
        self.next_byte_array().map(Into::into)
    }
    fn next_string(&mut self) -> DecodeResult<String>
    where
        Self: Sized,
    {
        self.next_byte_array().map(Into::into)
    }

    fn next_array<T: CairoDeserialize<Self>>(&mut self) -> DecodeResult<Vec<T>>
    where
        Self: Sized,
    {
        let len = self.next_u32()?;
        T::deserialize_multiple(self, len as usize).raise_eof()
    }

    fn next_fixed_size_array<T: CairoDeserialize<Self>>(
        &mut self,
        size: usize,
    ) -> DecodeResult<Vec<T>>
    where
        Self: Sized,
    {
        T::deserialize_multiple(self, size).raise_eof()
    }
    fn next_enum_variant(&mut self) -> DecodeResult<Felt> {
        self.next_felt()
    }
    fn next_option_is_some(&mut self) -> DecodeResult<bool> {
        self.next_bool_tag("option").map(|b| !b)
    }
    fn next_option<T: CairoDeserialize<Self>>(&mut self) -> DecodeResult<Option<T>>
    where
        Self: Sized,
    {
        match self.next_option_is_some()? {
            true => T::deserialize(self).raise_eof().map(Some),
            false => Ok(None),
        }
    }
    fn next_result_is_ok(&mut self) -> DecodeResult<bool> {
        self.next_bool_tag("result").map(|b| !b)
    }
    fn next_result<T: CairoDeserialize<Self>, E: CairoDeserialize<Self>>(
        &mut self,
    ) -> DecodeResult<Result<T, E>>
    where
        Self: Sized,
    {
        match self.next_result_is_ok()? {
            true => T::deserialize(self).raise_eof().map(Ok),
            false => E::deserialize(self).raise_eof().map(Err),
        }
    }
    fn next_nullable_is_null(&mut self) -> DecodeResult<bool> {
        self.next_bool_tag("nullable").map(|b| !b)
    }
    fn next_nullable<T: CairoDeserialize<Self>>(&mut self) -> DecodeResult<Option<T>>
    where
        Self: Sized,
    {
        match self.next_nullable_is_null()? {
            true => Ok(None),
            false => T::deserialize(self).raise_eof().map(Some),
        }
    }
    fn next_const_size_array<const N: usize, T: CairoDeserialize<Self>>(
        &mut self,
    ) -> DecodeResult<[T; N]>
    where
        Self: Sized,
    {
        let mut v = Vec::with_capacity(N);
        if N > 0 {
            v.push(T::deserialize(self)?);
            for _ in 1..N {
                v.push(T::deserialize(self).raise_eof()?);
            }
        }

        v.try_into()
            .map_err(|v: Vec<T>| DecodeError::unexpected_len("const size array", N, v.len()))
    }
}
