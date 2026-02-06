use num_traits::{One, Zero};
use primitive_types::{U256, U512};
use starknet_types_core::felt::{Felt, PrimitiveFromFeltError};

use crate::{DecodeError, DecodeResult};

pub struct Bytes31(pub [u8; 31]);

impl TryFrom<Felt> for Bytes31 {
    type Error = DecodeError;
    fn try_from(felt: Felt) -> Result<Self, DecodeError> {
        felt_to_bytes31_bytes(felt).map(Bytes31)
    }
}

impl From<Bytes31> for [u8; 31] {
    fn from(bytes31: Bytes31) -> Self {
        bytes31.0
    }
}

pub fn felt_to_utf8_string(felt: Felt) -> DecodeResult<String> {
    let bytes = felt_to_bytes31_bytes(felt)?;
    let first = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    Ok(String::from_utf8_lossy(&bytes[first..]).into_owned())
}

pub fn felt_to_bytes31_bytes(felt: Felt) -> DecodeResult<[u8; 31]> {
    let [first, rest @ ..] = felt.to_bytes_be();
    match first {
        0 => Ok(rest),
        _ => Err(DecodeError::InvalidBytes31Encoding(
            "first byte must be zero for valid Bytes31 encoding",
        )),
    }
}

pub struct ByteArray(pub Vec<u8>);

impl From<Vec<u8>> for ByteArray {
    fn from(vec: Vec<u8>) -> Self {
        ByteArray(vec)
    }
}

impl From<ByteArray> for Vec<u8> {
    fn from(byte_array: ByteArray) -> Self {
        byte_array.0
    }
}

impl ByteArray {
    pub fn new_from_parts(full: Vec<Bytes31>, pending: [u8; 31], pending_len: u8) -> Self {
        let mut data = full.into_iter().flat_map(|b| b.0).collect::<Vec<u8>>();
        data.extend_from_slice(&pending[31 - pending_len as usize..]);
        ByteArray(data)
    }
}

impl From<ByteArray> for String {
    fn from(value: ByteArray) -> Self {
        value.to_string()
    }
}

impl ToString for ByteArray {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }
}

impl ToString for Bytes31 {
    fn to_string(&self) -> String {
        let first_non_zero = self.0.iter().position(|&b| b != 0).unwrap_or(self.0.len());
        String::from_utf8_lossy(&self.0[first_non_zero..]).into_owned()
    }
}

pub trait Bytes31Array {
    fn to_bytes(self) -> Vec<u8>;
}

