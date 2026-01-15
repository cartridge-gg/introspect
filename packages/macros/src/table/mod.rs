use crate::Ty;
use crate::ty::CairoCoreType;

pub mod column;
pub mod primary;
pub use column::ColumnDef;
pub use primary::{PrimaryDef, PrimaryTypeDefVariant};

impl Ty {
    pub fn is_primary_type(&self) -> bool {
        match self.get_core_type() {
            Some(
                CairoCoreType::Felt252
                | CairoCoreType::Bool
                | CairoCoreType::U8
                | CairoCoreType::U16
                | CairoCoreType::U32
                | CairoCoreType::U64
                | CairoCoreType::U128
                | CairoCoreType::I8
                | CairoCoreType::I16
                | CairoCoreType::I32
                | CairoCoreType::I64
                | CairoCoreType::I128
                | CairoCoreType::Bytes31
                | CairoCoreType::ClassHash
                | CairoCoreType::ContractAddress
                | CairoCoreType::EthAddress
                | CairoCoreType::StorageAddress
                | CairoCoreType::StorageBaseAddress,
            ) => true,
            _ => false,
        }
    }
}
