use crate::{Attribute, Attributes, DecodeError, DecodeResult, ascii_str_to_limbs};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum ByteArrayDeserialization {
    Serde,
    ISerde,
}
pub struct Utf8String(pub String);
pub struct ByteArray(pub Vec<u8>);

impl Deref for Utf8String {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ByteArray {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ElementDef {}

macro_rules! define_type_variants {

    (
        $($variant:ident $( ( $inner:ty ) )? => $selector:expr),* $(,)?
    ) => {
        #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
        pub enum TypeDef{
            #[default]
            None,
            $($variant $( ( $inner ) )?,)*
        }

        #[allow(non_upper_case_globals)]
        pub mod selectors {
            use super::ascii_str_to_limbs;
            pub const None: [u64; 4] = [0; 4];
            $(pub const $variant: [u64; 4] = ascii_str_to_limbs($selector);)*
        }


        impl TypeDef {
            pub fn item_name(&self) -> &'static str {
                match self {
                    TypeDef::None => "None",
                    $(
                        define_type_variants!(@item_name_arm $variant $( ( $inner ) )?) => stringify!($variant),
                    )*
                }
            }
        }

        impl ElementDef for TypeDef {}
    };

    // unit variant: TypeDef::Foo => "Foo"
    (@item_name_arm $variant:ident) => {
        TypeDef::$variant
    };

