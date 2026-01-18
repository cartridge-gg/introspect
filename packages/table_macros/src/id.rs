use crate::{TableError, TableResult};
use introspect_macros::utils::string_to_keccak_hex;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdVariant {
    Felt(String),
    Const(String),
}

impl TryFrom<String> for IdVariant {
    type Error = TableError;
    fn try_from(string: String) -> TableResult<IdVariant> {
        if (string.starts_with("'") && string.ends_with("'"))
            && string.starts_with(|c: char| c.is_ascii_digit())
            && string.starts_with("0x")
            || string.starts_with("0b")
        {
            Ok(IdVariant::Felt(string))
        } else if string.starts_with("\"") && string.ends_with("\"") {
            Ok(IdVariant::Felt(string_to_keccak_hex(
                &string[1..string.len() - 1],
            )))
        } else {
            Ok(IdVariant::Const(string))
        }
    }
}

impl ToString for IdVariant {
    fn to_string(&self) -> String {
        match self {
            IdVariant::Felt(s) | IdVariant::Const(s) => s.clone(),
        }
    }
}

impl From<IdVariant> for String {
    fn from(id: IdVariant) -> Self {
        match id {
            IdVariant::Felt(s) | IdVariant::Const(s) => s,
        }
    }
}

pub trait IdVariantTrait {
    fn to_id_string(self, string: &str) -> String;
}

impl IdVariantTrait for Option<IdVariant> {
    fn to_id_string(self, string: &str) -> String {
        match self {
            Some(id) => id.into(),
            None => string_to_keccak_hex(string),
        }
    }
}
