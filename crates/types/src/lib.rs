use cainome_cairo_serde::{ByteArray, Bytes31};
use introspect_value::{Custom, ToValue, Value};
use num_traits::Zero;
use starknet_types_core::{felt::Felt, short_string::ShortString};
use std::collections::VecDeque;

pub enum TypeDef {
    None,
    Felt252,
    Bool,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Uint256,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    USize,
    ShortString,
    ClassHash,
    ContractAddress,
    EthAddress,
    ByteArray,
    Tuple(Vec<TypeDef>),
    Array(Box<TypeDef>),
    FixedArray(FixedArrayDef),
    Felt252Dict(Box<TypeDef>),
    Struct(StructDef),
    Enum(EnumDef),
    Ref(String),
    Schema(Vec<FieldDef>),
    Custom(String),
    Option(Box<TypeDef>),
    Result(CairoResult),
    Nullable(Box<TypeDef>),
    Encoding(String),
    DynamicEncoding,
}
pub struct FieldDef {
    pub selector: String,
    pub name: String,
    pub attrs: Vec<String>,
    pub ty: TypeDef,
}

pub struct StructDef {
    pub name: String,
    pub attrs: Vec<String>,
    pub children: Vec<MemberDef>,
}

pub struct EnumDef {
    pub name: String,
    pub attrs: Vec<String>,
    pub children: Vec<FieldDef>,
}

pub struct FixedArrayDef {
    pub ty: Box<TypeDef>,
    pub size: u32,
}

pub struct MemberDef {
    pub name: String,
    pub attrs: Vec<String>,
    pub ty: TypeDef,
}

pub struct CairoResult {
    pub ok: Box<TypeDef>,
    pub err: Box<TypeDef>,
}

pub fn pop_primitive<T: TryFrom<Felt>>(data: &mut VecDeque<Felt>) -> Option<T> {
    data.pop_front()?.try_into().ok()
}

fn read_serialized_array(data: &mut VecDeque<Felt>) -> Option<Vec<Felt>> {
    let len = pop_primitive(data)?;
    (0..len)
        .into_iter()
        .map(|_| data.pop_front())
        .collect::<Option<Vec<Felt>>>()
}

pub fn felt_to_utf8_string(felt: Felt) -> Option<ShortString> {
    let bytes = felt.to_bytes_be();
    let first = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    String::from_utf8(bytes[first..32].to_vec())
        .ok()
        .and_then(|v| ShortString::try_from(v).ok())
}

pub fn byte_array_felts_to_string(data: &mut VecDeque<Felt>) -> Option<String> {
    let len = data.pop_front()?.try_into().ok()?;

    let mut bytes: Vec<Bytes31> = Vec::with_capacity(len);
    for _ in 0..len {
        bytes.push(Bytes31::new(data.pop_front()?).ok()?);
    }
    let pending_word = data.pop_front()?;
    let pending_word_len = data.pop_front()?.try_into().ok()?;

    Some(
        ByteArray {
            data: bytes,
            pending_word,
            pending_word_len,
        }
        .to_string()
        .ok()?,
    )
}

fn parse_tuple_to_value(tys: &Vec<TypeDef>, data: &mut VecDeque<Felt>) -> Option<Vec<Value>> {
    tys.iter()
        .map(|ty| ty.to_value(data))
        .collect::<Option<Vec<Value>>>()
}

fn to_custom_value(name: &str, data: &mut VecDeque<Felt>) -> Option<Custom> {
    Some(Custom {
        name: name.to_string(),
        values: read_serialized_array(data)?,
    })
}

fn to_option_value(ty: &TypeDef, data: &mut VecDeque<Felt>) -> Option<Option<Value>> {
    let is_some = data.pop_front()?.is_zero();
    match is_some {
        true => ty.to_value(data).map(Some),
        false => Some(None),
    }
}

impl ToValue for TypeDef {
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Value> {
        match self {
            TypeDef::None => Some(Value::None),
            TypeDef::Felt252 => pop_primitive(data).map(Value::Felt252),
            TypeDef::Bool => data.pop_front().map(|v| Value::Bool(!v.is_zero())),
            TypeDef::Uint8 => pop_primitive(data).map(Value::Uint8),
            TypeDef::Uint16 => pop_primitive(data).map(Value::Uint16),
            TypeDef::Uint32 => pop_primitive(data).map(Value::Uint32),
            TypeDef::Uint64 => pop_primitive(data).map(Value::Uint64),
            TypeDef::Uint128 => pop_primitive(data).map(Value::Uint128),
            TypeDef::Uint256 => {
                let low = pop_primitive(data)?;
                let high = pop_primitive(data)?;
                Some(Value::Uint256(introspect_value::U256 { low, high }))
            }
            TypeDef::Int8 => pop_primitive(data).map(Value::Int8),
            TypeDef::Int16 => pop_primitive(data).map(Value::Int16),
            TypeDef::Int32 => pop_primitive(data).map(Value::Int32),
            TypeDef::Int64 => pop_primitive(data).map(Value::Int64),
            TypeDef::Int128 => pop_primitive(data).map(Value::Int128),
            TypeDef::USize => pop_primitive(data).map(Value::USize),
            TypeDef::ShortString => felt_to_utf8_string(data.pop_front()?).map(Value::ShortString),
            TypeDef::ClassHash => pop_primitive(data).map(Value::ClassHash),
            TypeDef::ContractAddress => data.pop_front().map(Value::ContractAddress),
            TypeDef::EthAddress => data.pop_front().map(Value::EthAddress),
            TypeDef::ByteArray => byte_array_felts_to_string(data).map(Value::ByteArray),
            TypeDef::Tuple(tys) => parse_tuple_to_value(tys, data).map(Value::Tuple),
            TypeDef::Array(ty) => {
                let size = pop_primitive(data)?;
                ty.to_value_multiple(data, size).map(Value::Array)
            }
            TypeDef::FixedArray(fa) => fa
                .ty
                .to_value_multiple(data, fa.size as usize)
                .map(Value::FixedArray),
            TypeDef::Felt252Dict(ty) => None,
            TypeDef::Struct(s) => None,
            TypeDef::Enum(e) => None,
            TypeDef::Ref(_) => None,
            TypeDef::Schema(fields) => None,
            TypeDef::Custom(name) => to_custom_value(name, data).map(Value::Custom),
            TypeDef::Option(ty) => to_option_value(ty, data).map(Box::new).map(Value::Option),
            TypeDef::Result(r) => None,
            TypeDef::Nullable(ty) => None,
            TypeDef::Encoding(_) => None,
            TypeDef::DynamicEncoding => None,
        }
    }
}
