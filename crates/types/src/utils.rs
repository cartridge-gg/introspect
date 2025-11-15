use cainome_cairo_serde::{ByteArray, Bytes31};
use primitive_types::{U256, U512};
use starknet_types_core::felt::Felt;

pub type FeltIterator = dyn Iterator<Item = Felt>;

pub fn felt_to_string(value: &Felt) -> String {
    format!("0x{:016x}", value)
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
    let bytes = felt.to_bytes_be();
    let first = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    String::from_utf8(bytes[first..32].to_vec()).ok()
}

pub fn pop_bytes31(data: &mut FeltIterator) -> Option<[u8; 31]> {
    data.next()?.to_bytes_be()[1..32].try_into().ok()
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
