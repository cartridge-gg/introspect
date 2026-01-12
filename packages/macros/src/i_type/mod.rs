pub mod enums;
pub mod extraction;
pub mod structs;
use crate::params::GenericParams;
use crate::type_def::child_defs_tpl;
use crate::{AsCairo, I_PATH, Item, ItemTrait, Result, Ty};
pub use enums::{IEnum, IVariant};
pub use extraction::{DefaultIExtractor, IExtract};
use introspect_types::TypeDef;
use std::ops::Deref;
pub use structs::{IMember, IStruct};

#[derive(Clone, Debug)]
pub enum TypeDefVariant {
    Default,
    TypeDef(TypeDef),
    Fn(String),
}

impl TypeDefVariant {
    pub fn type_def(&self, ty: Ty) -> String {
        match self {
            TypeDefVariant::Default => {
                format!("introspect::Introspect::<{}>::type_def()", ty.as_cairo())
            }
            TypeDefVariant::TypeDef(type_def) => type_def.as_cairo(),
            TypeDefVariant::Fn(call) => call.clone(),
        }
    }
}

pub trait IType {
    type SyntaxType;
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
        format!("[{}].span()", self.to_type_defs_csv())
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

impl Ty {
    fn child_defs_if_not_of_base_types(&self) -> Option<String> {
        match self.is_of_base_types() {
            true => None,
            false => Some(child_defs_tpl(&self.as_cairo())),
        }
    }
}

pub trait IntrospectItemTrait {
    type ModuleType;
    fn kind(&self) -> &str;
    fn child_types(&self) -> Vec<Ty>;
    fn child_defs(&self) -> String {
        let mut defs: Vec<_> = self
            .child_types()
            .iter()
            .filter_map(Ty::child_defs_if_not_of_base_types)
            .collect();
        match defs.len() {
            0 => "array![]".to_string(),
            1 => defs.pop().unwrap(),
            _ => format!("{I_PATH}::merge_defs(array![{}])", defs.join(", ")),
        }
    }
}

pub enum IItem {
    Struct(IStruct),
    Enum(IEnum),
}

impl ItemTrait for IItem {
    fn name(&self) -> &str {
        match self {
            IItem::Struct(s) => s.name(),
            IItem::Enum(e) => e.name(),
        }
    }
    fn generic_params(&self) -> &GenericParams {
        match self {
            IItem::Struct(s) => s.generic_params(),
            IItem::Enum(e) => e.generic_params(),
        }
    }
}

impl IntrospectItemTrait for IItem {
    type ModuleType = Item;
    fn kind(&self) -> &str {
        match self {
            IItem::Struct(s) => s.kind(),
            IItem::Enum(e) => e.kind(),
        }
    }
    fn child_types(&self) -> Vec<Ty> {
        match self {
            IItem::Struct(s) => s.child_types(),
            IItem::Enum(e) => e.child_types(),
        }
    }
}

impl IExtract<IItem> for DefaultIExtractor {
    type SyntaxType = Item;
    fn iextract(&self, item: &mut Item) -> Result<IItem> {
        match item {
            Item::Struct(s) => self.iextract(s).map(IItem::Struct),
            Item::Enum(e) => self.iextract(e).map(IItem::Enum),
        }
    }
}
