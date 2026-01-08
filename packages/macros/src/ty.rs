use std::ops::Deref;

use crate::{AstInto, AstToString, FromAst};
use cairo_lang_syntax::node::ast::{OptionTypeClause, TypeClause};
use salsa::Database;

#[derive(Clone)]
pub struct Ty(pub String);

impl Deref for Ty {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
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

pub fn get_inner_type(type_name: &str) -> String {
    let start = type_name.find('<').unwrap();
    type_name[start + 1..type_name.len() - 1].to_string()
}

pub fn get_fixed_array_inner_type(type_name: &str) -> &str {
    type_name[1..type_name.len() - 1]
        .rsplitn(2, ';')
        .last()
        .unwrap()
        .trim()
}

pub fn get_tuple_inner_types(type_name: &str) -> Vec<String> {
    let inner = &type_name[1..type_name.len() - 1];
    let types: Vec<String> = inner
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    types
}

pub fn is_of_base_types(type_name: &str) -> bool {
    if type_name.ends_with(">")
        && (["Span<", "Array<", "Option<"]
            .iter()
            .any(|g| type_name.starts_with(g)))
    {
        is_of_base_types(&get_inner_type(&type_name))
    } else if type_name.starts_with("[") && type_name.ends_with("]") {
        is_of_base_types(get_fixed_array_inner_type(type_name))
    } else if type_name.starts_with("(") && type_name.ends_with(")") {
        get_tuple_inner_types(type_name)
            .iter()
            .all(|e| is_of_base_types(e))
    } else {
        is_base_type(type_name)
    }
}

impl Ty {
    pub fn is_base_type(&self) -> bool {
        is_base_type(&self.0)
    }

    pub fn is_of_base_types(&self) -> bool {
        is_of_base_types(&self.0)
    }

    pub fn child_defs(&self) -> String {
        format!("introspect::child_defs::<{}>()", &self.0)
    }

    pub fn child_defs_if_needed(&self) -> Option<String> {
        match self.is_of_base_types() {
            true => None,
            false => Some(self.child_defs()),
        }
    }
}

impl<'db> FromAst<'db, TypeClause<'db>> for Ty {
    fn from_ast(ast: TypeClause<'db>, db: &'db dyn Database) -> Self {
        Ty(ast.to_string(db))
    }
}

impl<'db> FromAst<'db, OptionTypeClause<'db>> for Option<Ty> {
    fn from_ast(ast: OptionTypeClause<'db>, db: &'db dyn Database) -> Self {
        match ast {
            OptionTypeClause::Empty(_) => None,
            OptionTypeClause::TypeClause(ty) => Some(ty.ast_into(db)),
        }
    }
}

pub trait Tys {
    fn child_defs(&self) -> String;
}

impl Tys for Vec<Ty> {
    fn child_defs(&self) -> String {
        let defs: Vec<String> = self.iter().filter_map(Ty::child_defs_if_needed).collect();
        match defs.len() {
            0 => "array![]".to_string(),
            1 => defs[0].clone(),
            _ => format!("introspect::merge_defs(array![{}])", defs.join(", ")),
        }
    }
}
