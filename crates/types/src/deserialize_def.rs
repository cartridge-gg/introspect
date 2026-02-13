use crate::decode_error::DecodeResultTrait;
use crate::deserialize::{CairoDeserialize, CairoDeserializer};
use crate::type_def::selectors;
use crate::{
    ArrayDef, Attribute, ByteArrayEncodedDef, Bytes31EncodedDef, ColumnDef, CustomDef, DecodeError,
    DecodeResult, EnumDef, Felt252DictDef, FixedArrayDef, ItemDefTrait, MemberDef, NullableDef,
    OptionDef, PrimaryDef, PrimaryTypeDef, RefDef, ResultDef, StructDef, TupleDef, TypeDef,
    VariantDef,
};
use starknet_types_core::felt::Felt;

impl<D: CairoDeserializer> CairoDeserialize<D> for TypeDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let selector = deserializer.next_felt()?;
        match selector.to_be_digits() {
            selectors::None => Ok(TypeDef::None),
            selectors::Felt252 => Ok(TypeDef::Felt252),
            selectors::ShortUtf8 => Ok(TypeDef::ShortUtf8),
            selectors::Bytes31 => Ok(TypeDef::Bytes31),
            selectors::Bytes31Encoded => Bytes31EncodedDef::deserialize_item(deserializer),
            selectors::Bool => Ok(TypeDef::Bool),
            selectors::U8 => Ok(TypeDef::U8),
            selectors::U16 => Ok(TypeDef::U16),
            selectors::U32 => Ok(TypeDef::U32),
            selectors::U64 => Ok(TypeDef::U64),
            selectors::U128 => Ok(TypeDef::U128),
            selectors::U256 => Ok(TypeDef::U256),
            selectors::U512 => Ok(TypeDef::U512),
            selectors::I8 => Ok(TypeDef::I8),
            selectors::I16 => Ok(TypeDef::I16),
            selectors::I32 => Ok(TypeDef::I32),
            selectors::I64 => Ok(TypeDef::I64),
            selectors::I128 => Ok(TypeDef::I128),
            selectors::ClassHash => Ok(TypeDef::ClassHash),
            selectors::ContractAddress => Ok(TypeDef::ContractAddress),
            selectors::EthAddress => Ok(TypeDef::EthAddress),
            selectors::StorageAddress => Ok(TypeDef::StorageAddress),
            selectors::StorageBaseAddress => Ok(TypeDef::StorageBaseAddress),
            selectors::ByteArray => Ok(TypeDef::ByteArray),
            selectors::Utf8String => Ok(TypeDef::Utf8String),
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
            _ => Err(DecodeError::invalid_enum_selector("TypeDef", selector)),
        }
    }
}

pub trait CairoDeserializeItemDef<D> {
    fn deserialize_item(deserializer: &mut D) -> DecodeResult<TypeDef>;
}

impl<Item: ItemDefTrait + CairoDeserialize<D>, D> CairoDeserializeItemDef<D> for Item {
    fn deserialize_item(deserializer: &mut D) -> DecodeResult<TypeDef> {
        Item::deserialize(deserializer)
            .raise_eof()
            .map(Item::wrap_to_type_def)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for ByteArrayEncodedDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        String::deserialize(deserializer).map(ByteArrayEncodedDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for Bytes31EncodedDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        String::deserialize(deserializer).map(Bytes31EncodedDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for TupleDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        deserializer.next_array().map(TupleDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for ArrayDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        TypeDef::deserialize(deserializer).map(ArrayDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for FixedArrayDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let element_type = TypeDef::deserialize(deserializer)?;
        let size = deserializer.next_u32()?;
        Ok(FixedArrayDef::new(element_type, size))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for Felt252DictDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        TypeDef::deserialize(deserializer).map(Felt252DictDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for OptionDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        TypeDef::deserialize(deserializer).map(OptionDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for NullableDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        TypeDef::deserialize(deserializer).map(NullableDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for RefDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        deserializer.next_felt().map(RefDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for CustomDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        deserializer.next_string().map(CustomDef::new)
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for ResultDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let ok_type = TypeDef::deserialize(deserializer)?;
        let err_type = TypeDef::deserialize(deserializer)?;
        Ok(ResultDef::new(ok_type, err_type))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for StructDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>().raise_eof()?;
        let members = deserializer.next_array::<MemberDef>().raise_eof()?;
        Ok(StructDef::new(name, attributes, members))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for EnumDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>().raise_eof()?;
        let variants = deserializer
            .next_array::<(Felt, VariantDef)>()
            .raise_eof()?;
        Ok(EnumDef::new(name, attributes, variants))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for MemberDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>().raise_eof()?;
        let type_def = TypeDef::deserialize(deserializer).raise_eof()?;
        Ok(MemberDef::new(name, attributes, type_def))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for VariantDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>().raise_eof()?;
        let type_def = TypeDef::deserialize(deserializer).raise_eof()?;
        Ok(VariantDef::new(name, attributes, type_def))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for ColumnDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let id = deserializer.next_felt()?;
        let name = deserializer.next_string().raise_eof()?;
        let attributes = deserializer.next_array::<Attribute>().raise_eof()?;
        let type_def = TypeDef::deserialize(deserializer).raise_eof()?;
        Ok(ColumnDef::new(id, name, attributes, type_def))
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for PrimaryTypeDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let selector = deserializer.next_felt()?;
        match selector.to_be_digits() {
            selectors::Felt252 => Ok(PrimaryTypeDef::Felt252),
            selectors::ShortUtf8 => Ok(PrimaryTypeDef::ShortUtf8),
            selectors::Bytes31 => Ok(PrimaryTypeDef::Bytes31),
            selectors::Bytes31Encoded => {
                Bytes31EncodedDef::deserialize(deserializer).map(PrimaryTypeDef::Bytes31Encoded)
            }
            selectors::Bool => Ok(PrimaryTypeDef::Bool),
            selectors::U8 => Ok(PrimaryTypeDef::U8),
            selectors::U16 => Ok(PrimaryTypeDef::U16),
            selectors::U32 => Ok(PrimaryTypeDef::U32),
            selectors::U64 => Ok(PrimaryTypeDef::U64),
            selectors::U128 => Ok(PrimaryTypeDef::U128),
            selectors::I8 => Ok(PrimaryTypeDef::I8),
            selectors::I16 => Ok(PrimaryTypeDef::I16),
            selectors::I32 => Ok(PrimaryTypeDef::I32),
            selectors::I64 => Ok(PrimaryTypeDef::I64),
            selectors::I128 => Ok(PrimaryTypeDef::I128),
            selectors::ClassHash => Ok(PrimaryTypeDef::ClassHash),
            selectors::ContractAddress => Ok(PrimaryTypeDef::ContractAddress),
            selectors::EthAddress => Ok(PrimaryTypeDef::EthAddress),
            selectors::StorageAddress => Ok(PrimaryTypeDef::StorageAddress),
            selectors::StorageBaseAddress => Ok(PrimaryTypeDef::StorageBaseAddress),
            _ => Err(DecodeError::invalid_enum_selector(
                "PrimaryTypeDef",
                selector,
            )),
        }
    }
}

impl<D: CairoDeserializer> CairoDeserialize<D> for PrimaryDef
where
    Attribute: CairoDeserialize<D>,
{
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>().raise_eof()?;
        let type_def = PrimaryTypeDef::deserialize(deserializer).raise_eof()?;
        Ok(PrimaryDef::new(name, attributes, type_def))
    }
}
