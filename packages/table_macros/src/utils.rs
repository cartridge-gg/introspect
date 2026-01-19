use starknet_types_core::felt::Felt;
pub fn felt_to_hex_string(value: &Felt) -> String {
    format!("0x{:064x}", value)
}
