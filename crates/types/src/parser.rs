use crate::deserialize::CairoDeserializer;
use crate::type_def::{ByteArrayEncodedDef, FixedArrayDef, MemberDef, StructDef, TypeDef};
use crate::utils::ResultInto;
use crate::value::{Enum, Nullable, Value};
use crate::{
    ArrayDef, Bytes31, Bytes31EncodedDef, CairoOption, CairoResult, ColumnDef, Custom, CustomDef,
    DecodeError, Encoded31Bytes, EncodedBytes, EnumDef, Field, Member, NullableDef, OptionDef,
    ResultDef, Struct, TupleDef,
};
use starknet_types_core::felt::Felt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeParserError {
    #[error(transparent)]
    Decode(#[from] DecodeError),

    #[error("type not supported: {0}")]
    UnsupportedType(&'static str),

    #[error("unimplemented: {0}")]
    Unimplemented(&'static str),

    #[error("invalid enum selector for {enum_name}: {selector:?}")]
    InvalidEnumSelector { enum_name: String, selector: Felt },

    #[error("Cannot parse an un expaneded Ref type. ")]
    RefNotSupported,

    #[error("unknown custom encoding: {0}")]
    UnknownEncoding(String),

    #[error("invalid array length for {what}: {len} (max {max:?})")]
    InvalidLength {
        what: &'static str,
        len: usize,
        max: Option<usize>,
    },

    #[error("invariant violation: {0}")]
    InvariantViolation(&'static str),
}

impl TypeParserError {
    #[inline]
    pub fn decode(e: DecodeError) -> Self {
        Self::Decode(e)
    }

    #[inline]
    pub fn unsupported_type(ty: &'static str) -> Self {
        Self::UnsupportedType(ty)
    }

    #[inline]
    pub fn unimplemented(what: &'static str) -> Self {
        Self::Unimplemented(what)
    }

    #[inline]
    pub fn invalid_enum_selector(enum_name: impl Into<String>, selector: Felt) -> Self {
        Self::InvalidEnumSelector {
            enum_name: enum_name.into(),
            selector,
        }
    }

    #[inline]
    pub fn unknown_encoding(enc: impl Into<String>) -> Self {
        Self::UnknownEncoding(enc.into())
    }

    #[inline]
    pub fn invalid_length(what: &'static str, len: usize, max: Option<usize>) -> Self {
        Self::InvalidLength { what, len, max }
    }

    #[inline]
    pub fn invariant(msg: &'static str) -> Self {
        Self::InvariantViolation(msg)
    }
}
pub type TypeParserResult<T> = Result<T, TypeParserError>;

pub trait ParseValues<D: CairoDeserializer> {
    type Value;
    fn parse_values(&self, deserializer: &mut D) -> TypeParserResult<Vec<Self::Value>>;
}

impl<T: ParseValue<D>, D: CairoDeserializer> ParseValues<D> for Vec<T> {
    type Value = <T as ParseValue<D>>::Value;
    fn parse_values(&self, deserializer: &mut D) -> TypeParserResult<Vec<Self::Value>> {
        self.iter()
            .map(|item| item.parse(deserializer))
            .collect::<TypeParserResult<Vec<Self::Value>>>()
    }
}

pub trait ParseValue<D: CairoDeserializer> {
    type Value;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Self::Value>;
    fn parse_value_boxed(&self, deserializer: &mut D) -> TypeParserResult<Box<Self::Value>> {
        self.parse(deserializer).map(Box::new)
    }
    fn parse_multiple_values(
        &self,
        deserializer: &mut D,
        count: usize,
    ) -> TypeParserResult<Vec<Self::Value>> {
        (0..count)
            .into_iter()
            .map(|_| self.parse(deserializer))
            .collect()
    }
}

impl<D: CairoDeserializer> ParseValue<D> for TypeDef {
    type Value = Value;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Value> {
        match self {
            TypeDef::None => Ok(Value::None),
            TypeDef::Felt252 => Ok(Value::Felt252(deserializer.next_felt()?)),
            TypeDef::ShortUtf8 => deserializer
                .next_bytes31()
                .map_into(|b| Value::ShortUtf8(b.to_string())),
            TypeDef::Bytes31 => deserializer.next_bytes::<31>().map_into(Value::Bytes31),
            TypeDef::Bytes31Encoded(b) => b.parse(deserializer).map(Value::Bytes31Encoded),
            TypeDef::Bool => deserializer.next_bool().map_into(Value::Bool),
            TypeDef::U8 => deserializer.next_u8().map_into(Value::U8),
            TypeDef::U16 => deserializer.next_u16().map_into(Value::U16),
            TypeDef::U32 => deserializer.next_u32().map_into(Value::U32),
            TypeDef::U64 => deserializer.next_u64().map_into(Value::U64),
            TypeDef::U128 => deserializer.next_u128().map_into(Value::U128),
            TypeDef::U256 => deserializer.next_u256().map_into(Value::U256),
            TypeDef::U512 => deserializer.next_u512().map_into(Value::U512),
            TypeDef::I8 => deserializer.next_i8().map_into(Value::I8),
            TypeDef::I16 => deserializer.next_i16().map_into(Value::I16),
            TypeDef::I32 => deserializer.next_i32().map_into(Value::I32),
            TypeDef::I64 => deserializer.next_i64().map_into(Value::I64),
            TypeDef::I128 => deserializer.next_i128().map_into(Value::I128),
            TypeDef::ClassHash => deserializer.next_felt().map_into(Value::ClassHash),
            TypeDef::ContractAddress => deserializer.next_felt().map_into(Value::ContractAddress),
            TypeDef::EthAddress => deserializer.next_felt().map_into(Value::EthAddress),
            TypeDef::StorageAddress => deserializer.next_felt().map_into(Value::StorageAddress),
            TypeDef::StorageBaseAddress => {
                deserializer.next_felt().map_into(Value::StorageBaseAddress)
            }
            TypeDef::ByteArray => deserializer
                .next_byte_array_bytes()
                .map_into(Value::ByteArray),
            TypeDef::Utf8String => deserializer.next_string().map_into(Value::Utf8String),
            TypeDef::ByteArrayEncoded(bae) => bae.parse(deserializer).map(Value::ByteArrayEncoded),
            TypeDef::Tuple(tuple) => tuple.parse(deserializer).map(Value::Tuple),
            TypeDef::Array(a) => a.parse(deserializer).map(Value::Array),
            TypeDef::FixedArray(fa) => fa.parse(deserializer).map(Value::FixedArray),
            TypeDef::Felt252Dict(_ty) => Err(TypeParserError::Unimplemented("Felt252Dict")), // TODO: implement Felt252Dict parsing
            TypeDef::Struct(s) => s.parse(deserializer).map(Value::Struct),
            TypeDef::Enum(e) => e.parse_value_boxed(deserializer).map(Value::Enum),
            TypeDef::Ref(_) => Err(TypeParserError::RefNotSupported), // TODO: implement Ref parsing
            TypeDef::Custom(custom) => custom.parse(deserializer).map(Value::Custom),
            TypeDef::Option(option) => option.parse_value_boxed(deserializer).map(Value::Option),
            TypeDef::Result(r) => r.parse_value_boxed(deserializer).map(Value::Result),
            TypeDef::Nullable(nullable) => nullable
                .parse_value_boxed(deserializer)
                .map(Value::Nullable),
        }
    }
}

impl<D: CairoDeserializer> ParseValue<D> for MemberDef {
    type Value = Member;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Member> {
        Ok(Member {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.parse(deserializer)?,
        })
    }
}

impl<D: CairoDeserializer> ParseValue<D> for StructDef {
    type Value = Struct;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Struct> {
        Ok(Struct {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            members: self.members.parse_values(deserializer)?,
        })
    }
}

impl<D: CairoDeserializer> ParseValue<D> for ArrayDef {
    type Value = Vec<Value>;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Vec<Value>> {
        let count = deserializer.next_u32()?;
        self.type_def
            .parse_multiple_values(deserializer, count as usize)
    }
}

impl<D: CairoDeserializer> ParseValue<D> for FixedArrayDef {
    type Value = Vec<Value>;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Vec<Value>> {
        self.type_def
            .parse_multiple_values(deserializer, self.size as usize)
    }
}

impl<D: CairoDeserializer> ParseValue<D> for TupleDef {
    type Value = Vec<Value>;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Vec<Value>> {
        self.elements.parse_values(deserializer)
    }
}

impl<D: CairoDeserializer> ParseValue<D> for EnumDef {
    type Value = Enum;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Enum> {
        let selector = deserializer.next_felt()?;
        let field = self
            .variants
            .get(&selector)
            .ok_or(TypeParserError::InvalidEnumSelector {
                enum_name: self.name.clone(),
                selector,
            })?;

        Ok(Enum {
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            variant: field.name.clone(),
            variant_attributes: field.attributes.clone(),
            value: field.type_def.parse(deserializer)?,
        })
    }
}

impl<D: CairoDeserializer> ParseValue<D> for OptionDef {
    type Value = CairoOption<Value>;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<CairoOption<Value>> {
        match deserializer.next_option_is_some()? {
            true => self.type_def.parse(deserializer).map(CairoOption::Some),
            false => Ok(CairoOption::None),
        }
    }
}

impl<D: CairoDeserializer> ParseValue<D> for ResultDef {
    type Value = CairoResult<Value, Value>;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<CairoResult<Value, Value>> {
        match deserializer.next_result_is_ok()? {
            true => self.ok.parse(deserializer).map(CairoResult::Ok),
            false => self.err.parse(deserializer).map(CairoResult::Err),
        }
    }
}

impl<D: CairoDeserializer> ParseValue<D> for NullableDef {
    type Value = Nullable;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Nullable> {
        match deserializer.next_nullable_is_null()? {
            false => self.type_def.parse(deserializer).map(Nullable::NotNull),
            true => Ok(Nullable::Null),
        }
    }
}

impl<D: CairoDeserializer> ParseValue<D> for CustomDef {
    type Value = Custom;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Custom> {
        Ok(Custom {
            encoding: self.encoding.clone(),
            values: deserializer.next_array()?,
        })
    }
}

impl<D: CairoDeserializer> ParseValue<D> for ByteArrayEncodedDef {
    type Value = EncodedBytes;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Self::Value> {
        let bytes = deserializer.next_byte_array_bytes()?;
        Ok(EncodedBytes {
            encoding: self.encoding.clone(),
            bytes,
        })
    }
}

impl<D: CairoDeserializer> ParseValue<D> for Bytes31EncodedDef {
    type Value = Encoded31Bytes;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Self::Value> {
        let bytes = deserializer.next_bytes31()?;
        Ok(Encoded31Bytes {
            encoding: self.encoding.clone(),
            bytes: bytes.into(),
        })
    }
}

impl Bytes31EncodedDef {
    pub fn to_encoded_bytes_31(&self, felt: Felt) -> TypeParserResult<Encoded31Bytes> {
        let bytes31: Bytes31 = felt.try_into()?;
        Ok(Encoded31Bytes {
            encoding: self.encoding.clone(),
            bytes: bytes31.into(),
        })
    }
}

impl<D: CairoDeserializer> ParseValue<D> for ColumnDef {
    type Value = Field;
    fn parse(&self, deserializer: &mut D) -> TypeParserResult<Field> {
        Ok(Field {
            id: self.id.clone(),
            name: self.name.clone(),
            attributes: self.attributes.clone(),
            value: self.type_def.parse(deserializer)?,
        })
    }
}

// pub trait ColumnDefs {
//     fn parse_id_values<D: CairoDeserializer>(
//         &self,
//         deserializer: &mut D,
//     ) -> TypeParserResult<Vec<IdValue>>;
// }
