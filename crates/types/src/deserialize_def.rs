use crate::deserialize::{CairoDeserialize, CairoDeserializer};
use crate::type_def::selectors;
use crate::{
    ArrayDef, Attribute, ByteArrayEncodedDef, Bytes31EncodedDef, ColumnDef, CustomDef, EnumDef,
    Felt252DictDef, FixedArrayDef, ItemDefTrait, MemberDef, NullableDef, OptionDef, PrimaryDef,
    PrimaryTypeDef, RefDef, ResultDef, StructDef, TupleDef, TypeDef, VariantDef,
};
use starknet_types_core::felt::Felt;

impl<D: CairoDeserializer> CairoDeserialize<D> for TypeDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let selector = deserializer.next_digits()?;
        match selector {
            selectors::None => Some(TypeDef::None),
            selectors::Felt252 => Some(TypeDef::Felt252),
            selectors::ShortUtf8 => Some(TypeDef::ShortUtf8),
            selectors::Bytes31 => Some(TypeDef::Bytes31),
            selectors::Bytes31Encoded => Bytes31EncodedDef::deserialize_item(deserializer),
            selectors::Bool => Some(TypeDef::Bool),
            selectors::U8 => Some(TypeDef::U8),
            selectors::U16 => Some(TypeDef::U16),
            selectors::U32 => Some(TypeDef::U32),
            selectors::U64 => Some(TypeDef::U64),
            selectors::U128 => Some(TypeDef::U128),
            selectors::U256 => Some(TypeDef::U256),
            selectors::U512 => Some(TypeDef::U512),
            selectors::I8 => Some(TypeDef::I8),
            selectors::I16 => Some(TypeDef::I16),
            selectors::I32 => Some(TypeDef::I32),
            selectors::I64 => Some(TypeDef::I64),
            selectors::I128 => Some(TypeDef::I128),
            selectors::ClassHash => Some(TypeDef::ClassHash),
            selectors::ContractAddress => Some(TypeDef::ContractAddress),
            selectors::EthAddress => Some(TypeDef::EthAddress),
            selectors::StorageAddress => Some(TypeDef::StorageAddress),
            selectors::StorageBaseAddress => Some(TypeDef::StorageBaseAddress),
            selectors::ByteArray => Some(TypeDef::ByteArray),
            selectors::Utf8String => Some(TypeDef::Utf8String),
            selectors::ByteArrayEncoded => ByteArrayEncodedDef::deserialize_item(deserializer),
            selectors::Tuple => TupleDef::deserialize_item(deserializer),
            selectors::Array => ArrayDef::deserialize_item(deserializer),
            selectors::FixedArray => FixedArrayDef::deserialize_item(deserializer),
            selectors::Felt252Dict => Felt252DictDef::deserialize_item(deserializer),
            selectors::Struct => StructDef::deserialize_item(deserializer),
            selectors::Enum => EnumDef::deserialize_item(deserializer),
            selectors::Option => OptionDef::deserialize_item(deserializer),
            selectors::Result => ResultDef::deserialize_item(deserializer),
            selectors::Nullable => NullableDef::deserialize_item(deserializer),
            selectors::Ref => RefDef::deserialize_item(deserializer),
            selectors::Custom => CustomDef::deserialize_item(deserializer),
            _ => None,
        }
    }
}

pub trait CairoDeserializeItemDef<D> {
    fn deserialize_item(deserializer: &mut D) -> Option<TypeDef>;
}

impl<Item: ItemDefTrait + CairoDeserialize<D>, D> CairoDeserializeItemDef<D> for Item {
    fn deserialize_item(deserializer: &mut D) -> Option<TypeDef> {
        
        Item::deserialize(deserializer).map(Item::wrap_to_type_def)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for ByteArrayEncodedDef {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        String::deserialize(deserializer).map(ByteArrayEncodedDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for Bytes31EncodedDef {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        String::deserialize(deserializer).map(Bytes31EncodedDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for TupleDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.next_array().map(TupleDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for ArrayDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        TypeDef::deserialize(deserializer).map(ArrayDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for FixedArrayDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let element_type = TypeDef::deserialize(deserializer)?;
        let size = deserializer.next_u32()?;
        Some(FixedArrayDef::new(element_type, size))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for Felt252DictDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        TypeDef::deserialize(deserializer).map(Felt252DictDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for OptionDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        TypeDef::deserialize(deserializer).map(OptionDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for NullableDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        TypeDef::deserialize(deserializer).map(NullableDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for RefDef {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.next_felt().map(RefDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for CustomDef {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.next_string().map(CustomDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for ResultDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let ok_type = TypeDef::deserialize(deserializer)?;
        let err_type = TypeDef::deserialize(deserializer)?;
        Some(ResultDef::new(ok_type, err_type))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for StructDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>()?;
        let members = deserializer.next_array::<MemberDef>()?;
        Some(StructDef::new(name, attributes, members))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for EnumDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>()?;
        let variants = deserializer.next_array::<(Felt, VariantDef)>()?;
        Some(EnumDef::new(name, attributes, variants))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for MemberDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>()?;
        let type_def = TypeDef::deserialize(deserializer)?;
        Some(MemberDef::new(name, attributes, type_def))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for VariantDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>()?;
        let type_def = TypeDef::deserialize(deserializer)?;
        Some(VariantDef::new(name, attributes, type_def))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for ColumnDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let id = deserializer.next_felt()?;
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>()?;
        let type_def = TypeDef::deserialize(deserializer)?;
        Some(ColumnDef::new(id, name, attributes, type_def))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for PrimaryTypeDef {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let selector = deserializer.next_digits()?;
        match selector {
            selectors::Felt252 => Some(PrimaryTypeDef::Felt252),
            selectors::ShortUtf8 => Some(PrimaryTypeDef::ShortUtf8),
            selectors::Bytes31 => Some(PrimaryTypeDef::Bytes31),
            selectors::Bytes31Encoded => {
                Bytes31EncodedDef::deserialize(deserializer).map(PrimaryTypeDef::Bytes31Encoded)
            }
            selectors::Bool => Some(PrimaryTypeDef::Bool),
            selectors::U8 => Some(PrimaryTypeDef::U8),
            selectors::U16 => Some(PrimaryTypeDef::U16),
            selectors::U32 => Some(PrimaryTypeDef::U32),
            selectors::U64 => Some(PrimaryTypeDef::U64),
            selectors::U128 => Some(PrimaryTypeDef::U128),
            selectors::I8 => Some(PrimaryTypeDef::I8),
            selectors::I16 => Some(PrimaryTypeDef::I16),
            selectors::I32 => Some(PrimaryTypeDef::I32),
            selectors::I64 => Some(PrimaryTypeDef::I64),
            selectors::I128 => Some(PrimaryTypeDef::I128),
            selectors::ClassHash => Some(PrimaryTypeDef::ClassHash),
            selectors::ContractAddress => Some(PrimaryTypeDef::ContractAddress),
            selectors::EthAddress => Some(PrimaryTypeDef::EthAddress),
            selectors::StorageAddress => Some(PrimaryTypeDef::StorageAddress),
            selectors::StorageBaseAddress => Some(PrimaryTypeDef::StorageBaseAddress),
            _ => None,
        }
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for PrimaryDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>()?;
        let type_def = PrimaryTypeDef::deserialize(deserializer)?;
        Some(PrimaryDef::new(name, attributes, type_def))
    }
}
