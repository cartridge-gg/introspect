use crate::decode_error::DecodeResultTrait;
use crate::deserialize::{CairoDeserialize, CairoDeserializer};
use crate::type_def::selectors;
use crate::utils::ideserialize_byte_array_with_last;
use crate::{
    ArrayDef, Attribute, ByteArray, ByteArrayEncodedDef, Bytes31EncodedDef, CairoISerde,
    CairoSerde, ColumnDef, CustomDef, DecodeError, DecodeResult, EnumDef, Felt252DictDef,
    FeltSource, FixedArrayDef, ItemDefTrait, MemberDef, NullableDef, OptionDef, PrimaryDef,
    PrimaryTypeDef, RefDef, ResultDef, StructDef, TupleDef, TypeDef, VariantDef,
};
use starknet_types_core::felt::Felt;

impl<F: FeltSource> TypeDefDeserializer for CairoISerde<F> {
    fn deserialize_attribute(&mut self) -> DecodeResult<Attribute> {
        let (name_bytes, info) = ideserialize_byte_array_with_last(self)?;
        let name = String::from_utf8_lossy(&name_bytes).into_owned();
        let data = if info & 0b100 != 0 {
            Some(self.next_byte_array().raise_eof()?.into())
        } else {
            None
        };
        Ok(Attribute { name, data })
    }
}

impl<F: FeltSource> TypeDefDeserializer for CairoSerde<F> {
    fn deserialize_attribute(&mut self) -> DecodeResult<Attribute> {
        let name = self.next_string()?;
        let data = self.next_option::<ByteArray>().raise_eof()?.map(Into::into);
        Ok(Attribute { name, data })
    }
}

pub trait TypeDefDeserializer: CairoDeserializer + Sized {
    fn deserialize_attribute(&mut self) -> DecodeResult<Attribute>;

    fn deserialize_type_def(&mut self) -> DecodeResult<TypeDef> {
        let selector = self.next_enum_variant()?;
        match selector.to_be_digits() {
            selectors::None => Ok(TypeDef::None),
            selectors::Felt252 => Ok(TypeDef::Felt252),
            selectors::ShortUtf8 => Ok(TypeDef::ShortUtf8),
            selectors::Bytes31 => Ok(TypeDef::Bytes31),
            selectors::Bytes31Encoded => Bytes31EncodedDef::deserialize_item(self),
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
            selectors::ByteArrayEncoded => ByteArrayEncodedDef::deserialize_item(self),
            selectors::Tuple => TupleDef::deserialize_item(self),
            selectors::Array => ArrayDef::deserialize_item(self),
            selectors::FixedArray => FixedArrayDef::deserialize_item(self),
            selectors::Felt252Dict => Felt252DictDef::deserialize_item(self),
            selectors::Struct => StructDef::deserialize_item(self),
            selectors::Enum => EnumDef::deserialize_item(self),
            selectors::Option => OptionDef::deserialize_item(self),
            selectors::Result => ResultDef::deserialize_item(self),
            selectors::Nullable => NullableDef::deserialize_item(self),
            selectors::Ref => RefDef::deserialize_item(self),
            selectors::Custom => CustomDef::deserialize_item(self),
            _ => Err(DecodeError::invalid_enum_selector("TypeDef", selector)),
        }
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for TypeDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        deserializer.deserialize_type_def()
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for Attribute {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        deserializer.deserialize_attribute()
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

impl<D: TypeDefDeserializer> CairoDeserialize<D> for TupleDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        deserializer.next_array().map(TupleDef::new)
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for ArrayDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        TypeDef::deserialize(deserializer).map(ArrayDef::new)
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for FixedArrayDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let element_type = TypeDef::deserialize(deserializer)?;
        let size = deserializer.next_u32()?;
        Ok(FixedArrayDef::new(element_type, size))
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for Felt252DictDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        TypeDef::deserialize(deserializer).map(Felt252DictDef::new)
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for OptionDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        TypeDef::deserialize(deserializer).map(OptionDef::new)
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for NullableDef {
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

impl<D: TypeDefDeserializer> CairoDeserialize<D> for ResultDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let ok_type = TypeDef::deserialize(deserializer)?;
        let err_type = TypeDef::deserialize(deserializer)?;
        Ok(ResultDef::new(ok_type, err_type))
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for StructDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>().raise_eof()?;
        let members = deserializer.next_array::<MemberDef>().raise_eof()?;
        Ok(StructDef::new(name, attributes, members))
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for EnumDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>().raise_eof()?;
        let variants = deserializer
            .next_array::<(Felt, VariantDef)>()
            .raise_eof()?;
        Ok(EnumDef::new(name, attributes, variants))
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for MemberDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>().raise_eof()?;
        let type_def = TypeDef::deserialize(deserializer).raise_eof()?;
        Ok(MemberDef::new(name, attributes, type_def))
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for VariantDef {
    fn deserialize(deserializer: &mut D) -> DecodeResult<Self> {
        let name = deserializer.next_string()?;
        let attributes = deserializer.next_array::<Attribute>().raise_eof()?;
        let type_def = TypeDef::deserialize(deserializer).raise_eof()?;
        Ok(VariantDef::new(name, attributes, type_def))
    }
}

impl<D: TypeDefDeserializer> CairoDeserialize<D> for ColumnDef {
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
