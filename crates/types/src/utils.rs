use cainome_cairo_serde::{ByteArray, Bytes31};
use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::MontgomeryConfigStark252PrimeField;
use lambdaworks_math::field::fields::montgomery_backed_prime_fields::MontgomeryBackendPrimeField;
use lambdaworks_math::unsigned_integer::element::UnsignedInteger;
use lambdaworks_math::unsigned_integer::montgomery::MontgomeryAlgorithms;
use primitive_types::{U256, U512};
use starknet_types_core::felt::Felt;

pub type FeltIterator = dyn Iterator<Item = Felt> + Send + Sync;

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

pub fn pop_primitive<T: TryFrom<Felt>>(data: &mut FeltIterator) -> Option<T> {
    data.next()?.try_into().ok()
}

pub fn read_serialized_felt_array(data: &mut FeltIterator) -> Option<Vec<Felt>> {
    let len = pop_primitive(data)?;
    (0..len)
        .into_iter()
        .map(|_| data.next())
        .collect::<Option<Vec<Felt>>>()
}

pub fn felt_to_utf8_string(felt: Felt) -> Option<String> {
    let bytes = felt_to_bytes31(felt)?;
    let first = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    String::from_utf8(bytes[first..].to_vec()).ok()
}

pub fn felt_to_bytes31(felt: Felt) -> Option<[u8; 31]> {
    let bytes = felt.to_bytes_be();
    match bytes[0] {
        0 => bytes[1..].try_into().ok(),
        _ => None,
    }
}

pub fn pop_bytes31(data: &mut FeltIterator) -> Option<[u8; 31]> {
    felt_to_bytes31(data.next()?)
}

pub fn pop_short_utf8(data: &mut FeltIterator) -> Option<String> {
    felt_to_utf8_string(data.next()?)
}

pub fn pop_u256(data: &mut FeltIterator) -> Option<U256> {
    match [
        pop_primitive::<Felt>(data)?.to_be_digits(),
        pop_primitive::<Felt>(data)?.to_be_digits(),
    ] {
        [[0, 0, l2, l1], [0, 0, h2, h1]] => Some(U256([h2, h1, l2, l1])),
        _ => None,
    }
}

pub fn pop_u512(data: &mut FeltIterator) -> Option<U512> {
    match [
        pop_primitive::<Felt>(data)?.to_be_digits(),
        pop_primitive::<Felt>(data)?.to_be_digits(),
        pop_primitive::<Felt>(data)?.to_be_digits(),
        pop_primitive::<Felt>(data)?.to_be_digits(),
    ] {
        [
            [0, 0, limb0h, limb0l],
            [0, 0, limb1h, limb1l],
            [0, 0, limb2h, limb2l],
            [0, 0, limb3h, limb3l],
        ] => Some(U512([
            limb3h, limb3l, limb2h, limb2l, limb1h, limb1l, limb0h, limb0l,
        ])),
        _ => None,
    }
}

pub fn deserialize_byte_array(data: &mut FeltIterator) -> Option<Vec<u8>> {
    let len = data.next()?.try_into().ok()?;

    let mut bytes: Vec<Bytes31> = Vec::with_capacity(len);
    for _ in 0..len {
        bytes.push(Bytes31::new(data.next()?).ok()?);
    }
    let pending_word = data.next()?;
    let pending_word_len = data.next()?.try_into().ok()?;

    Some(
        ByteArray {
            data: bytes,
            pending_word,
            pending_word_len,
        }
        .to_bytes(),
    )
}

pub fn deserialize_byte_array_string(data: &mut FeltIterator) -> Option<String> {
    deserialize_byte_array(data).map(|bytes| String::from_utf8_lossy(&bytes).to_string())
}

pub fn ideserialize_byte_array(data: &mut FeltIterator) -> Option<Vec<u8>> {
    ideserialize_byte_array_with_last(data).map(|(bytes, _)| bytes)
}

pub fn ideserialize_byte_array_with_last(data: &mut FeltIterator) -> Option<(Vec<u8>, u8)> {
    let mut bytes = Vec::new();
    loop {
        let felt_bytes = data.next()?.to_bytes_be();
        let info = felt_bytes[0];
        bytes.extend_from_slice(match info & 2 {
            0 => &felt_bytes[1..32],
            _ => &felt_bytes[(32 - felt_bytes[1] as usize)..32],
        });

        if info & 1 == 1 {
            return Some((bytes, info));
        }
    }
}

pub fn ideserialize_utf8_string(data: &mut FeltIterator) -> Option<String> {
    let byte_array = ideserialize_byte_array(data)?;
    String::from_utf8_lossy(&byte_array).into_owned().into()
}
