use starknet_types_core::felt::Felt;

use crate::type_def::{ByteArrayDeserialization, selectors};
use crate::{
    Attribute, EnumDef, FeltIterator, FixedArrayDef, MemberDef, ResultDef, StructDef, TypeDef,
    VariantDef, deserialize_byte_array_string, pop_primitive, read_serialized_felt_array,
};
pub trait CairoDeserialize
where
    Self: Sized,
{
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self>;
    fn c_deserialize_boxed(data: &mut FeltIterator) -> Option<Box<Self>> {
        Self::c_deserialize(data).map(Box::new)
    }
}

impl CairoDeserialize for Attribute {
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let id = data.next()?;
        let data = read_serialized_felt_array(data)?;
        Some(Attribute { id, data })
    }
}

impl<T> CairoDeserialize for Vec<T>
where
    T: CairoDeserialize,
{
    fn c_deserialize(data: &mut FeltIterator) -> Option<Self> {
        let len = pop_primitive::<usize>(data)?;
        (0..len)
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
            selectors::Utf8Array => Some(TypeDef::Utf8Array(ByteArrayDeserialization::ISerde)),
            selectors::ByteArrayE => Some(TypeDef::ByteArrayE(data.next()?)),
            selectors::Tuple => Vec::<TypeDef>::c_deserialize(data).map(TypeDef::Tuple),
            selectors::Array => TypeDef::c_deserialize_boxed(data).map(TypeDef::Array),
            selectors::FixedArray => {
                FixedArrayDef::c_deserialize_boxed(data).map(TypeDef::FixedArray)
            }
            selectors::Felt252Dict => TypeDef::c_deserialize_boxed(data).map(TypeDef::Felt252Dict),
            selectors::Struct => StructDef::c_deserialize(data).map(TypeDef::Struct),
            selectors::Enum => EnumDef::c_deserialize(data).map(TypeDef::Enum),
            selectors::Option => TypeDef::c_deserialize_boxed(data).map(TypeDef::Option),
            selectors::Result => ResultDef::c_deserialize_boxed(data).map(TypeDef::Result),
            selectors::Nullable => TypeDef::c_deserialize_boxed(data).map(TypeDef::Nullable),
            selectors::Ref => Some(TypeDef::Ref(data.next()?)),
            selectors::Custom => Some(TypeDef::Custom(data.next()?)),
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
