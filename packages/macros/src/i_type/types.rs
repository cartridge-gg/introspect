pub use super::{IEnum, IExtract, IMember, IStruct, IVariant};
use crate::type_def::child_defs_tpl;
use crate::{AsCairo, I_PATH, Result, Ty};
use introspect_types::{ItemDefTrait, TypeDef};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub enum TypeDefVariant {
    Default,
    TypeDef(TypeDef),
    Fn(String),
}

pub trait ExtractTypeDef {
    type MacroAttribute;
    fn extract_type_def(
        &self,
        ty: &Ty,
        attributes: &[Self::MacroAttribute],
    ) -> Result<TypeDefVariant>;
    fn extract_option_type_def(
        &self,
        ty: &Option<Ty>,
        attributes: &[Self::MacroAttribute],
    ) -> Result<TypeDefVariant>;
}

impl TypeDefVariant {
    pub fn type_def(&self, ty: &Ty) -> String {
        match self {
            TypeDefVariant::Default => {
                format!("{I_PATH}::type_def::<{}>()", ty.as_cairo())
            }
            TypeDefVariant::TypeDef(type_def) => type_def.as_cairo(),
            TypeDefVariant::Fn(call) => call.clone(),
        }
    }
}

pub trait ToTypeDefVariant: ItemDefTrait + Sized {
    fn to_type_def_variant(self) -> TypeDefVariant;
}

impl<T> ToTypeDefVariant for T
where
    T: ItemDefTrait + Sized,
{
    fn to_type_def_variant(self) -> TypeDefVariant {
        TypeDefVariant::TypeDef(self.wrap_to_type_def())
    }
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

pub trait ITys {
    fn child_defs(&self) -> String;
}

impl ITys for [&Ty] {
    fn child_defs(&self) -> String {
        let mut defs: Vec<_> = self
            .iter()
            .filter(|t| !t.is_of_base_types())
            .map(|t| child_defs_tpl(&t.as_cairo()))
            .collect();
        match defs.len() {
            0 => "array![]".to_string(),
            1 => defs.pop().unwrap(),
            _ => format!("{I_PATH}::merge_defs(array![{}])", defs.join(", ")),
        }
    }
}
