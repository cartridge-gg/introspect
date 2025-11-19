use crate::{Attribute, ascii_str_to_limbs};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum ByteArrayDeserialization {
    Serde,
    ISerde,
}

pub trait ItemDefTrait {
    fn wrap_to_type_def(self) -> TypeDef;
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum TypeDef {
    #[default]
    None,
    Felt252,
    ShortUtf8,
    Bytes31,
    Bytes31E(Felt),
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    U512,
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
    ByteArray(ByteArrayDeserialization),
    Utf8String(ByteArrayDeserialization),
    ByteArrayE(ByteArrayEDef),
    Tuple(TupleDef),
    Array(Box<ArrayDef>),
    FixedArray(Box<FixedArrayDef>),
    Felt252Dict(Box<Felt252DictDef>),
    Struct(StructDef),
    Enum(EnumDef),
    Option(Box<OptionDef>),
    Result(Box<ResultDef>),
    Nullable(Box<NullableDef>),
    Ref(RefDef),
    Custom(CustomDef),
}

#[allow(non_upper_case_globals)]
pub mod selectors {
    use super::ascii_str_to_limbs;

    pub const None: [u64; 4] = [0; 4];
    pub const Felt252: [u64; 4] = ascii_str_to_limbs("felt252");
    pub const ShortUtf8: [u64; 4] = ascii_str_to_limbs("ShortUtf8");
    pub const Bytes31: [u64; 4] = ascii_str_to_limbs("bytes31");
    pub const Bytes31E: [u64; 4] = ascii_str_to_limbs("bytes31e");
    pub const Bool: [u64; 4] = ascii_str_to_limbs("bool");
    pub const U8: [u64; 4] = ascii_str_to_limbs("u8");
    pub const U16: [u64; 4] = ascii_str_to_limbs("u16");
    pub const U32: [u64; 4] = ascii_str_to_limbs("u32");
    pub const U64: [u64; 4] = ascii_str_to_limbs("u64");
    pub const U128: [u64; 4] = ascii_str_to_limbs("u128");
    pub const U256: [u64; 4] = ascii_str_to_limbs("u256");
    pub const U512: [u64; 4] = ascii_str_to_limbs("u512");
    pub const I8: [u64; 4] = ascii_str_to_limbs("i8");
    pub const I16: [u64; 4] = ascii_str_to_limbs("i16");
    pub const I32: [u64; 4] = ascii_str_to_limbs("i32");
    pub const I64: [u64; 4] = ascii_str_to_limbs("i64");
    pub const I128: [u64; 4] = ascii_str_to_limbs("i128");
    pub const ShortString: [u64; 4] = ascii_str_to_limbs("ShortString");
    pub const ClassHash: [u64; 4] = ascii_str_to_limbs("ClassHash");
    pub const ContractAddress: [u64; 4] = ascii_str_to_limbs("ContractAddress");
    pub const EthAddress: [u64; 4] = ascii_str_to_limbs("EthAddress");
    pub const StorageAddress: [u64; 4] = ascii_str_to_limbs("StorageAddress");
    pub const StorageBaseAddress: [u64; 4] = ascii_str_to_limbs("StorageBaseAddress");
    pub const ByteArray: [u64; 4] = ascii_str_to_limbs("ByteArray");
    pub const Utf8String: [u64; 4] = ascii_str_to_limbs("Utf8String");
    pub const ByteArrayE: [u64; 4] = ascii_str_to_limbs("ByteArrayE");
    pub const Tuple: [u64; 4] = ascii_str_to_limbs("Tuple");
    pub const Array: [u64; 4] = ascii_str_to_limbs("Array");
    pub const FixedArray: [u64; 4] = ascii_str_to_limbs("FixedArray");
    pub const Felt252Dict: [u64; 4] = ascii_str_to_limbs("Felt252Dict");
    pub const Struct: [u64; 4] = ascii_str_to_limbs("struct");
    pub const Enum: [u64; 4] = ascii_str_to_limbs("enum");
    pub const Ref: [u64; 4] = ascii_str_to_limbs("Ref");
    pub const Custom: [u64; 4] = ascii_str_to_limbs("Custom");
    pub const Option: [u64; 4] = ascii_str_to_limbs("Option");
    pub const Result: [u64; 4] = ascii_str_to_limbs("Result");
    pub const Nullable: [u64; 4] = ascii_str_to_limbs("Nullable");
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StructDef {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub members: Vec<MemberDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MemberDef {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumDef {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub variants: HashMap<Felt, VariantDef>,
    pub order: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VariantDef {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ArrayDef {
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FixedArrayDef {
    pub type_def: TypeDef,
    pub size: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TupleDef {
    pub elements: Vec<TypeDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ByteArrayEDef {
    pub mode: ByteArrayDeserialization,
    pub encoding: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Felt252DictDef {
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RefDef {
    pub id: Felt,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CustomDef {
    pub id: Felt,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OptionDef {
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResultDef {
    pub ok: TypeDef,
    pub err: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NullableDef {
    pub type_def: TypeDef,
}

impl ItemDefTrait for TupleDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::Tuple(self)
    }
}

impl ItemDefTrait for ArrayDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::Array(Box::new(self))
    }
}

impl ItemDefTrait for FixedArrayDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::FixedArray(Box::new(self))
    }
}

impl ItemDefTrait for StructDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::Struct(self)
    }
}

impl ItemDefTrait for EnumDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::Enum(self)
    }
}

impl ItemDefTrait for Felt252DictDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::Felt252Dict(Box::new(self))
    }
}

impl ItemDefTrait for OptionDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::Option(Box::new(self))
    }
}

impl ItemDefTrait for ResultDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::Result(Box::new(self))
    }
}

