use num_bigint::BigUint;
use sha3::{Digest, Keccak256};
use starknet_types_core::felt::Felt;

pub fn string_to_keccak_hex(s: &str) -> String {
    format!("0x{}", starknet_keccak(s.as_bytes()).to_str_radix(16))
}

pub fn string_to_keccak_felt(s: &str) -> Felt {
    starknet_keccak(s.as_bytes()).into()
}

pub fn starknet_keccak(data: &[u8]) -> BigUint {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let mut result = hasher.finalize();

    // Truncate result to 250 bits.
    result[0] &= 3;
    BigUint::from_bytes_be(&result)
}
