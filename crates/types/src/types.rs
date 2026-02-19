use crate::utils::Subarray;
use crate::{DecodeError, DecodeResult};
use starknet_types_core::felt::Felt;
use std::ops::Deref;

const MAX_L1_ADDRESS: Felt = Felt::from_raw([
    461478224317121089,
    18446743936270598144,
    74766790688767,
    18406070939574861858,
]);

pub struct EthAddress(pub [u8; 20]);
pub struct Bytes31(pub [u8; 31]);
pub struct ByteArray(pub Vec<u8>);

impl TryFrom<Felt> for Bytes31 {
    type Error = DecodeError;
    fn try_from(felt: Felt) -> Result<Self, DecodeError> {
        felt_to_bytes31_bytes(felt).map(Bytes31)
    }
}

impl TryFrom<Felt> for EthAddress {
    type Error = DecodeError;
    fn try_from(felt: Felt) -> Result<Self, DecodeError> {
        TryFrom::try_from(&felt)
    }
}

impl TryFrom<&Felt> for EthAddress {
    type Error = DecodeError;
    fn try_from(felt: &Felt) -> Result<Self, DecodeError> {
        if felt <= &MAX_L1_ADDRESS {
            Ok(felt.to_bytes_be().subarray::<12, 20>().into())
        } else {
            Err(DecodeError::invalid_encoding(
                "Felt value exceeds maximum for ETH address",
            ))
        }
    }
}

impl From<&[u8; 20]> for EthAddress {
    fn from(bytes: &[u8; 20]) -> Self {
        EthAddress(*bytes)
    }
}

impl From<[u8; 20]> for EthAddress {
    fn from(bytes: [u8; 20]) -> Self {
        EthAddress(bytes)
    }
}

impl From<EthAddress> for [u8; 20] {
    fn from(address: EthAddress) -> Self {
        address.0
    }
}

impl From<Bytes31> for [u8; 31] {
    fn from(bytes31: Bytes31) -> Self {
        bytes31.0
    }
}

impl Deref for ByteArray {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl ByteArray {
    pub fn new_from_parts(full: Vec<Bytes31>, pending: [u8; 31], pending_len: u8) -> Self {
        assert!(
            pending_len <= 31,
            "pending_len must be at most 31 since pending is 31 bytes"
        );
        let mut data = full.into_iter().flat_map(|b| b.0).collect::<Vec<u8>>();
        data.extend_from_slice(&pending[31 - pending_len as usize..]);
        ByteArray(data)
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