impl ItemDefTrait for NullableDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::Nullable(Box::new(self))
    }
}

impl ItemDefTrait for ByteArrayEDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::ByteArrayE(self)
    }
}

impl ItemDefTrait for RefDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::Ref(self)
    }
}

impl ItemDefTrait for CustomDef {
    fn wrap_to_type_def(self) -> TypeDef {
        TypeDef::Custom(self)
    }
}

pub trait TypeName {
    fn type_name(&self) -> String;
}

impl TypeName for TypeDef {
    fn type_name(&self) -> String {
        match self {
            TypeDef::None => "None".to_string(),
            TypeDef::ShortUtf8 => "ShortUtf8".to_string(),
            TypeDef::Felt252 => "Felt252".to_string(),
            TypeDef::Bytes31E(encoding) => format!("ByteArrayE: {}", encoding),
            TypeDef::Bytes31 => "bytes31".to_string(),
            TypeDef::Bool => "bool".to_string(),
            TypeDef::U8 => "u8".to_string(),
            TypeDef::U16 => "u16".to_string(),
            TypeDef::U32 => "u32".to_string(),
            TypeDef::U64 => "u64".to_string(),
            TypeDef::U128 => "u128".to_string(),
            TypeDef::U256 => "u256".to_string(),
            TypeDef::U512 => "u512".to_string(),
            TypeDef::I8 => "i8".to_string(),
            TypeDef::I16 => "i16".to_string(),
            TypeDef::I32 => "i32".to_string(),
            TypeDef::I64 => "i64".to_string(),
            TypeDef::I128 => "i128".to_string(),
            TypeDef::ClassHash => "ClassHash".to_string(),
            TypeDef::ContractAddress => "ContractAddress".to_string(),
            TypeDef::EthAddress => "EthAddress".to_string(),
            TypeDef::StorageAddress => "StorageAddress".to_string(),
            TypeDef::StorageBaseAddress => "StorageBaseAddress".to_string(),
            TypeDef::ByteArray(_) => "ByteArray".to_string(),
            TypeDef::Utf8String(_) => "Utf8String".to_string(),
            TypeDef::ByteArrayE(inner) => format!("ByteArrayE: {}", inner.encoding),
            TypeDef::Tuple(inner) => format!(
                "({})",
                inner
                    .elements
                    .iter()
                    .map(|e| e.type_name())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            TypeDef::Array(inner) => format!("Vec<{}>", inner.type_def.type_name()),
            TypeDef::FixedArray(inner) => {
                format!("[{}; {}]", inner.type_def.type_name(), inner.size)
            }
            TypeDef::Felt252Dict(inner) => format!("Felt252Dict<{}>", inner.type_def.type_name()),
            TypeDef::Struct(s) => s.name.clone(),
            TypeDef::Enum(e) => e.name.clone(),

            TypeDef::Option(inner) => format!("Option<{}>", inner.type_def.type_name()),
            TypeDef::Result(inner) => format!(
                "Result<{}, {}>",
                inner.ok.type_name(),
                inner.err.type_name()
            ),
            TypeDef::Nullable(inner) => format!("Nullable<{}>", inner.type_def.type_name()),
            TypeDef::Ref(inner) => inner.id.to_hex_string(),
            TypeDef::Custom(inner) => inner.id.to_hex_string(),
        }
    }
}

impl PartialEq for EnumDef {
    fn eq(&self, other: &Self) -> bool {
        let is_eq = self.name == other.name && self.attributes == other.attributes;
        if !is_eq {
            return false;
        }

        self.variants
            .iter()
            .all(|(k, v)| other.variants.get(k).map(|ov| v == ov).unwrap_or(false))
    }
}

pub trait SingletonTypeDefTrait
where
    Self: Sized,
{
    fn inner_type_def(self) -> TypeDef;
    fn from_inner_type_def(type_def: TypeDef) -> Self;
    fn to_wrapped_type_def(self) -> TypeDef;
    fn wrap_inner_type_def(type_def: TypeDef) -> TypeDef {
        Self::to_wrapped_type_def(Self::from_inner_type_def(type_def))
    }
}

impl SingletonTypeDefTrait for ArrayDef {
    fn inner_type_def(self) -> TypeDef {
        self.type_def
    }
    fn from_inner_type_def(type_def: TypeDef) -> Self {
        ArrayDef { type_def }
    }
    fn to_wrapped_type_def(self) -> TypeDef {
        TypeDef::Array(Box::new(self))
    }
}

impl SingletonTypeDefTrait for OptionDef {
    fn inner_type_def(self) -> TypeDef {
        self.type_def
    }
    fn from_inner_type_def(type_def: TypeDef) -> Self {
        OptionDef { type_def }
    }
    fn to_wrapped_type_def(self) -> TypeDef {
        TypeDef::Option(Box::new(self))
    }
}

impl SingletonTypeDefTrait for NullableDef {
    fn inner_type_def(self) -> TypeDef {
        self.type_def
    }
    fn from_inner_type_def(type_def: TypeDef) -> Self {
        NullableDef { type_def }
    }
    fn to_wrapped_type_def(self) -> TypeDef {
        TypeDef::Nullable(Box::new(self))
    }
}

impl SingletonTypeDefTrait for Felt252DictDef {
    fn inner_type_def(self) -> TypeDef {
        self.type_def
    }
    fn from_inner_type_def(type_def: TypeDef) -> Self {
        Felt252DictDef { type_def }
    }
    fn to_wrapped_type_def(self) -> TypeDef {
        TypeDef::Felt252Dict(Box::new(self))
    }
}

impl StructDef {
    pub fn new(name: String, attributes: Vec<Attribute>, members: Vec<MemberDef>) -> Self {
        StructDef {
            name,
            attributes,
            members,
        }
    }

