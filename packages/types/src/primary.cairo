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
}

impl IPrimaryImpl<T, const Enum: PrimaryTypeDef> of PrimaryTrait<T> {
    fn to_type_def() -> PrimaryTypeDef {
        Enum
    }
}

impl Felt252PrimaryImpl = IPrimaryImpl<felt252, PrimaryTypeDef::Felt252>;
impl Bytes31PrimaryImpl = IPrimaryImpl<bytes31, PrimaryTypeDef::Bytes31>;
impl BoolPrimaryImpl = IPrimaryImpl<bool, PrimaryTypeDef::Bool>;
impl U8PrimaryImpl = IPrimaryImpl<u8, PrimaryTypeDef::U8>;
impl U16PrimaryImpl = IPrimaryImpl<u16, PrimaryTypeDef::U16>;
impl U32PrimaryImpl = IPrimaryImpl<u32, PrimaryTypeDef::U32>;
impl U64PrimaryImpl = IPrimaryImpl<u64, PrimaryTypeDef::U64>;
impl U128PrimaryImpl = IPrimaryImpl<u128, PrimaryTypeDef::U128>;
impl I8PrimaryImpl = IPrimaryImpl<i8, PrimaryTypeDef::I8>;
impl I16PrimaryImpl = IPrimaryImpl<i16, PrimaryTypeDef::I16>;
impl I32PrimaryImpl = IPrimaryImpl<i32, PrimaryTypeDef::I32>;
impl I64PrimaryImpl = IPrimaryImpl<i64, PrimaryTypeDef::I64>;
impl I128PrimaryImpl = IPrimaryImpl<i128, PrimaryTypeDef::I128>;
impl ClassHashPrimaryImpl = IPrimaryImpl<ClassHash, PrimaryTypeDef::ClassHash>;
impl ContractAddressPrimaryImpl = IPrimaryImpl<ContractAddress, PrimaryTypeDef::ContractAddress>;
impl EthAddressPrimaryImpl = IPrimaryImpl<EthAddress, PrimaryTypeDef::EthAddress>;
impl StorageAddressPrimaryImpl = IPrimaryImpl<StorageAddress, PrimaryTypeDef::StorageAddress>;
impl StorageBaseAddressPrimaryImpl =
    IPrimaryImpl<StorageBaseAddress, PrimaryTypeDef::StorageBaseAddress>;

