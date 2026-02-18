use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::MontgomeryConfigStark252PrimeField;
use lambdaworks_math::field::fields::montgomery_backed_prime_fields::MontgomeryBackendPrimeField;
use lambdaworks_math::unsigned_integer::element::UnsignedInteger;
use lambdaworks_math::unsigned_integer::montgomery::MontgomeryAlgorithms;
use starknet_types_core::felt::Felt;

use crate::{DecodeResult, FeltSource};

// pub type FeltIterator = dyn Iterator<Item = Felt> + Send + Sync;

pub fn bytes31_to_hex_string<T: AsRef<[u8]>>(bytes: T) -> String {
    assert!(bytes.as_ref().len() == 31, "Input must be 31 bytes long");
    format!("0x{}", hex::encode(bytes))
}

pub fn felt_to_hex_string(value: &Felt) -> String {
    format!("0x{:064x}", value)
}

pub const fn ascii_str_to_felt(s: &str) -> Felt {
    Felt::from_raw(ascii_str_to_limbs(s))
}

pub const fn ascii_str_to_limbs(s: &str) -> [u64; 4] {
    pub const FELT_MODULUS: UnsignedInteger<4> = UnsignedInteger::<4>::from_hex_unchecked(
        "800000000000011000000000000000000000000000000000000000000000001",
    );
    const fn shift_u64_char(value: &mut u64, c: u64) -> u64 {
        let carry = *value >> 56;
        *value = (*value << 8) + c;
        carry
    }
    let mut limbs = [0; 4];
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut n = 0;
    while n < len {
        assert!(bytes[n] <= 0x7F, "Non-ASCII character in string");
        let carry = shift_u64_char(&mut limbs[3], bytes[n] as u64);
        let carry = shift_u64_char(&mut limbs[2], carry);
        let carry = shift_u64_char(&mut limbs[1], carry);
        shift_u64_char(&mut limbs[0], carry);
        n += 1;
    }
    assert!(n < 32, "String too long to convert to Felt");

    MontgomeryAlgorithms::cios(
        &UnsignedInteger { limbs },
        &MontgomeryBackendPrimeField::<MontgomeryConfigStark252PrimeField, 4>::R2,
        &FELT_MODULUS,
        &MontgomeryBackendPrimeField::<MontgomeryConfigStark252PrimeField, 4>::MU,
    )
    .limbs
}

pub fn ideserialize_byte_array<I: FeltSource>(data: &mut I) -> DecodeResult<Vec<u8>> {
    ideserialize_byte_array_with_last(data).map(|(bytes, _)| bytes)
}

pub fn ideserialize_byte_array_with_last<I: FeltSource>(
    data: &mut I,
) -> DecodeResult<(Vec<u8>, u8)> {
    let mut bytes = Vec::new();
    loop {
        let felt_bytes = data.next()?.to_bytes_be();
        let info = felt_bytes[0];
        bytes.extend_from_slice(match info & 2 {
            0 => &felt_bytes[1..32],
            _ => &felt_bytes[(32 - felt_bytes[1] as usize)..32],
        });

        if info & 1 == 1 {
            return Ok((bytes, info));
        }
    }
}

pub fn ideserialize_utf8_string<I: FeltSource>(data: &mut I) -> DecodeResult<String> {
    let byte_array = ideserialize_byte_array(data)?;
    Ok(String::from_utf8_lossy(&byte_array).into_owned())
}

pub trait ResultInto<T, E0> {
    fn map_into<U, E1, F>(self, op: F) -> Result<U, E1>
    where
        E0: Into<E1>,
        F: FnOnce(T) -> U;
}

impl<T, E0> ResultInto<T, E0> for Result<T, E0> {
    #[inline]
    fn map_into<U, E1, F>(self, op: F) -> Result<U, E1>
    where
        E0: Into<E1>,
        F: FnOnce(T) -> U,
    {
        match self {
            Ok(t) => Ok(op(t)),
            Err(e) => Err(e.into()),
        }
    }
}

pub fn string_to_cairo_serialize_byte_array(value: &str) -> Vec<Felt> {
    let (full, remainder) = value.as_bytes().as_chunks::<31>();
    let mut result = Vec::with_capacity(full.len() + 3);
    result.push(full.len().into());
    for f in full {
        result.push(Felt::from_bytes_be_slice(f));
    }
    result.push(Felt::from_bytes_be_slice(remainder));
    result.push(remainder.len().into());
    result
}

// pub trait Subarray<const N: usize, T> {
//     fn subarray<const START: usize, const SIZE: usize>(&self) -> [T; SIZE]
//     where
//         T: Copy,
//     {
//         *self.subarray_ref::<START, SIZE>()
//     }
//     fn subarray_ref<const START: usize, const SIZE: usize>(&self) -> &[T; SIZE];
//     // fn subarrays_ref<const SIZE: usize>(
//     //     &self,
//     // ) -> (
//     //     &[[T; SIZE]; <[T; N] as SubArrays<{ SIZE }>>::COUNT],
//     //     &[T; <[T; N] as SubArrays<{ SIZE }>>::REMAINDER],
//     // )
//     // where
//     //     [T; N]: SubArrays<{ SIZE }>;
// }

// trait SubArrays<const OUT: usize> {
//     type Item;
//     const COUNT: usize;
//     const REMAINDER: usize;
//     fn subarrays_ref<const SIZE: usize>(
//         &self,
//     ) -> (
//         &[[Self::Item; SIZE]; Self::COUNT],
//         &[Self::Item; Self::REMAINDER],
//     );
// }

// impl<const N: usize, const OUT: usize, T> SubArrays<OUT> for [T; N] {
//     const COUNT: usize = N / OUT;
//     const REMAINDER: usize = N % OUT;
//     type Item = T;
// }

// impl<const N: usize, T> Subarray<N, T> for [T; N] {
//     fn subarray_ref<const START: usize, const SIZE: usize>(&self) -> &[T; SIZE] {
//         const {
//             assert!(
//                 START <= N && SIZE <= N - START,
//                 "subarray_ref: START..START+SIZE out of bounds"
//             );
//         }
//         unsafe { &*(self.as_ptr().add(START) as *const [T; SIZE]) }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CairoDeserializer, CairoSerde};
    fn test_string_to_cairo_serialize_bytearray(value: &str) {
        let serialized = string_to_cairo_serialize_byte_array(value);
        let mut serde: CairoSerde<_> = serialized.into();
        let string = serde.next_string().unwrap();
        assert_eq!(string, value);
    }
    #[test]
    fn test_empty_string_serialization() {
        test_string_to_cairo_serialize_bytearray("");
    }
    #[test]
    fn test_len_1_string_serialization() {
        test_string_to_cairo_serialize_bytearray("a");
    }
    #[test]
    fn test_len_31_string_serialization() {
        test_string_to_cairo_serialize_bytearray("what is this? a center for ants");
    }

    #[test]
    fn test_len_32_string_serialization() {
        test_string_to_cairo_serialize_bytearray("this is a string with length 32!");
    }
}
