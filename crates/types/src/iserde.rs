use crate::type_def::selectors;
use crate::utils::{
    ideserialize_byte_array, ideserialize_byte_array_with_last, ideserialize_utf8_string,
};
use crate::{
    ArrayDef, Attribute, ByteArrayEDef, Bytes31EDef, ColumnDef, CustomDef, EnumDef, Felt252DictDef,
    FeltIterator, FixedArrayDef, ItemDefTrait, MemberDef, NullableDef, OptionDef, PrimaryDef,
    PrimaryTypeDef, RefDef, ResultDef, StructDef, TupleDef, TypeDef, VariantDef,
    deserialize_byte_array_string, pop_primitive,
};
use starknet_types_core::felt::Felt;

pub trait ISerde
where
    Self: Sized,
{
    fn ideserialize(data: &mut FeltIterator) -> Option<Self>;
    fn ideserialize_boxed(data: &mut FeltIterator) -> Option<Box<Self>> {
        Self::ideserialize(data).map(Box::new)
    }
}

pub trait ISerdeItem {
    fn ideserialize_item(data: &mut FeltIterator) -> Option<TypeDef>;
}

pub trait ISerdeEnd
where
    Self: Sized,
{
    fn ideserialize_end(data: &mut FeltIterator) -> Option<Vec<Self>>;
}

impl<Item: ItemDefTrait + ISerde> ISerdeItem for Item {
    fn ideserialize_item(data: &mut FeltIterator) -> Option<TypeDef> {
        Item::ideserialize(data).map(Item::wrap_to_type_def)
    }
}

impl<T: ISerde> ISerdeEnd for T {
    fn ideserialize_end(data: &mut FeltIterator) -> Option<Vec<Self>> {
        let mut items = Vec::new();
        while let Some(item) = T::ideserialize(data) {
            items.push(item);
        }
        Some(items)
    }
}

impl ISerde for Attribute {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let (name_bytes, info) = ideserialize_byte_array_with_last(data)?;
        let name = String::from_utf8(name_bytes).ok()?;
        let data = if info & 0b10000000 != 0 {
            Some(ideserialize_byte_array(data)?)
        } else {
            None
        };
        Some(Attribute { name, data })
    }
}

impl<T> ISerde for Vec<T>
where
    T: ISerde,
{
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        (0..pop_primitive::<usize>(data)?)
            .into_iter()
            .map(|_| T::ideserialize(data))
            .collect()
    }
}

impl ISerde for Felt {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        data.next()
    }
}

impl ISerde for TypeDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let selector = data.next()?.to_raw();
        match selector {
            selectors::None => Some(TypeDef::None),
            selectors::Felt252 => Some(TypeDef::Felt252),
            selectors::ShortUtf8 => Some(TypeDef::ShortUtf8),
            selectors::Bytes31 => Some(TypeDef::Bytes31),
            selectors::Bytes31E => Bytes31EDef::ideserialize_item(data),
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
            selectors::ByteArrayE => ByteArrayEDef::ideserialize_item(data),
            selectors::Tuple => TupleDef::ideserialize_item(data),
            selectors::Array => ArrayDef::ideserialize_item(data),
            selectors::FixedArray => FixedArrayDef::ideserialize_item(data),
            selectors::Felt252Dict => Felt252DictDef::ideserialize_item(data),
            selectors::Struct => StructDef::ideserialize_item(data),
            selectors::Enum => EnumDef::ideserialize_item(data),
            selectors::Option => OptionDef::ideserialize_item(data),
            selectors::Result => ResultDef::ideserialize_item(data),
            selectors::Nullable => NullableDef::ideserialize_item(data),
            selectors::Ref => RefDef::ideserialize_item(data),
            selectors::Custom => CustomDef::ideserialize_item(data),
            _ => None,
        }
    }
}

