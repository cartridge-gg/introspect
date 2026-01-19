use crate::Ty;
use crate::ty::CairoPrimitiveType;

pub mod column;
pub mod primary;
pub use column::ColumnDef;
pub use primary::{PrimaryDef, PrimaryTypeDefVariant};

impl Ty {
    pub fn is_primary_type(&self) -> bool {
        match self.get_primitive_type() {
            Some(
                CairoPrimitiveType::Felt252
                | CairoPrimitiveType::Bool
                | CairoPrimitiveType::U8
                | CairoPrimitiveType::U16
                | CairoPrimitiveType::U32
                | CairoPrimitiveType::U64
                | CairoPrimitiveType::U128
                | CairoPrimitiveType::I8
                | CairoPrimitiveType::I16
                | CairoPrimitiveType::I32
                | CairoPrimitiveType::I64
                | CairoPrimitiveType::I128
                | CairoPrimitiveType::Bytes31
                | CairoPrimitiveType::ClassHash
                | CairoPrimitiveType::ContractAddress
                | CairoPrimitiveType::EthAddress
                | CairoPrimitiveType::StorageAddress
                | CairoPrimitiveType::StorageBaseAddress,
            ) => true,
            _ => false,
        }
    }
}
