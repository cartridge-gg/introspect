use num_traits::Zero;
use primitive_types::{U256, U512};
use starknet_types_core::felt::Felt;

pub struct Bytes31(pub [u8; 31]);

impl From<Felt> for Bytes31 {
    fn from(felt: Felt) -> Self {
        let bytes = felt.to_bytes_be();
        let mut arr = [0u8; 31];
        arr.copy_from_slice(&bytes[1..]);
        Self(arr)
    }
}

impl From<Bytes31> for [u8; 31] {
    fn from(bytes31: Bytes31) -> Self {
        bytes31.0
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
    pub fn new_from_parts(full: Vec<Bytes31>, pending: [u8; 31], pending_len: usize) -> Self {
        let mut data = full.into_iter().flat_map(|b| b.0).collect::<Vec<u8>>();
        data.extend_from_slice(&pending[31 - pending_len..]);
        ByteArray(data)
    }
}

impl TryFrom<ByteArray> for String {
    type Error = std::string::FromUtf8Error;
    fn try_from(value: ByteArray) -> Result<Self, Self::Error> {
        String::from_utf8(value.0)
    }
}

impl ToString for ByteArray {
    fn to_string(&self) -> String {
        String::from_utf8(self.0.clone()).unwrap()
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

pub trait CairoDeserialize<D>
where
    Self: Sized,
{
    fn deserialize(deserializer: &mut D) -> Option<Self>;
    fn deserialize_multiple(deserializer: &mut D, count: usize) -> Option<Vec<Self>> {
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(Self::deserialize(deserializer)?);
        }
        Some(items)
    }
}

impl<D: CairoDeserializer, T: CairoDeserialize<D>> CairoDeserialize<D> for (Felt, T) {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        Some((deserializer.next_felt()?, T::deserialize(deserializer)?))
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for Felt {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_felt()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for Bytes31 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_bytes31()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for bool {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_bool()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for u8 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_u8()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for u16 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_u16()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for u32 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_u32()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for u64 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_u64()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for u128 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_u128()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for U256 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_u256()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for U512 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_u512()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for i8 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_i8()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for i16 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_i16()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for i32 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_i32()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for i64 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_i64()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for i128 {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_i128()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for ByteArray {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_byte_array()
    }
}

impl<D: CairoDeserializer, T> CairoDeserialize<D> for Vec<T>
where
    T: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.next_array()
    }
}

impl<T: CairoDeserializer> CairoDeserialize<T> for String {
    fn deserialize(deserializer: &mut T) -> Option<Self> {
        deserializer.next_string()
    }
}

pub trait CairoDeserializer {
    fn drain(&mut self) -> Vec<Felt> {
        let mut felts = Vec::new();
        while let Some(felt) = self.next_felt() {
            felts.push(felt);
        }
        felts
    }
    fn next_felt(&mut self) -> Option<Felt>;
    fn next_felt_bytes(&mut self) -> Option<[u8; 32]> {
        self.next_felt().map(|f| f.to_bytes_be())
    }
    fn next_bytes31(&mut self) -> Option<Bytes31> {
        self.next_primitive()
    }
    fn next_short_string(&mut self) -> Option<String> {
        self.next_bytes31().map(|b| b.to_string())
    }
    fn next_digits(&mut self) -> Option<[u64; 4]> {
        self.next_felt().map(|f| f.to_be_digits())
    }
    fn next_bool(&mut self) -> Option<bool> {
        self.next_felt().map(|f| !f.is_zero())
    }
    fn next_primitive<T: TryFrom<Felt>>(&mut self) -> Option<T> {
        self.next_felt()?.try_into().ok()
    }
    fn next_u8(&mut self) -> Option<u8> {
        self.next_primitive()
    }
    fn next_u16(&mut self) -> Option<u16> {
        self.next_primitive()
    }
    fn next_u32(&mut self) -> Option<u32> {
        self.next_primitive()
    }
    fn next_usize(&mut self) -> Option<usize> {
        self.next_u32().map(|u| u as usize)
    }
    fn next_u64(&mut self) -> Option<u64> {
        self.next_primitive()
    }
    fn next_u128(&mut self) -> Option<u128> {
        self.next_primitive()
    }
    fn next_u256(&mut self) -> Option<U256> {
        match [self.next_digits()?, self.next_digits()?] {
            [[0, 0, l2, l1], [0, 0, h2, h1]] => Some(U256([h2, h1, l2, l1])),
            _ => None,
        }
    }
    fn next_u512(&mut self) -> Option<U512> {
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
            ] => Some(U512([h4, h3, h2, h1, l4, l3, l2, l1])),
            _ => None,
        }
    }
    fn next_i8(&mut self) -> Option<i8> {
        self.next_primitive()
    }
    fn next_i16(&mut self) -> Option<i16> {
        self.next_primitive()
    }
    fn next_i32(&mut self) -> Option<i32> {
        self.next_primitive()
    }
    fn next_i64(&mut self) -> Option<i64> {
        self.next_primitive()
    }
    fn next_i128(&mut self) -> Option<i128> {
        self.next_primitive()
    }
    fn next_byte_array(&mut self) -> Option<ByteArray>
    where
        Self: Sized,
    {
        Some(ByteArray::new_from_parts(
            self.next_array::<Bytes31>()?,
            self.next_bytes31()?.into(),
            self.next_usize()?,
        ))
    }
    fn next_byte_array_bytes(&mut self) -> Option<Vec<u8>>
    where
        Self: Sized,
    {
        self.next_byte_array().map(Into::into)
    }
    fn next_string(&mut self) -> Option<String>
    where
        Self: Sized,
    {
        self.next_byte_array()?.try_into().ok()
    }

    fn next_array<T: CairoDeserialize<Self>>(&mut self) -> Option<Vec<T>>
    where
        Self: Sized,
    {
        let len = self.next_usize()?;
        T::deserialize_multiple(self, len)
    }

    fn next_fixed_size_array<T: CairoDeserialize<Self>>(&mut self, size: usize) -> Option<Vec<T>>
    where
        Self: Sized,
    {
        T::deserialize_multiple(self, size)
    }
    fn next_option_is_some(&mut self) -> Option<bool> {
        self.next_bool().map(|b| !b)
    }
    fn next_option<T: CairoDeserialize<Self>>(&mut self) -> Option<Option<T>>
    where
        Self: Sized,
    {
        if self.next_option_is_some()? {
            Some(None)
        } else {
            T::deserialize(self).map(Some)
        }
    }
    fn next_result_is_ok(&mut self) -> Option<bool> {
        self.next_bool().map(|b| !b)
    }
    fn next_result<T: CairoDeserialize<Self>, E: CairoDeserialize<Self>>(
        &mut self,
    ) -> Option<Result<T, E>>
    where
        Self: Sized,
    {
        if self.next_result_is_ok()? {
            T::deserialize(self).map(Ok)
        } else {
            E::deserialize(self).map(Err)
        }
    }
    fn next_nullable_is_null(&mut self) -> Option<bool> {
        self.next_bool().map(|b| !b)
    }
    fn next_nullable<T: CairoDeserialize<Self>>(&mut self) -> Option<Option<T>>
    where
        Self: Sized,
    {
        if self.next_nullable_is_null()? {
            Some(None)
        } else {
            T::deserialize(self).map(Some)
        }
    }
    fn next_const_size_array<const N: usize, T: CairoDeserialize<Self>>(&mut self) -> Option<[T; N]>
    where
        Self: Sized,
    {
        let arr = (0..N)
            .map(|_| T::deserialize(self))
            .collect::<Option<Vec<T>>>()?;
        arr.try_into().ok()
    }
}
