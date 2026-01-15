use starknet::storage_access::StorageBaseAddress;
use starknet::{ClassHash, ContractAddress, EthAddress, StorageAddress};
use crate::type_def::{SelectorTrait, selectors};
use crate::utils::SpanDefault;
use crate::{Attribute, ISerde};


#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct PrimaryDef {
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: PrimaryTypeDef,
}

#[derive(Drop, PartialEq, Default, Debug)]
pub enum PrimaryTypeDef {
    #[default]
    Felt252,
    ShortUtf8,
    Bytes31,
    Bytes31E: ByteArray,
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


impl PrimaryTypeDefSelector of SelectorTrait<PrimaryTypeDef> {
    const fn selector(self: @PrimaryTypeDef) -> felt252 {
        match self {
            PrimaryTypeDef::Felt252 => selectors::Felt252,
            PrimaryTypeDef::ShortUtf8 => selectors::ShortUtf8,
            PrimaryTypeDef::Bytes31 => selectors::Bytes31,
            PrimaryTypeDef::Bytes31E(_) => selectors::Bytes31E,
            PrimaryTypeDef::Bool => selectors::Bool,
            PrimaryTypeDef::U8 => selectors::U8,
            PrimaryTypeDef::U16 => selectors::U16,
            PrimaryTypeDef::U32 => selectors::U32,
            PrimaryTypeDef::U64 => selectors::U64,
            PrimaryTypeDef::U128 => selectors::U128,
            PrimaryTypeDef::I8 => selectors::I8,
            PrimaryTypeDef::I16 => selectors::I16,
            PrimaryTypeDef::I32 => selectors::I32,
            PrimaryTypeDef::I64 => selectors::I64,
            PrimaryTypeDef::I128 => selectors::I128,
            PrimaryTypeDef::ClassHash => selectors::ClassHash,
            PrimaryTypeDef::ContractAddress => selectors::ContractAddress,
            PrimaryTypeDef::EthAddress => selectors::EthAddress,
            PrimaryTypeDef::StorageAddress => selectors::StorageAddress,
            PrimaryTypeDef::StorageBaseAddress => selectors::StorageBaseAddress,
        }
    }
}

impl PrimaryTypeDefSerde of Serde<PrimaryTypeDef> {
    fn serialize(self: @PrimaryTypeDef, ref output: Array<felt252>) {
        output.append(self.selector());
        if let PrimaryTypeDef::Bytes31E(encoding) = self {
            encoding.serialize(ref output);
        }
    }
    fn deserialize(ref serialized: Span<felt252>) -> Option<PrimaryTypeDef> {
        let tag = *serialized.pop_front()?;
        if tag == selectors::Felt252 {
            Option::Some(PrimaryTypeDef::Felt252)
        } else if tag == selectors::ShortUtf8 {
            Option::Some(PrimaryTypeDef::ShortUtf8)
        } else if tag == selectors::Bytes31 {
            Option::Some(PrimaryTypeDef::Bytes31)
        } else if tag == selectors::Bytes31E {
            Option::Some(PrimaryTypeDef::Bytes31E(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Bool {
            Option::Some(PrimaryTypeDef::Bool)
        } else if tag == selectors::U8 {
            Option::Some(PrimaryTypeDef::U8)
        } else if tag == selectors::U16 {
            Option::Some(PrimaryTypeDef::U16)
        } else if tag == selectors::U32 {
            Option::Some(PrimaryTypeDef::U32)
        } else if tag == selectors::U64 {
            Option::Some(PrimaryTypeDef::U64)
        } else if tag == selectors::U128 {
            Option::Some(PrimaryTypeDef::U128)
        } else if tag == selectors::I8 {
            Option::Some(PrimaryTypeDef::I8)
        } else if tag == selectors::I16 {
            Option::Some(PrimaryTypeDef::I16)
        } else if tag == selectors::I32 {
            Option::Some(PrimaryTypeDef::I32)
        } else if tag == selectors::I64 {
            Option::Some(PrimaryTypeDef::I64)
        } else if tag == selectors::I128 {
            Option::Some(PrimaryTypeDef::I128)
        } else if tag == selectors::ClassHash {
            Option::Some(PrimaryTypeDef::ClassHash)
        } else if tag == selectors::ContractAddress {
            Option::Some(PrimaryTypeDef::ContractAddress)
        } else if tag == selectors::EthAddress {
            Option::Some(PrimaryTypeDef::EthAddress)
        } else if tag == selectors::StorageAddress {
            Option::Some(PrimaryTypeDef::StorageAddress)
        } else if tag == selectors::StorageBaseAddress {
            Option::Some(PrimaryTypeDef::StorageBaseAddress)
        } else {
            Option::None
        }
    }
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
        #[inline(always)]
        fn to_type_def() -> PrimaryTypeDef {
            Enum
        }
        #[inline(always)]
        fn to_felt252(self: @T) -> felt252 {
            (*self).into()
        }
    }
}

impl Felt252PrimaryImpl = tmp_impl::IPrimaryImpl<felt252, PrimaryTypeDef::Felt252>;
impl Bytes31PrimaryImpl = tmp_impl::IPrimaryImpl<bytes31, PrimaryTypeDef::ShortUtf8>;
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

pub impl PrimaryTypeDefISerde of ISerde<PrimaryTypeDef> {
    fn iserialize(self: @PrimaryTypeDef, ref output: Array<felt252>) {
        output.append(self.selector());
        if let PrimaryTypeDef::Bytes31E(encoding) = self {
            encoding.iserialize(ref output);
        }
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<PrimaryTypeDef> {
        let tag = *serialized.pop_front()?;
        if tag == selectors::Felt252 {
            Option::Some(PrimaryTypeDef::Felt252)
        } else if tag == selectors::ShortUtf8 {
            Option::Some(PrimaryTypeDef::ShortUtf8)
        } else if tag == selectors::Bytes31 {
            Option::Some(PrimaryTypeDef::Bytes31)
        } else if tag == selectors::Bytes31E {
            Option::Some(PrimaryTypeDef::Bytes31E(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Bool {
            Option::Some(PrimaryTypeDef::Bool)
        } else if tag == selectors::U8 {
            Option::Some(PrimaryTypeDef::U8)
        } else if tag == selectors::U16 {
            Option::Some(PrimaryTypeDef::U16)
        } else if tag == selectors::U32 {
            Option::Some(PrimaryTypeDef::U32)
        } else if tag == selectors::U64 {
            Option::Some(PrimaryTypeDef::U64)
        } else if tag == selectors::U128 {
            Option::Some(PrimaryTypeDef::U128)
        } else if tag == selectors::I8 {
            Option::Some(PrimaryTypeDef::I8)
        } else if tag == selectors::I16 {
            Option::Some(PrimaryTypeDef::I16)
        } else if tag == selectors::I32 {
            Option::Some(PrimaryTypeDef::I32)
        } else if tag == selectors::I64 {
            Option::Some(PrimaryTypeDef::I64)
        } else if tag == selectors::I128 {
            Option::Some(PrimaryTypeDef::I128)
        } else if tag == selectors::ClassHash {
            Option::Some(PrimaryTypeDef::ClassHash)
        } else if tag == selectors::ContractAddress {
            Option::Some(PrimaryTypeDef::ContractAddress)
        } else if tag == selectors::EthAddress {
            Option::Some(PrimaryTypeDef::EthAddress)
        } else if tag == selectors::StorageAddress {
            Option::Some(PrimaryTypeDef::StorageAddress)
        } else if tag == selectors::StorageBaseAddress {
            Option::Some(PrimaryTypeDef::StorageBaseAddress)
        } else {
            Option::None
        }
    }
}

pub impl PrimaryDefISerde of ISerde<PrimaryDef> {
    fn iserialize(self: @PrimaryDef, ref output: Array<felt252>) {
        self.name.iserialize(ref output);
        self.attributes.iserialize(ref output);
        self.type_def.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<PrimaryDef> {
        let name = ISerde::ideserialize(ref serialized)?;
        let attributes = ISerde::ideserialize(ref serialized)?;
        let type_def = PrimaryTypeDefISerde::ideserialize(ref serialized)?;
        Some(PrimaryDef { name, attributes, type_def })
    }
}