impl ISerde for StructDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let name = ideserialize_utf8_string(data)?;
        let attributes = Vec::<Attribute>::ideserialize(data)?;
        let members = Vec::<MemberDef>::ideserialize(data)?;
        Some(StructDef {
            name,
            attributes,
            members,
        })
    }
}

impl ISerde for MemberDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let name = ideserialize_utf8_string(data)?;
        let attributes = Vec::<Attribute>::ideserialize(data)?;
        let type_def = TypeDef::ideserialize(data)?;
        Some(MemberDef {
            name,
            attributes,
            type_def,
        })
    }
}

impl ISerde for EnumDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let name = ideserialize_utf8_string(data)?;
        let attributes = Vec::<Attribute>::ideserialize(data)?;
        let variants = Vec::<(Felt, VariantDef)>::ideserialize(data)?;
        Some(EnumDef::new(name, attributes, variants))
    }
}

impl ISerde for Bytes31EDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(Bytes31EDef {
            encoding: ideserialize_utf8_string(data)?,
        })
    }
}

impl ISerde for ByteArrayEDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(ByteArrayEDef {
            encoding: ideserialize_utf8_string(data)?,
        })
    }
}

impl ISerde for (Felt, VariantDef) {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let selector = data.next()?;
        let name = ideserialize_utf8_string(data)?;
        let attributes = Vec::<Attribute>::ideserialize(data)?;
        let type_def = TypeDef::ideserialize(data)?;
        Some((
            selector,
            VariantDef {
                name,
                attributes,
                type_def,
            },
        ))
    }
}

impl ISerde for TupleDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        Vec::<TypeDef>::ideserialize(data).map(TupleDef::new)
    }
}

impl ISerde for ArrayDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        TypeDef::ideserialize(data).map(ArrayDef::new)
    }
}

impl ISerde for Felt252DictDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        TypeDef::ideserialize(data).map(Felt252DictDef::new)
    }
}

impl ISerde for OptionDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        TypeDef::ideserialize(data).map(OptionDef::new)
    }
}

impl ISerde for NullableDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        TypeDef::ideserialize(data).map(NullableDef::new)
    }
}

impl ISerde for FixedArrayDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let type_def = TypeDef::ideserialize(data)?;
        let size = pop_primitive::<u32>(data)?;
        Some(FixedArrayDef { type_def, size })
    }
}

impl ISerde for ResultDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let ok = TypeDef::ideserialize(data)?;
        let err = TypeDef::ideserialize(data)?;
        Some(ResultDef { ok, err })
    }
}

impl ISerde for RefDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        pop_primitive::<Felt>(data).map(RefDef::new)
    }
}

impl ISerde for CustomDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        deserialize_byte_array_string(data).map(CustomDef::new)
    }
}

impl ISerde for ColumnDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let id = pop_primitive(data)?;
        let name = ideserialize_utf8_string(data)?;
        let attributes: Vec<Attribute> = Vec::<Attribute>::ideserialize(data)?;
        let type_def: TypeDef = TypeDef::ideserialize(data)?;
        Some(ColumnDef {
            id,
            name,
            attributes,
            type_def,
        })
    }
}

impl ISerde for PrimaryDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let name = ideserialize_utf8_string(data)?;
        let attributes: Vec<Attribute> = Vec::<Attribute>::ideserialize(data)?;
        let type_def: PrimaryTypeDef = PrimaryTypeDef::ideserialize(data)?;
        Some(PrimaryDef {
            name,
            attributes,
            type_def,
        })
    }
}

impl ISerde for PrimaryTypeDef {
    fn ideserialize(data: &mut FeltIterator) -> Option<Self> {
        let selector = data.next()?.to_raw();
        match selector {
            selectors::Felt252 => Some(PrimaryTypeDef::Felt252),
            selectors::ShortUtf8 => Some(PrimaryTypeDef::ShortUtf8),
            selectors::Bytes31 => Some(PrimaryTypeDef::Bytes31),
            selectors::Bytes31E => Some(PrimaryTypeDef::Bytes31E(ideserialize_utf8_string(data)?)),
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
