use crate::type_def::{ByteArrayEDef, selectors};
use crate::{
    ArrayDef, Attribute, ByteArrayDeserialization, ColumnDef, CustomDef, EnumDef, Felt252DictDef,
    FeltIterator, FixedArrayDef, ItemDefTrait, MemberDef, NullableDef, OptionDef, PrimaryDef,
    PrimaryTypeDef, RefDef, ResultDef, StructDef, TupleDef, TypeDef, VariantDef,
    deserialize_byte_array_string, pop_primitive,
};
use starknet_types_core::felt::Felt;

pub trait CairoDeserialize
where
    Self: Sized,
{
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self>;
    fn c_deserialize_boxed(data: &mut FeltIterator) -> Option<Box<Self>> {
        Self::c_deserialize(data).map(Box::new)
    }
}

trait CairoDeserializeItem {
    fn c_deserialize_item(data: &mut FeltIterator) -> Option<TypeDef>;
}

impl<Item: ItemDefTrait + CairoDeserialize> CairoDeserializeItem for Item {
    fn c_deserialize_item(data: &mut FeltIterator) -> Option<TypeDef> {
        Item::c_deserialize(data).map(Item::wrap_to_type_def)
    }
}

impl CairoDeserialize for Attribute {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let id = data.next()?;
        let data = Vec::<Felt>::c_deserialize(data)?;
        Some(Attribute { id, data })
    }
}

impl<T> CairoDeserialize for Vec<T>
where
    T: CairoDeserialize,
{
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        (0..pop_primitive::<usize>(data)?)
            .into_iter()
            .map(|_| T::c_deserialize(data))
            .collect()
    }
}

impl CairoDeserialize for Felt {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        data.next()
    }
}

impl CairoDeserialize for TypeDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let selector = data.next()?.to_raw();
        match selector {
            selectors::None => Some(TypeDef::None),
            selectors::Felt252 => Some(TypeDef::Felt252),
            selectors::ShortUtf8 => Some(TypeDef::ShortUtf8),
            selectors::Bytes31 => Some(TypeDef::Bytes31),
            selectors::Bytes31E => Some(TypeDef::Bytes31E(data.next()?)),
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
            selectors::ByteArray => Some(TypeDef::ByteArray(ByteArrayDeserialization::ISerde)),
            selectors::Utf8String => Some(TypeDef::Utf8String(ByteArrayDeserialization::ISerde)),
            selectors::ByteArrayE => ByteArrayEDef::c_deserialize_item(data),
            selectors::Tuple => TupleDef::c_deserialize_item(data),
            selectors::Array => ArrayDef::c_deserialize_item(data),
            selectors::FixedArray => FixedArrayDef::c_deserialize_item(data),
            selectors::Felt252Dict => Felt252DictDef::c_deserialize_item(data),
            selectors::Struct => StructDef::c_deserialize_item(data),
            selectors::Enum => EnumDef::c_deserialize_item(data),
            selectors::Option => OptionDef::c_deserialize_item(data),
            selectors::Result => ResultDef::c_deserialize_item(data),
            selectors::Nullable => NullableDef::c_deserialize_item(data),
            selectors::Ref => RefDef::c_deserialize_item(data),
            selectors::Custom => CustomDef::c_deserialize_item(data),
            _ => None,
        }
    }
}

impl CairoDeserialize for StructDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let name = deserialize_byte_array_string(data)?;
        let attributes = Vec::<Attribute>::c_deserialize(data)?;
        let members = Vec::<MemberDef>::c_deserialize(data)?;
        Some(StructDef {
            name,
            attributes,
            members,
        })
    }
}

impl CairoDeserialize for MemberDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let name = deserialize_byte_array_string(data)?;
        let attributes = Vec::<Attribute>::c_deserialize(data)?;
        let type_def = TypeDef::c_deserialize(data)?;
        Some(MemberDef {
            name,
            attributes,
            type_def,
        })
    }
}

impl CairoDeserialize for EnumDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let name = deserialize_byte_array_string(data)?;
        let attributes = Vec::<Attribute>::c_deserialize(data)?;
        let variants = Vec::<(Felt, VariantDef)>::c_deserialize(data)?;
        Some(EnumDef::new(name, attributes, variants))
    }
}

impl CairoDeserialize for ByteArrayEDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(ByteArrayEDef {
            mode: ByteArrayDeserialization::ISerde,
            encoding: data.next()?,
        })
    }
}

impl CairoDeserialize for (Felt, VariantDef) {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let selector = data.next()?;
        let name = deserialize_byte_array_string(data)?;
        let attributes = Vec::<Attribute>::c_deserialize(data)?;
        let type_def = TypeDef::c_deserialize(data)?;
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

impl CairoDeserialize for TupleDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(TupleDef {
            elements: Vec::<TypeDef>::c_deserialize(data)?,
        })
    }
}

impl CairoDeserialize for ArrayDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(ArrayDef {
            type_def: TypeDef::c_deserialize(data)?,
        })
    }
}

impl CairoDeserialize for Felt252DictDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(Felt252DictDef {
            type_def: TypeDef::c_deserialize(data)?,
        })
    }
}

impl CairoDeserialize for OptionDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(OptionDef {
            type_def: TypeDef::c_deserialize(data)?,
        })
    }
}

impl CairoDeserialize for NullableDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(NullableDef {
            type_def: TypeDef::c_deserialize(data)?,
        })
    }
}

impl CairoDeserialize for FixedArrayDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let type_def = TypeDef::c_deserialize(data)?;
        let size = pop_primitive::<u32>(data)?;
        Some(FixedArrayDef { type_def, size })
    }
}

impl CairoDeserialize for ResultDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let ok = TypeDef::c_deserialize(data)?;
        let err = TypeDef::c_deserialize(data)?;
        Some(ResultDef { ok, err })
    }
}

impl CairoDeserialize for RefDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(RefDef {
            id: pop_primitive::<Felt>(data)?,
        })
    }
}

impl CairoDeserialize for CustomDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        Some(CustomDef {
            id: pop_primitive::<Felt>(data)?,
        })
    }
}

impl CairoDeserialize for ColumnDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let id = pop_primitive(data)?;
        let name = deserialize_byte_array_string(data)?;
        let attributes: Vec<Attribute> = Vec::<Attribute>::c_deserialize(data)?;
        let type_def: TypeDef = TypeDef::c_deserialize(data)?;
        Some(ColumnDef {
            id,
            name,
            attributes,
            type_def,
        })
    }
}

impl CairoDeserialize for PrimaryDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let name = deserialize_byte_array_string(data)?;
        let attributes: Vec<Attribute> = Vec::<Attribute>::c_deserialize(data)?;
        let type_def: PrimaryTypeDef = PrimaryTypeDef::c_deserialize(data)?;
        Some(PrimaryDef {
            name,
            attributes,
            type_def,
        })
    }
}

impl CairoDeserialize for PrimaryTypeDef {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let selector = data.next()?.to_raw();
        match selector {
            selectors::Felt252 => Some(PrimaryTypeDef::Felt252),
            selectors::ShortUtf8 => Some(PrimaryTypeDef::ShortUtf8),
            selectors::Bytes31 => Some(PrimaryTypeDef::Bytes31),
            selectors::Bytes31E => Some(PrimaryTypeDef::Bytes31E(data.next()?)),
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
