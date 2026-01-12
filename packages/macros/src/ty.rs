use crate::{AsCairo, AstToString, AstTryInto, IntrospectError, Result, TryFromAst};
use cairo_lang_syntax::node::ast::{OptionTypeClause, TypeClause};
use itertools::Itertools;
use salsa::Database;

#[derive(Clone, Debug, PartialEq)]
pub struct FixedArray {
    pub ty: Ty,
    pub size: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyItem {
    pub name: String,
    pub params: Option<Vec<Ty>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Item(TyItem),
    Tuple(Vec<Ty>),
    FixedArray(Box<FixedArray>),
}

impl TyItem {
    pub fn parse(type_str: &str) -> Result<Self> {
        if type_str.ends_with('>') {
            let (name, types) =
                parse_wrapped_types(type_str).ok_or(IntrospectError::FailedToParseType)?;
            let params = types.into_iter().map(Ty::parse).collect::<Result<_>>()?;
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

    pub fn parse_ty(type_str: &str) -> Result<Ty> {
        Self::parse(type_str).map(Ty::Item)
    }
}

impl FixedArray {
    pub fn parse(string: &str) -> Result<Self> {
        let (type_str, size) =
            parse_fixed_array(string).ok_or(IntrospectError::FailedToParseType)?;
        Ty::parse(type_str).map(|ty| Self {
            ty,
            size: size.to_string(),
        })
    }
    pub fn parse_ty(string: &str) -> Result<Ty> {
        Self::parse(string).map(|fa| Ty::FixedArray(Box::new(fa)))
    }
}

impl Ty {
    pub fn parse(type_str: &str) -> Result<Self> {
        let type_str = type_str.trim();
        if type_str.starts_with('(') && type_str.ends_with(')') {
            Ty::parse_list(type_str).map(Ty::Tuple)
        } else if type_str.starts_with('[') && type_str.ends_with(']') {
            FixedArray::parse_ty(type_str)
        } else {
            TyItem::parse(type_str).map(Ty::Item)
        }
    }

    pub fn parse_list(type_str: &str) -> Result<Vec<Self>> {
        match parse_list(type_str) {
            Some(types) => types.into_iter().map(Ty::parse).collect::<Result<Vec<_>>>(),
            None => Err(IntrospectError::FailedToParseType),
        }
    }

    pub fn parse_wrapped(type_str: &str) -> Result<(&str, Vec<Self>)> {
        let (wrapper, types) =
            parse_wrapped_types(type_str).ok_or(IntrospectError::FailedToParseType)?;
        let parsed_types: Result<Vec<Ty>> = types.into_iter().map(Ty::parse).collect();
        parsed_types.map(|pts| (wrapper, pts))
    }

    pub fn is_of_base_types(&self) -> bool {
        match self {
            Ty::Item(i) => is_base_type(&i.name),
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
    fn try_from_ast(ast: TypeClause<'db>, db: &'db dyn Database) -> Result<Self> {
        Ty::parse(&ast.to_string(db))
    }
}

impl<'db> TryFromAst<'db, OptionTypeClause<'db>> for Option<Ty> {
    fn try_from_ast(ast: OptionTypeClause<'db>, db: &'db dyn Database) -> Result<Self> {
        match ast {
            OptionTypeClause::Empty(_) => Ok(None),
            OptionTypeClause::TypeClause(ty) => ty.ast_try_into(db).map(Some),
        }
    }
}
