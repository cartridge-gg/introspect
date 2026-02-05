use cairo_lang_starknet_classes::keccak::starknet_keccak;
use starknet_types_core::felt::Felt;

pub fn string_to_keccak_hex(s: &str) -> String {
    format!("0x{}", starknet_keccak(s.as_bytes()).to_str_radix(16))
}

pub fn string_to_keccak_felt(s: &str) -> Felt {
    starknet_keccak(s.as_bytes()).into()
}
