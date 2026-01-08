pub mod enums;
pub mod parser;
pub mod structs;

pub use enums::{IEnum, IVariant};
pub use parser::{DefaultIExtractor, IExtract};
pub use structs::{IMember, IStruct};

use crate::params::GenericParams;
use crate::ty::Tys;
use crate::{AsCairo, ModuleItem, Ty};
use introspect_types::TypeDef;
use std::ops::Deref;

pub enum TypeDefVariant {
    Default,
    TypeDef(TypeDef),
    Fn(String),
}

pub trait ToTypeDef {
    fn to_type_def(&self) -> String;
}

pub trait ToTypeDefs {
    fn to_type_defs(&self) -> Vec<String>;
    fn to_type_defs_csv(&self) -> String {
        self.to_type_defs().join(",")
    }
    fn to_type_defs_span(&self) -> String {
        format!("[{}].span", self.to_type_defs_csv())
    }
    fn to_type_defs_array(&self) -> String {
        format!("array![{}]", self.to_type_defs_csv())
    }
}

impl<T, S> ToTypeDefs for T
where
    T: Deref<Target = [S]>,
    S: ToTypeDef,
{
    fn to_type_defs(&self) -> Vec<String> {
        self.iter().map(S::to_type_def).collect()
    }
}

pub trait IntrospectItemTrait {
    type ModuleType;
    fn kind(&self) -> &str;
    fn name(&self) -> &str;
    fn generic_params(&self) -> &GenericParams;
    fn child_types(&self) -> Vec<Ty>;
    fn generics_clause(&self) -> String {
        self.generic_params().as_cairo()
    }
    fn full_name(&self) -> String {
        format!("{}{}", self.name(), self.generics_clause())
    }
    fn generics_call(&self) -> String {
        self.generic_params().as_cairo_callable()
    }
    fn full_call(&self) -> String {
        format!("{}{}", self.name(), self.generics_call())
    }
    fn generics_with_traits(&self, traits: &[&str]) -> String {
        self.generic_params().with_trait_bounds(traits)
    }
    fn child_defs(&self) -> String {
        self.child_types().child_defs()
    }
}

pub enum IntrospectItem {
    Struct(IStruct),
    Enum(IEnum),
}

impl IntrospectItemTrait for IntrospectItem {
    type ModuleType = ModuleItem;
    fn kind(&self) -> &str {
        match self {
            IntrospectItem::Struct(s) => s.kind(),
            IntrospectItem::Enum(e) => e.kind(),
        }
    }
    fn name(&self) -> &str {
        match self {
            IntrospectItem::Struct(s) => s.name(),
            IntrospectItem::Enum(e) => e.name(),
        }
    }
    fn generic_params(&self) -> &GenericParams {
        match self {
            IntrospectItem::Struct(s) => s.generic_params(),
            IntrospectItem::Enum(e) => e.generic_params(),
        }
    }
    fn child_types(&self) -> Vec<Ty> {
        match self {
            IntrospectItem::Struct(s) => s.child_types(),
            IntrospectItem::Enum(e) => e.child_types(),
        }
    }
}
