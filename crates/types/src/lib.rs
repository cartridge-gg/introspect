use cainome_cairo_serde::{ByteArray, Bytes31};
use introspect_value::{Custom, Enum, Field, Member, Struct, ToValue, Value};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;
use std::collections::{HashMap, VecDeque};
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum TypeDef {
    #[default]
    None,
    Felt252,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    I8,
    I16,
    I32,
    I64,
    I128,
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
    Result(ResultDef),
    Nullable(Box<TypeDef>),
    Encoding(String),
    DynamicEncoding,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldDef {
    pub selector: Felt,
    pub name: String,
    pub attrs: Vec<String>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldInfo {
    pub selector: Felt,
    pub name: String,
    pub attrs: Vec<String>,
}

impl From<&FieldDef> for FieldInfo {
    fn from(field_def: &FieldDef) -> Self {
        FieldInfo {
            selector: field_def.selector.clone(),
            name: field_def.name.clone(),
            attrs: field_def.attrs.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VariantDef {
    pub name: String,
    pub attrs: Vec<String>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructDef {
    pub name: String,
    pub attrs: Vec<String>,
    pub members: Vec<MemberDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumDef {
    pub name: String,
    pub attrs: Vec<String>,
    pub variants: HashMap<Felt, VariantDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FixedArrayDef {
    pub type_def: Box<TypeDef>,
    pub size: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemberDef {
    pub name: String,
    pub attrs: Vec<String>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResultDef {
    pub ok: Box<TypeDef>,
    pub err: Box<TypeDef>,
}

pub fn pop_primitive<T: TryFrom<Felt>>(data: &mut VecDeque<Felt>) -> Option<T> {
    data.pop_front()?.try_into().ok()
}

pub fn pop_short_string(data: &mut VecDeque<Felt>) -> Option<String> {
    felt_to_utf8_string(data.pop_front()?)
}

pub fn read_serialized_felt_array(data: &mut VecDeque<Felt>) -> Option<Vec<Felt>> {
    let len = pop_primitive(data)?;
    (0..len)
        .into_iter()
        .map(|_| data.pop_front())
        .collect::<Option<Vec<Felt>>>()
}

pub fn felt_to_utf8_string(felt: Felt) -> Option<String> {
    let bytes = felt.to_bytes_be();
    let first = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    String::from_utf8(bytes[first..32].to_vec()).ok()
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

fn parse_tuple_to_value(type_defs: &Vec<TypeDef>, data: &mut VecDeque<Felt>) -> Option<Vec<Value>> {
    type_defs
        .iter()
        .map(|type_def| type_def.to_value(data))
        .collect::<Option<Vec<Value>>>()
}

fn to_custom_value(name: &str, data: &mut VecDeque<Felt>) -> Option<Custom> {
    Some(Custom {
        name: name.to_string(),
        values: read_serialized_felt_array(data)?,
    })
}

fn to_option_value(type_def: &TypeDef, data: &mut VecDeque<Felt>) -> Option<Option<Value>> {
    let is_some = data.pop_front()?.is_zero();
    match is_some {
        true => type_def.to_value(data).map(Some),
        false => Some(None),
    }
}

impl ToValue for MemberDef {
    type Value = Member;
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Member> {
        Some(Member {
            name: self.name.clone(),
            attrs: self.attrs.clone(),
            value: self.type_def.to_value(data)?,
        })
    }
}

impl ToValue for StructDef {
    type Value = Struct;
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Struct> {
        Some(Struct {
            name: self.name.clone(),
            attrs: self.attrs.clone(),
            members: self
                .members
                .iter()
                .map(|child| child.to_value(data))
                .collect::<Option<Vec<Member>>>()?,
        })
    }
}

impl ToValue for FixedArrayDef {
    type Value = Vec<Value>;
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Vec<Value>> {
        self.type_def.to_value_multiple(data, self.size as usize)
    }
}

impl ToValue for EnumDef {
    type Value = Enum;
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Enum> {
        let selector = pop_primitive::<Felt>(data)?;
        let field = self.variants.get(&selector)?;

        Some(Enum {
            name: self.name.clone(),
            attrs: self.attrs.clone(),
            variant: field.name.clone(),
            variant_attrs: field.attrs.clone(),
            value: field.type_def.to_value(data)?,
        })
    }

    fn to_value_multiple(
        &self,
        data: &mut VecDeque<Felt>,
        count: usize,
    ) -> Option<Vec<Self::Value>> {
        (0..count)
            .into_iter()
            .map(|_| self.to_value(data))
            .collect()
    }
}

impl ToValue for TypeDef {
    type Value = Value;
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Value> {
        match self {
            TypeDef::None => Some(Value::None),
            TypeDef::Felt252 => pop_primitive(data).map(Value::Felt252),
            TypeDef::Bool => data.pop_front().map(|v| Value::Bool(!v.is_zero())),
            TypeDef::U8 => pop_primitive(data).map(Value::U8),
            TypeDef::U16 => pop_primitive(data).map(Value::U16),
            TypeDef::U32 => pop_primitive(data).map(Value::U32),
            TypeDef::U64 => pop_primitive(data).map(Value::U64),
            TypeDef::U128 => pop_primitive(data).map(Value::U128),
            TypeDef::U256 => {
                let low = pop_primitive(data)?;
                let high = pop_primitive(data)?;
                Some(Value::U256(introspect_value::U256 { low, high }))
            }
            TypeDef::I8 => pop_primitive(data).map(Value::I8),
            TypeDef::I16 => pop_primitive(data).map(Value::I16),
            TypeDef::I32 => pop_primitive(data).map(Value::I32),
            TypeDef::I64 => pop_primitive(data).map(Value::I64),
            TypeDef::I128 => pop_primitive(data).map(Value::I128),
            TypeDef::USize => pop_primitive(data).map(Value::USize),
            TypeDef::ShortString => felt_to_utf8_string(data.pop_front()?).map(Value::ShortString),
            TypeDef::ClassHash => pop_primitive(data).map(Value::ClassHash),
            TypeDef::ContractAddress => data.pop_front().map(Value::ContractAddress),
            TypeDef::EthAddress => data.pop_front().map(Value::EthAddress),
            TypeDef::ByteArray => byte_array_felts_to_string(data).map(Value::ByteArray),
            TypeDef::Tuple(type_defs) => parse_tuple_to_value(type_defs, data).map(Value::Tuple),
            TypeDef::Array(type_def) => {
                let size = pop_primitive(data)?;
                type_def.to_value_multiple(data, size).map(Value::Array)
            }
            TypeDef::FixedArray(fa) => fa.to_value(data).map(Value::FixedArray),
            TypeDef::Felt252Dict(_ty) => None,
            TypeDef::Struct(s) => s.to_value(data).map(Value::Struct),
            TypeDef::Enum(e) => e.to_value(data).map(Box::new).map(Value::Enum),
            TypeDef::Ref(_) => None,
            TypeDef::Schema(_fields) => None,
            TypeDef::Custom(name) => to_custom_value(name, data).map(Value::Custom),
            TypeDef::Option(type_def) => to_option_value(type_def, data)
                .map(Box::new)
                .map(Value::Option),
            TypeDef::Result(_r) => None,
            TypeDef::Nullable(_ty) => None,
            TypeDef::Encoding(_) => None,
            TypeDef::DynamicEncoding => None,
        }
    }
}

impl ToValue for FieldDef {
    type Value = Field;
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Field> {
        Some(Field {
            selector: self.selector.clone(),
            name: self.name.clone(),
            attrs: self.attrs.clone(),
            value: self.type_def.to_value(data)?,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeDefVec(pub Vec<TypeDef>);

impl std::ops::Deref for TypeDefVec {
    type Target = Vec<TypeDef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TypeDefVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldDefVec(pub Vec<FieldDef>);

impl std::ops::Deref for FieldDefVec {
    type Target = Vec<FieldDef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<FieldDef> for FieldDefVec {
    fn from_iter<T: IntoIterator<Item = FieldDef>>(iter: T) -> Self {
        FieldDefVec(iter.into_iter().collect())
    }
}

impl std::ops::DerefMut for FieldDefVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToValue for FieldDefVec {
    type Value = Vec<Field>;
    fn to_value(&self, data: &mut VecDeque<Felt>) -> Option<Vec<Field>> {
        self.0
            .iter()
            .map(|field_def| field_def.to_value(data))
            .collect::<Option<Vec<Field>>>()
    }
}

impl From<Vec<FieldDef>> for FieldDefVec {
    fn from(fields: Vec<FieldDef>) -> Self {
        FieldDefVec(fields)
    }
}
