use crate::{AsCairo, AstToString, AstTryInto, IntrospectError, IntrospectResult, TryFromAst};
use cairo_lang_syntax::node::ast::{OptionTypeClause, TypeClause};
use itertools::Itertools;
use salsa::Database;

const CORE_TYPES: &[&str] = &[
    "felt252",
    "bool",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "u256",
    "u512",
    "core::integer::u512",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "bytes31",
    "ClassHash",
    "starknet::ClassHash",
    "ContractAddress",
    "starknet::ContractAddress",
    "EthAddress",
    "starknet::EthAddress",
    "StorageAddress",
    "starknet::StorageAddress",
    "StorageBaseAddress",
    "starknet::storage_access::StorageBaseAddress",
    "ByteArray",
];

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct FixedArray {
    pub ty: Ty,
    pub size: String,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct TyItem {
    pub name: String,
    pub params: Option<Vec<Ty>>,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Ty {
    Item(TyItem),
    Tuple(Vec<Ty>),
    FixedArray(Box<FixedArray>),
}

pub enum CairoCoreType {
    Felt252,
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
    Bytes31,
    ClassHash,
    ContractAddress,
    EthAddress,
    StorageAddress,
    StorageBaseAddress,
    ByteArray,
}

impl CairoCoreType {
    pub fn type_str(&self) -> &str {
        match self {
            CairoCoreType::Felt252 => "felt252",
            CairoCoreType::Bool => "bool",
            CairoCoreType::U8 => "u8",
            CairoCoreType::U16 => "u16",
            CairoCoreType::U32 => "u32",
            CairoCoreType::U64 => "u64",
            CairoCoreType::U128 => "u128",
            CairoCoreType::U256 => "u256",
            CairoCoreType::U512 => "u512",
            CairoCoreType::I8 => "i8",
            CairoCoreType::I16 => "i16",
            CairoCoreType::I32 => "i32",
            CairoCoreType::I64 => "i64",
            CairoCoreType::I128 => "i128",
            CairoCoreType::Bytes31 => "bytes31",
            CairoCoreType::ClassHash => "ClassHash",
            CairoCoreType::ContractAddress => "ContractAddress",
            CairoCoreType::EthAddress => "EthAddress",
            CairoCoreType::StorageAddress => "StorageAddress",
            CairoCoreType::StorageBaseAddress => "StorageBaseAddress",
            CairoCoreType::ByteArray => "ByteArray",
        }
    }
}

impl TyItem {
    pub fn parse(type_str: &str) -> IntrospectResult<Self> {
        if type_str.ends_with('>') {
            let (name, types) =
                parse_wrapped_types(type_str).ok_or(IntrospectError::FailedToParseType)?;
            let params = types
                .into_iter()
                .map(Ty::parse)
                .collect::<IntrospectResult<_>>()?;
            Ok(Self {
                name: name.to_string(),
                params: Some(params),
            })
        } else {
            Ok(Self {
                name: type_str.to_string(),
                params: None,
            })
        }
    }

    pub fn parse_ty(type_str: &str) -> IntrospectResult<Ty> {
        Self::parse(type_str).map(Ty::Item)
    }

    pub fn is_of_base_types(&self) -> bool {
        match (self.name.as_str(), &self.params) {
            ("Array" | "Span" | "Nullable" | "Felt252Dict" | "Option", Some(params))
                if params.len() == 1 =>
            {
                params[0].is_of_base_types()
            }
            ("Result", Some(params)) if params.len() == 2 => {
                params.iter().all(Ty::is_of_base_types)
            }
            (name, None) => is_base_type(name),
            _ => false,
        }
    }
}

impl FixedArray {
    pub fn parse(string: &str) -> IntrospectResult<Self> {
        let (type_str, size) =
            parse_fixed_array(string).ok_or(IntrospectError::FailedToParseType)?;
        Ty::parse(type_str).map(|ty| Self {
            ty,
            size: size.to_string(),
        })
    }
    pub fn parse_ty(string: &str) -> IntrospectResult<Ty> {
        Self::parse(string).map(|fa| Ty::FixedArray(Box::new(fa)))
    }
}

impl Ty {
    pub fn parse(type_str: &str) -> IntrospectResult<Self> {
        let type_str = type_str.trim();
        if type_str.starts_with('(') && type_str.ends_with(')') {
            Ty::parse_list(type_str).map(Ty::Tuple)
        } else if type_str.starts_with('[') && type_str.ends_with(']') {
            FixedArray::parse_ty(type_str)
        } else {
            TyItem::parse(type_str).map(Ty::Item)
        }
    }

    pub fn parse_list(type_str: &str) -> IntrospectResult<Vec<Self>> {
        match parse_list(type_str) {
            Some(types) => types
                .into_iter()
                .map(Ty::parse)
                .collect::<IntrospectResult<Vec<_>>>(),
            None => Err(IntrospectError::FailedToParseType),
        }
    }

    pub fn parse_wrapped(type_str: &str) -> IntrospectResult<(&str, Vec<Self>)> {
        let (wrapper, types) =
            parse_wrapped_types(type_str).ok_or(IntrospectError::FailedToParseType)?;
        let parsed_types: IntrospectResult<Vec<Ty>> = types.into_iter().map(Ty::parse).collect();
        parsed_types.map(|pts| (wrapper, pts))
    }

    pub fn get_core_type(&self) -> Option<CairoCoreType> {
        match self {
            Ty::Item(item) if item.params.is_none() => match item.name.as_str() {
                "felt252" => Some(CairoCoreType::Felt252),
                "bool" => Some(CairoCoreType::Bool),
                "u8" => Some(CairoCoreType::U8),
                "u16" => Some(CairoCoreType::U16),
                "u32" => Some(CairoCoreType::U32),
                "u64" => Some(CairoCoreType::U64),
                "u128" => Some(CairoCoreType::U128),
                "u256" => Some(CairoCoreType::U256),
                "u512" => Some(CairoCoreType::U512),
                "i8" => Some(CairoCoreType::I8),
                "i16" => Some(CairoCoreType::I16),
                "i32" => Some(CairoCoreType::I32),
                "i64" => Some(CairoCoreType::I64),
                "i128" => Some(CairoCoreType::I128),
                "bytes31" => Some(CairoCoreType::Bytes31),
                "ClassHash" | "starknet::ClassHash" => Some(CairoCoreType::ClassHash),
                "ContractAddress" | "starknet::ContractAddress" => {
                    Some(CairoCoreType::ContractAddress)
                }
                "EthAddress" | "starknet::EthAddress" => Some(CairoCoreType::EthAddress),
                "StorageAddress" | "starknet::StorageAddress" => {
                    Some(CairoCoreType::StorageAddress)
                }
                "StorageBaseAddress" | "starknet::storage_access::StorageBaseAddress" => {
                    Some(CairoCoreType::StorageBaseAddress)
                }
                "ByteArray" => Some(CairoCoreType::ByteArray),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn is_core_type(&self, core_type: CairoCoreType) -> bool {
        match self {
            Ty::Item(item) => item.name == core_type.type_str() && item.params.is_none(),
            _ => false,
        }
    }

    pub fn is_of_base_types(&self) -> bool {
        match self {
            Ty::Item(i) => i.is_of_base_types(),
            Ty::FixedArray(a) => a.ty.is_of_base_types(),
            Ty::Tuple(t) => t.iter().all(Ty::is_of_base_types),
        }
    }

    pub fn is_not_of_base_types(&self) -> bool {
        !self.is_of_base_types()
    }
}

fn tys_to_list_string(tys: &[Ty]) -> String {
    tys.iter().map(Ty::as_cairo).join(", ")
}

impl AsCairo for Ty {
    fn as_cairo(&self) -> String {
        match self {
            Ty::Item(item) => match &item.params {
                Some(params) => format!("{}<{}>", item.name, tys_to_list_string(params)),
                None => item.name.clone(),
            },
            Ty::Tuple(types) => {
                format!("({})", tys_to_list_string(types))
            }
            Ty::FixedArray(fixed_array) => {
                format!("[{}; {}]", fixed_array.ty.as_cairo(), fixed_array.size)
            }
        }
    }
}

pub fn is_base_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "felt252"
            | "bool"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "u256"
            | "u512"
            | "core::integer::u512"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "bytes31"
            | "ClassHash"
            | "starknet::ClassHash"
            | "ContractAddress"
            | "starknet::ContractAddress"
            | "EthAddress"
            | "starknet::EthAddress"
            | "StorageAddress"
            | "starknet::StorageAddress"
            | "StorageBaseAddress"
            | "starknet::storage_access::StorageBaseAddress"
            | "ByteArray"
    )
}

pub fn parse_wrapped_types(type_name: &str) -> Option<(&str, Vec<&str>)> {
    let start = type_name.find('<').unwrap();
    Some((&type_name[..start], parse_list(&type_name[start..])?))
}

pub fn parse_fixed_array(type_name: &str) -> Option<(&str, &str)> {
    let mut splits = type_name[1..type_name.len() - 1].rsplitn(2, ';');
    let len = splits.next()?.trim();
    let inner_type = splits.last()?.trim();
    Some((inner_type, len))
}

pub fn parse_list(type_name: &str) -> Option<Vec<&str>> {
    let inner = &type_name[1..type_name.len() - 1];
    let mut types: Vec<&str> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    let mut start = 0;

    for (i, c) in inner.chars().enumerate() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '<' => stack.push('>'),
            '{' => stack.push('}'),
            ')' | ']' | '>' | '}' => {
                if stack.pop() != Some(c) {
                    return None;
                }
            }
            ',' if stack.is_empty() => {
                types.push(inner[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }

    match stack.is_empty() {
        true => {
            if start < inner.len() {
                types.push(inner[start..].trim());
            }
            Some(types)
        }
        false => None,
    }
}

impl<'db> TryFromAst<'db, TypeClause<'db>> for Ty {
    fn try_from_ast(ast: TypeClause<'db>, db: &'db dyn Database) -> IntrospectResult<Self> {
        Ty::parse(&ast.to_string(db))
    }
}

impl<'db> TryFromAst<'db, OptionTypeClause<'db>> for Option<Ty> {
    fn try_from_ast(ast: OptionTypeClause<'db>, db: &'db dyn Database) -> IntrospectResult<Self> {
        match ast {
            OptionTypeClause::Empty(_) => Ok(None),
            OptionTypeClause::TypeClause(ty) => ty.ast_try_into(db).map(Some),
        }
    }
}