    // single-field tuple variant: TypeDef::Bar(_) => "Bar"
    (@item_name_arm $variant:ident ( $inner:ty )) => {
        TypeDef::$variant (_)
    };
}

define_type_variants! (
        Felt252 => "felt252",
        ShortUtf8 => "short_utf8",
        Bytes31 => "bytes31",
        Bytes31Encoded(Bytes31EncodedDef) => "bytes31_encoded",
        Bool => "bool",
        U8 => "u8",
        U16 => "u16",
        U32 => "u32",
        U64 => "u64",
        U128 => "u128",
        U256 => "u256",
        U512 => "u512",
        I8 => "i8",
        I16 => "i16",
        I32 => "i32",
        I64 => "i64",
        I128 => "i128",
        ClassHash => "class_hash",
        ContractAddress => "contract_address",
        EthAddress => "eth_address",
        StorageAddress => "storage_address",
        StorageBaseAddress => "storage_base_address",
        ByteArray => "byte_array",
        Utf8String => "utf8_string",
        ByteArrayEncoded(ByteArrayEncodedDef) => "byte_array_encoded",
        Tuple(TupleDef) => "tuple",
        Array(Box<ArrayDef>) => "array",
        FixedArray(Box<FixedArrayDef>) => "fixed_array",
        Felt252Dict(Box<Felt252DictDef>) => "felt252_dict",
        Struct(StructDef) => "struct",
        Enum(EnumDef) => "enum",
        Option(Box<OptionDef>) => "option",
        Result(Box<ResultDef>) => "result",
        Nullable(Box<NullableDef>) => "nullable",
        Ref(RefDef) => "ref",
        Custom(CustomDef) => "custom",
);

pub trait TypeName {
    fn type_name(&self) -> String;
}

impl TypeName for TypeDef {
    fn type_name(&self) -> String {
        match self {
            TypeDef::ByteArrayEncoded(inner) => format!("ByteArrayEncoded: {}", inner.encoding),
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
            TypeDef::Custom(inner) => inner.encoding.clone(),
            _ => self.item_name().to_string(),
        }
    }
}

pub trait ItemDefTrait {
    fn wrap_to_type_def(self) -> TypeDef;
}

pub trait SimpleDefTrait
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

macro_rules! item_def_trait {
    ($type:ty, $variant:ident) => {
        item_def_trait!(@impl $type, $variant, false);
    };
    ($type:ty, $variant:ident, Box) => {
        item_def_trait!(@impl $type, $variant, true);
    };
    (@impl $type:ty, $variant:ident, $boxed:ident) => {
        impl ItemDefTrait for $type {
            fn wrap_to_type_def(self) -> TypeDef {
                TypeDef::$variant(maybe_boxed!(self, $boxed))
            }
        }

        impl ElementDef for $type {}
    };
}

macro_rules! maybe_boxed {
    ($to_box:expr, true) => {
        Box::new($to_box)
    };
    ($to_box:expr, false) => {
        $to_box
    };
}

macro_rules! item_def_constructors {
    ($type:ident, $variant:ident) => {
        impl SimpleDefTrait for $type {
            fn inner_type_def(self) -> TypeDef {
                self.type_def
            }
            fn from_inner_type_def(type_def: TypeDef) -> Self {
                Self { type_def: type_def }
            }
            fn to_wrapped_type_def(self) -> TypeDef {
                TypeDef::$variant(Box::new(self))
            }
        }

        impl Deref for $type {
            type Target = TypeDef;
            fn deref(&self) -> &Self::Target {
                &self.type_def
            }
        }

        item_def_constructors!(@impl $type, $variant, [type_def: TypeDef], true);
    };
    // For types with simple fields
    ($type:ident, $variant:ident, [$($field:ident: $field_ty:ty),*]) => {
        item_def_constructors!(@impl $type, $variant, [$($field: $field_ty),*], false);
    };
    // For types that need Box wrapping
    ($type:ident, $variant:ident, [$($field:ident: $field_ty:ty),*], Box) => {
        item_def_constructors!(@impl $type, $variant, [$($field: $field_ty),*], true);
    };
    (@impl $type:ident, $variant:ident, [$($field:ident: $field_ty:ty),*], $boxed:ident) => {

        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
        pub struct $type {
            $(pub $field: $field_ty),*
        }

        impl $type {
            pub fn new($($field: $field_ty),*) -> Self {
                Self { $($field),* }
            }

            pub fn new_type_def($($field: $field_ty),*) -> TypeDef {
                {
                    let value = Self::new($($field),*);
                    TypeDef::$variant(maybe_boxed!(value, $boxed))
                }
            }
        }
        item_def_trait!(@impl  $type, $variant, $boxed);
    };

}

item_def_constructors!(ArrayDef, Array);
item_def_constructors!(OptionDef, Option);
item_def_constructors!(NullableDef, Nullable);
item_def_constructors!(Felt252DictDef, Felt252Dict);
item_def_constructors!(StructDef, Struct, [name: String, attributes: Vec<Attribute>, members: Vec<MemberDef>]);
item_def_constructors!(RefDef, Ref, [id: Felt]);
item_def_constructors!(CustomDef, Custom, [encoding: String]);
item_def_constructors!(ByteArrayEncodedDef, ByteArrayEncoded, [encoding: String]);
item_def_constructors!(Bytes31EncodedDef, Bytes31Encoded, [encoding: String]);
item_def_constructors!(FixedArrayDef, FixedArray,  [type_def: TypeDef, size: u32], Box);
item_def_constructors!(ResultDef, Result,  [ok: TypeDef, err: TypeDef], Box);

item_def_trait!(EnumDef, Enum);
item_def_trait!(TupleDef, Tuple);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumDef {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub variants: HashMap<Felt, VariantDef>,
    pub order: Vec<Felt>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MemberDef {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VariantDef {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TupleDef {
    pub elements: Vec<TypeDef>,
}

impl Attributes for EnumDef {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for StructDef {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for VariantDef {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Attributes for MemberDef {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Deref for TupleDef {
    type Target = Vec<TypeDef>;
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl Deref for RefDef {
    type Target = Felt;
    fn deref(&self) -> &Self::Target {
        &self.id
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

impl ElementDef for MemberDef {}

impl VariantDef {
    pub fn new(name: String, attributes: Vec<Attribute>, type_def: TypeDef) -> Self {
        VariantDef {
            name,
            attributes,
            type_def,
        }
    }
}

impl ElementDef for VariantDef {}

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

    pub fn get_variant(&self, selector: &Felt) -> DecodeResult<&VariantDef> {
        self.variants
            .get(&selector)
            .ok_or(DecodeError::invalid_enum_selector(
                self.name.clone(),
                *selector,
            ))
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
