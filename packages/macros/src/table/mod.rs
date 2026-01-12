use crate::Ty;
use crate::ty::TyItem;

pub mod column;
pub mod primary;
pub use column::ColumnDef;
pub use primary::{PrimaryDef, PrimaryTypeDefVariant};

impl Ty {
    pub fn is_primary_type(&self) -> bool {
        match &self {
            Ty::Item(TyItem { name, params: None }) => ty_string_is_primary_type(name),
            _ => false,
        }
    }
}

pub fn ty_string_is_primary_type(type_str: &str) -> bool {
    matches!(
        type_str,
        "felt252"
            | "bool"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "bytes31"
            | "ClassHash"
            | "ContractAddress"
            | "EthAddress"
            | "StorageAddress"
            | "StorageBaseAddress"
    )
}
