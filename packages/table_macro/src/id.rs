use crate::{TableResult, TableError};
use introspect_macros::utils::string_to_keccak_felt;
use introspect_types::ascii_str_to_felt;
use starknet_types_core::felt::Felt;

pub enum IdVariant {
    Felt(Felt),
    Const(String),
}

impl TryFrom<String> for IdVariant {
    type Error = TableError;
    fn try_from(arg: String) -> TableResult<Self> {
        if arg.starts_with("'") && arg.ends_with("'") {
            Ok(IdVariant::Felt(ascii_str_to_felt(&arg[1..arg.len() - 1])))
        } else if arg.starts_with("\"") && arg.ends_with("\"") {
            Ok(IdVariant::Felt(string_to_keccak_felt(
                &arg[1..arg.len() - 1],
            )))
        } else if arg.starts_with("0x") {
            Ok(IdVariant::Felt(
                Felt::from_hex(&arg).map_err(|_| TableError::ColumnIdParseError)?,
            ))
        } else if arg.starts_with("0b") {
            Err(TableError::ColumnIdParseError)
        } else if let Ok(felt) = Felt::from_dec_str(&arg) {
            Ok(IdVariant::Felt(felt))
        } else {
            Ok(IdVariant::Const(arg))
        }
    }
}