impl Bytes31Array for Vec<Bytes31> {
    fn to_bytes(self) -> Vec<u8> {
        self.into_iter() // consumes Vec
            .flat_map(|b| b.0) // consumes each Bytes31
            .collect()
    }
}

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
    fn drain(&mut self) -> Vec<Felt> {
        let mut felts = Vec::new();
        while let Ok(felt) = self.next_felt() {
            felts.push(felt);
        }
        felts
    }
    fn next_felt(&mut self) -> DecodeResult<Felt>;
    fn next_bool_tag(&mut self, what: &'static str) -> DecodeResult<bool> {
        let f = self.next_felt()?;
        if f.is_zero() {
            Ok(false)
        } else if f.is_one() {
            Ok(true)
        } else {
            Err(DecodeError::invalid_tag(what, f))
        }
    }
    fn next_felt_bytes(&mut self) -> DecodeResult<[u8; 32]> {
        self.next_felt().map(|f| f.to_bytes_be())
    }
    fn next_bytes31(&mut self) -> DecodeResult<Bytes31> {
        self.next_felt()?.try_into()
    }
    fn next_bytes31_bytes(&mut self) -> DecodeResult<[u8; 31]> {
        self.next_felt().and_then(felt_to_bytes31_bytes)
    }
    fn next_short_string(&mut self) -> DecodeResult<String> {
        self.next_bytes31().map(|b| b.to_string())
    }
    fn next_digits(&mut self) -> DecodeResult<[u64; 4]> {
        self.next_felt().map(|f| f.to_be_digits())
    }
    fn next_bool(&mut self) -> DecodeResult<bool> {
        self.next_bool_tag("bool")
    }
    fn next_primitive<T>(&mut self) -> DecodeResult<T>
    where
        T: TryFrom<Felt, Error = PrimitiveFromFeltError>,
    {
        self.next_felt()?.to_primitive()
    }
    fn next_u8(&mut self) -> DecodeResult<u8> {
        self.next_primitive()
    }
    fn next_u16(&mut self) -> DecodeResult<u16> {
        self.next_primitive()
    }
    fn next_u32(&mut self) -> DecodeResult<u32> {
        self.next_primitive()
    }
    fn next_u64(&mut self) -> DecodeResult<u64> {
        self.next_primitive()
    }
    fn next_u128(&mut self) -> DecodeResult<u128> {
        self.next_primitive()
    }
    fn next_u256(&mut self) -> DecodeResult<U256> {
        match [self.next_digits()?, self.next_digits()?] {
            [[0, 0, l2, l1], [0, 0, h2, h1]] => Ok(U256([h2, h1, l2, l1])),
            _ => Err(DecodeError::InvalidEncoding { what: "u256" }),
        }
    }
    fn next_u512(&mut self) -> DecodeResult<U512> {
        match [
            self.next_digits()?,
            self.next_digits()?,
            self.next_digits()?,
            self.next_digits()?,
        ] {
            [
                [0, 0, l2, l1],
                [0, 0, l4, l3],
                [0, 0, h2, h1],
                [0, 0, h4, h3],
            ] => Ok(U512([h4, h3, h2, h1, l4, l3, l2, l1])),
            _ => Err(DecodeError::InvalidEncoding { what: "u512" }),
        }
    }
    fn next_i8(&mut self) -> DecodeResult<i8> {
        self.next_primitive()
    }
    fn next_i16(&mut self) -> DecodeResult<i16> {
        self.next_primitive()
    }
    fn next_i32(&mut self) -> DecodeResult<i32> {
        self.next_primitive()
    }
    fn next_i64(&mut self) -> DecodeResult<i64> {
        self.next_primitive()
    }
    fn next_i128(&mut self) -> DecodeResult<i128> {
        self.next_primitive()
    }
    fn next_byte_array(&mut self) -> DecodeResult<ByteArray>
    where
        Self: Sized,
    {
        Ok(ByteArray::new_from_parts(
            self.next_array::<Bytes31>()?,
            self.next_bytes31()?.into(),
            self.next_u8()?,
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
        T::deserialize_multiple(self, len as usize)
    }

    fn next_fixed_size_array<T: CairoDeserialize<Self>>(
        &mut self,
        size: usize,
    ) -> DecodeResult<Vec<T>>
    where
        Self: Sized,
    {
        T::deserialize_multiple(self, size)
    }
    fn next_option_is_some(&mut self) -> DecodeResult<bool> {
        self.next_bool_tag("option").map(|b| !b)
    }
    fn next_option<T: CairoDeserialize<Self>>(&mut self) -> DecodeResult<Option<T>>
    where
        Self: Sized,
    {
        if self.next_option_is_some()? {
            Ok(None)
        } else {
            T::deserialize(self).map(Some)
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
        if self.next_result_is_ok()? {
            T::deserialize(self).map(Ok)
        } else {
            E::deserialize(self).map(Err)
        }
    }
    fn next_nullable_is_null(&mut self) -> DecodeResult<bool> {
        self.next_bool_tag("nullable").map(|b| !b)
    }
    fn next_nullable<T: CairoDeserialize<Self>>(&mut self) -> DecodeResult<Option<T>>
    where
        Self: Sized,
    {
        if self.next_nullable_is_null()? {
            Ok(None)
        } else {
            T::deserialize(self).map(Some)
        }
    }
    fn next_const_size_array<const N: usize, T: CairoDeserialize<Self>>(
        &mut self,
    ) -> DecodeResult<[T; N]>
    where
        Self: Sized,
    {
        let mut v = Vec::with_capacity(N);
        for _ in 0..N {
            v.push(T::deserialize(self)?);
        }
        v.try_into()
            .map_err(|v: Vec<T>| DecodeError::unexpected_len("const size array", N, v.len()))
    }
}