    pub fn new_type_def(
        name: String,
        attributes: Vec<Attribute>,
        members: Vec<MemberDef>,
    ) -> TypeDef {
        TypeDef::Struct(StructDef::new(name, attributes, members))
    }
}

impl MemberDef {
    pub fn new(name: String, attributes: Vec<Attribute>, type_def: TypeDef) -> Self {
        MemberDef {
            name,
            attributes,
            type_def,
        }
    }
}

impl EnumDef {
    pub fn new(
        name: String,
        attributes: Vec<Attribute>,
        variants: Vec<(Felt, VariantDef)>,
    ) -> Self {
        EnumDef {
            name,
            attributes,
            order: variants.iter().map(|(k, _)| k.clone()).collect(),
            variants: variants.into_iter().collect(),
        }
    }

    pub fn new_type_def(
        name: String,
        attributes: Vec<Attribute>,
        variants: Vec<(Felt, VariantDef)>,
    ) -> TypeDef {
        TypeDef::Enum(EnumDef::new(name, attributes, variants))
    }
}

impl VariantDef {
    pub fn new(name: String, attributes: Vec<Attribute>, type_def: TypeDef) -> Self {
        VariantDef {
            name,
            attributes,
            type_def,
        }
    }
}

impl Deref for ArrayDef {
    type Target = TypeDef;
    fn deref(&self) -> &Self::Target {
        &self.type_def
    }
}

impl FixedArrayDef {
    pub fn new(type_def: TypeDef, size: u32) -> Self {
        FixedArrayDef { type_def, size }
    }

    pub fn new_type_def(type_def: TypeDef, size: u32) -> TypeDef {
        TypeDef::FixedArray(Box::new(FixedArrayDef::new(type_def, size)))
    }
}

impl TupleDef {
    pub fn new(elements: Vec<TypeDef>) -> Self {
        TupleDef { elements }
    }

    pub fn new_type_def(elements: Vec<TypeDef>) -> TypeDef {
        TypeDef::Tuple(TupleDef::new(elements))
    }

    pub fn to_type_def(self) -> TypeDef {
        if self.elements.is_empty() {
            TypeDef::None
        } else if self.elements.len() == 1 {
            let mut element = self.elements;
            element.pop().unwrap()
        } else {
            TypeDef::Tuple(self)
        }
    }
}

impl Deref for TupleDef {
    type Target = Vec<TypeDef>;
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl Felt252DictDef {
    pub fn new(type_def: TypeDef) -> Self {
        Felt252DictDef { type_def }
    }

    pub fn new_type_def(type_def: TypeDef) -> TypeDef {
        TypeDef::Felt252Dict(Box::new(Felt252DictDef::new(type_def)))
    }
}

impl Deref for OptionDef {
    type Target = TypeDef;
    fn deref(&self) -> &Self::Target {
        &self.type_def
    }
}

impl ResultDef {
    pub fn new(ok: TypeDef, err: TypeDef) -> Self {
        ResultDef { ok, err }
    }

    pub fn new_type_def(ok: TypeDef, err: TypeDef) -> TypeDef {
        TypeDef::Result(Box::new(ResultDef::new(ok, err)))
    }
}

impl Deref for NullableDef {
    type Target = TypeDef;
    fn deref(&self) -> &Self::Target {
        &self.type_def
    }
}

impl CustomDef {
    pub fn new(id: Felt) -> Self {
        CustomDef { id }
    }

    pub fn new_type_def(id: Felt) -> TypeDef {
        TypeDef::Custom(CustomDef::new(id))
    }
}

impl RefDef {
    pub fn new(id: Felt) -> Self {
        RefDef { id }
    }

    pub fn new_type_def(id: Felt) -> TypeDef {
        TypeDef::Ref(RefDef::new(id))
    }
}
