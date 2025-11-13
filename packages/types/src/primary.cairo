use starknet::storage_access::StorageBaseAddress;
use starknet::{ClassHash, ContractAddress, EthAddress, StorageAddress};
use crate::Attribute;

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct PrimaryDef {
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: PrimaryTypeDef,
}

#[derive(Drop, Serde, PartialEq, Default, Debug)]
pub enum PrimaryTypeDef {
    #[default]
    Felt252,
    Bytes31,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    ClassHash,
    ContractAddress,
    EthAddress,
    StorageAddress,
    StorageBaseAddress,
}


pub trait PrimaryTrait<T> {
    fn to_type_def() -> PrimaryTypeDef;
    fn to_felt252(self: @T) -> felt252;
}
pub mod tmp_impl {
    use super::{PrimaryTrait, PrimaryTypeDef};

    pub impl IPrimaryImpl<
        T, const Enum: PrimaryTypeDef, +Copy<T>, +Into<T, felt252>,
    > of PrimaryTrait<T> {
        fn to_type_def() -> PrimaryTypeDef {
            Enum
        }
        fn to_felt252(self: @T) -> felt252 {
            (*self).into()
        }
    }
}

impl Felt252PrimaryImpl = tmp_impl::IPrimaryImpl<felt252, PrimaryTypeDef::Felt252>;
impl Bytes31PrimaryImpl = tmp_impl::IPrimaryImpl<bytes31, PrimaryTypeDef::Bytes31>;
impl BoolPrimaryImpl = tmp_impl::IPrimaryImpl<bool, PrimaryTypeDef::Bool>;
impl U8PrimaryImpl = tmp_impl::IPrimaryImpl<u8, PrimaryTypeDef::U8>;
impl U16PrimaryImpl = tmp_impl::IPrimaryImpl<u16, PrimaryTypeDef::U16>;
impl U32PrimaryImpl = tmp_impl::IPrimaryImpl<u32, PrimaryTypeDef::U32>;
impl U64PrimaryImpl = tmp_impl::IPrimaryImpl<u64, PrimaryTypeDef::U64>;
impl U128PrimaryImpl = tmp_impl::IPrimaryImpl<u128, PrimaryTypeDef::U128>;
impl I8PrimaryImpl = tmp_impl::IPrimaryImpl<i8, PrimaryTypeDef::I8>;
impl I16PrimaryImpl = tmp_impl::IPrimaryImpl<i16, PrimaryTypeDef::I16>;
impl I32PrimaryImpl = tmp_impl::IPrimaryImpl<i32, PrimaryTypeDef::I32>;
impl I64PrimaryImpl = tmp_impl::IPrimaryImpl<i64, PrimaryTypeDef::I64>;
impl I128PrimaryImpl = tmp_impl::IPrimaryImpl<i128, PrimaryTypeDef::I128>;
impl ClassHashPrimaryImpl = tmp_impl::IPrimaryImpl<ClassHash, PrimaryTypeDef::ClassHash>;
impl ContractAddressPrimaryImpl =
    tmp_impl::IPrimaryImpl<ContractAddress, PrimaryTypeDef::ContractAddress>;
impl EthAddressPrimaryImpl = tmp_impl::IPrimaryImpl<EthAddress, PrimaryTypeDef::EthAddress>;
impl StorageAddressPrimaryImpl =
    tmp_impl::IPrimaryImpl<StorageAddress, PrimaryTypeDef::StorageAddress>;
impl StorageBaseAddressPrimaryImpl =
    tmp_impl::IPrimaryImpl<StorageBaseAddress, PrimaryTypeDef::StorageBaseAddress>;

