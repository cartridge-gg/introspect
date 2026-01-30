use crate::{AstToString, IntrospectError as Error, IntrospectResult as Result};
use cairo_syntax_parser::{
    CairoWriteSlice, CairoWrite, Expr, ExprPath, FixedSizeArray as SynFixedSizeArray,
    OptionSizeHint, SizeHint,
};
use std::fmt::{Result as FmtResult, Write};

const PRIMITIVE_TYPES: &[&str] = &[
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

pub fn is_primitive_type(type_name: &str) -> bool {
    PRIMITIVE_TYPES.contains(&type_name)
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Ty {
    Item(TyItem),
    Tuple(Vec<Ty>),
    FixedArray(Box<FixedArray>),
}
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

impl TryFrom<Expr> for Ty {
    type Error = Error;
    fn try_from(expr: Expr) -> Result<Self> {
        match expr {
            Expr::Tuple(exprs) => exprs.try_into(),
            Expr::FixedSizeArray(arr) => arr.try_into(),
            Expr::Path(path) => path.try_into(),
            Expr::Parenthesized(expr) => (*expr).try_into(),
            _ => Error::FailedToParseType.into(),
        }
    }
}

impl TryFrom<Vec<Expr>> for Ty {
    type Error = Error;
    fn try_from(exprs: Vec<Expr>) -> Result<Self> {
        exprs.iter().map(TryFrom::try_from).collect().map(Ty::Tuple)
    }
}

impl TryFrom<SynFixedSizeArray> for FixedArray {
    type Error = Error;
    fn try_from(value: SynFixedSizeArray) -> Result<Self> {
        Ok(FixedArray {
            ty: Ty::from(*value.element),
            size: value.size.to_string(),
        })
    }
}

impl TryFrom<SynFixedSizeArray> for Ty {
    type Error = Error;
    fn try_from(array: SynFixedSizeArray) -> Result<Self> {
        FixedArray::try_from(array).map(|fa| Ty::FixedArray(Box::new(fa)))
    }
}

impl TryFrom<ExprPath> for TyItem {
    type Error = Error;
    fn try_from(path: ExprPath) -> Result<Self> {
        TyItem::parse(&path.to_string()).map_err(|_| Error::FailedToParseType)
    }
}

impl TryFrom<ExprPath> for Ty {
    type Error = Error;
    fn try_from(path: ExprPath) -> Result<Self> {
        TyItem::try_from(path).map(Ty::Item)
    }
}

pub enum CairoPrimitiveType {
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

impl CairoPrimitiveType {
    pub fn type_str(&self) -> &str {
        match self {
            CairoPrimitiveType::Felt252 => "felt252",
            CairoPrimitiveType::Bool => "bool",
            CairoPrimitiveType::U8 => "u8",
            CairoPrimitiveType::U16 => "u16",
            CairoPrimitiveType::U32 => "u32",
            CairoPrimitiveType::U64 => "u64",
            CairoPrimitiveType::U128 => "u128",
            CairoPrimitiveType::U256 => "u256",
            CairoPrimitiveType::U512 => "u512",
            CairoPrimitiveType::I8 => "i8",
            CairoPrimitiveType::I16 => "i16",
            CairoPrimitiveType::I32 => "i32",
            CairoPrimitiveType::I64 => "i64",
            CairoPrimitiveType::I128 => "i128",
            CairoPrimitiveType::Bytes31 => "bytes31",
            CairoPrimitiveType::ClassHash => "ClassHash",
            CairoPrimitiveType::ContractAddress => "ContractAddress",
            CairoPrimitiveType::EthAddress => "EthAddress",
            CairoPrimitiveType::StorageAddress => "StorageAddress",
            CairoPrimitiveType::StorageBaseAddress => "StorageBaseAddress",
            CairoPrimitiveType::ByteArray => "ByteArray",
        }
    }
}

impl TyItem {
    // pub fn parse(type_str: &str) -> IntrospectResult<Self> {
    //     if type_str.ends_with('>') {
    //         let (name, types) =
    //             parse_wrapped_types(type_str).ok_or(IntrospectError::FailedToParseType)?;
    //         let params = types
    //             .into_iter()
    //             .map(Ty::parse)
    //             .collect::<IntrospectResult<_>>()?;
    //         Ok(Self {
    //             name: name.to_string(),
    //             params: Some(params),
    //         })
    //     } else {
    //         Ok(Self {
    //             name: type_str.to_string(),
    //             params: None,
    //         })
    //     }
    // }

    // pub fn parse_ty(type_str: &str) -> IntrospectResult<Ty> {
    //     Self::parse(type_str).map(Ty::Item)
    // }

    pub fn is_core_type(&self) -> bool {
        match (self.name.as_str(), &self.params) {
            ("Array" | "Span" | "Nullable" | "Felt252Dict" | "Option", Some(params))
                if params.len() == 1 =>
            {
                params[0].is_core_type()
            }
            ("Result", Some(params)) if params.len() == 2 => params.iter().all(Ty::is_core_type),
            (name, None) => is_primitive_type(name),
            _ => false,
        }
    }
}

// impl FixedArray {
//     pub fn parse(string: &str) -> IntrospectResult<Self> {
//         let (type_str, size) =
//             parse_fixed_array(string).ok_or(IntrospectError::FailedToParseType)?;
//         Ty::parse(type_str).map(|ty| Self {
//             ty,
//             size: size.to_string(),
//         })
//     }
//     pub fn parse_ty(string: &str) -> IntrospectResult<Ty> {
//         Self::parse(string).map(|fa| Ty::FixedArray(Box::new(fa)))
//     }
// }

impl Ty {
    // pub fn parse(type_str: &str) -> IntrospectResult<Self> {
    //     let type_str = type_str.trim();
    //     if type_str.starts_with('(') && type_str.ends_with(')') {
    //         Ty::parse_list(type_str).map(Ty::Tuple)
    //     } else if type_str.starts_with('[') && type_str.ends_with(']') {
    //         FixedArray::parse_ty(type_str)
    //     } else {
    //         TyItem::parse(type_str).map(Ty::Item)
    //     }
    // }

    // pub fn parse_list(type_str: &str) -> IntrospectResult<Vec<Self>> {
    //     match parse_list(type_str) {
    //         Some(types) => types
    //             .into_iter()
    //             .map(Ty::parse)
    //             .collect::<IntrospectResult<Vec<_>>>(),
    //         None => Err(IntrospectError::FailedToParseType),
    //     }
    // }

    // pub fn parse_wrapped(type_str: &str) -> IntrospectResult<(&str, Vec<Self>)> {
    //     let (wrapper, types) =
    //         parse_wrapped_types(type_str).ok_or(IntrospectError::FailedToParseType)?;
    //     let parsed_types: IntrospectResult<Vec<Ty>> = types.into_iter().map(Ty::parse).collect();
    //     parsed_types.map(|pts| (wrapper, pts))
    // }

    pub fn get_primitive_type(&self) -> Option<CairoPrimitiveType> {
        match self {
            Ty::Item(item) if item.params.is_none() => match item.name.as_str() {
                "felt252" => Some(CairoPrimitiveType::Felt252),
                "bool" => Some(CairoPrimitiveType::Bool),
                "u8" => Some(CairoPrimitiveType::U8),
                "u16" => Some(CairoPrimitiveType::U16),
                "u32" => Some(CairoPrimitiveType::U32),
                "u64" => Some(CairoPrimitiveType::U64),
                "u128" => Some(CairoPrimitiveType::U128),
                "u256" => Some(CairoPrimitiveType::U256),
                "u512" => Some(CairoPrimitiveType::U512),
                "i8" => Some(CairoPrimitiveType::I8),
                "i16" => Some(CairoPrimitiveType::I16),
                "i32" => Some(CairoPrimitiveType::I32),
                "i64" => Some(CairoPrimitiveType::I64),
                "i128" => Some(CairoPrimitiveType::I128),
                "bytes31" => Some(CairoPrimitiveType::Bytes31),
                "ClassHash" | "starknet::ClassHash" => Some(CairoPrimitiveType::ClassHash),
                "ContractAddress" | "starknet::ContractAddress" => {
                    Some(CairoPrimitiveType::ContractAddress)
                }
                "EthAddress" | "starknet::EthAddress" => Some(CairoPrimitiveType::EthAddress),
                "StorageAddress" | "starknet::StorageAddress" => {
                    Some(CairoPrimitiveType::StorageAddress)
                }
                "StorageBaseAddress" | "starknet::storage_access::StorageBaseAddress" => {
                    Some(CairoPrimitiveType::StorageBaseAddress)
                }
                "ByteArray" => Some(CairoPrimitiveType::ByteArray),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn matches_primitive(&self, core_type: CairoPrimitiveType) -> bool {
        match self {
            Ty::Item(item) => item.name == core_type.type_str() && item.params.is_none(),
            _ => false,
        }
    }

    pub fn is_core_type(&self) -> bool {
        match self {
            Ty::Item(i) => i.is_core_type(),
            Ty::FixedArray(a) => a.ty.is_core_type(),
            Ty::Tuple(t) => t.iter().all(Ty::is_core_type),
        }
    }
}

impl CairoWrite for Ty {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        match self {
            Ty::Item(e) => e.cwrite(buf),
            Ty::Tuple(types) => types.cwrite_tuple(buf),
            Ty::FixedArray(fixed_array) => fixed_array.cwrite(buf),
        }
    }
    fn size_hint(&self) -> usize {
        match self {
            Ty::Item(e) => e.size_hint(),
            Ty::Tuple(types) => types.size_hint_tuple(),
            Ty::FixedArray(fixed_array) => fixed_array.size_hint(),
        }
    }
}

impl CairoWrite for TyItem {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.name.cwrite(buf);
        if let Some(params) = &self.params {
            params.cwrite_csv_angled(buf);
        }
    }
    fn size_hint(&self) -> usize {
        self.name.size_hint() + self.params.size_hint_option::<2, 0>()
    }
}

impl CairoWrite for FixedArray {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.ty.cwrite_prefixed(buf, '[');
        self.size.cwrite_prefixed_str(buf, "; ");
        buf.push_token_char(']');
    }
    fn size_hint(&self) -> usize {
        self.ty.size_hint() + self.size.len() + 4
    }
}

// pub fn parse_wrapped_types(type_name: &str) -> Option<(&str, Vec<&str>)> {
//     let start = type_name.find('<').unwrap();
//     Some((&type_name[..start], parse_list(&type_name[start..])?))
// }

// pub fn parse_fixed_array(type_name: &str) -> Option<(&str, &str)> {
//     let mut splits = type_name[1..type_name.len() - 1].rsplitn(2, ';');
//     let len = splits.next()?.trim();
//     let inner_type = splits.last()?.trim();
//     Some((inner_type, len))
// }

// pub fn parse_list(type_name: &str) -> Option<Vec<&str>> {
//     let inner = &type_name[1..type_name.len() - 1];
//     let mut types: Vec<&str> = Vec::new();
//     let mut stack: Vec<char> = Vec::new();
//     let mut start = 0;

//     for (i, c) in inner.chars().enumerate() {
//         match c {
//             '(' => stack.push(')'),
//             '[' => stack.push(']'),
//             '<' => stack.push('>'),
//             '{' => stack.push('}'),
//             ')' | ']' | '>' | '}' => {
//                 if stack.pop() != Some(c) {
//                     return None;
//                 }
//             }
//             ',' if stack.is_empty() => {
//                 types.push(inner[start..i].trim());
//                 start = i + 1;
//             }
//             _ => {}
//         }
//     }

//     match stack.is_empty() {
//         true => {
//             if start < inner.len() {
//                 types.push(inner[start..].trim());
//             }
//             Some(types)
//         }
//         false => None,
//     }
// }

// impl<'db> TryFromAst<'db, TypeClause<'db>> for Ty {
//     fn try_from_ast(ast: TypeClause<'db>, db: &'db dyn Database) -> IntrospectResult<Self> {
//         Ty::parse(&ast.to_string(db))
//     }
// }

// impl<'db> TryFromAst<'db, OptionTypeClause<'db>> for Option<Ty> {
//     fn try_from_ast(ast: OptionTypeClause<'db>, db: &'db dyn Database) -> IntrospectResult<Self> {
//         match ast {
//             OptionTypeClause::Empty(_) => Ok(None),
//             OptionTypeClause::TypeClause(ty) => ty.ast_try_into(db).map(Some),
//         }
//     }
// }
