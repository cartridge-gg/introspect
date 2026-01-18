pub use super::{IEnum, IExtract, IMember, IStruct, IVariant};
use crate::type_def::collect_child_defs_tpl;
use crate::{AsCairo, CairoTypeDef, I_PATH, IntrospectResult, Ty};
use introspect_types::{ItemDefTrait, TypeDef};
use itertools::Itertools;

#[derive(Clone, Debug, Default)]
pub enum TypeDefVariant {
    #[default]
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
    ) -> IntrospectResult<TypeDefVariant>;
    fn extract_option_type_def(
        &self,
        ty: &Option<Ty>,
        attributes: &[Self::MacroAttribute],
    ) -> IntrospectResult<TypeDefVariant>;
}

impl TypeDefVariant {
    pub fn type_def(&self, ty: &Ty, i_path: &str) -> String {
        match self {
            TypeDefVariant::Default => {
                format!("{I_PATH}::type_def::<{}>()", ty.as_cairo())
            }
            TypeDefVariant::TypeDef(type_def) => type_def.as_type_def(i_path),
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

// pub trait ToTypeDef {
//     fn to_type_def(&self) -> String;
// }

// pub trait ToTypeDefs {
//     fn to_type_defs(&self) -> Vec<String>;
//     fn to_type_defs_csv(&self) -> String {
//         self.to_type_defs().join(",")
//     }
//     fn to_type_defs_span(&self) -> String {
//         format!("[{}].span()", self.to_type_defs_csv())
//     }
//     fn to_type_defs_array(&self) -> String {
//         format!("array![{}]", self.to_type_defs_csv())
//     }
// }

// impl<T, S> ToTypeDefs for T
// where
//     T: Deref<Target = [S]>,
//     S: ToTypeDef,
// {
//     fn to_type_defs(&self) -> Vec<String> {
//         self.iter().map(S::to_type_def).collect()
//     }
// }

pub trait ITys {
    fn collect_child_defs(&self, i_path: &str) -> String;
}

impl ITys for [&Ty] {
    fn collect_child_defs(&self, i_path: &str) -> String {
        self.iter()
            .unique()
            .filter(|t| !t.is_of_base_types())
            .map(|t| collect_child_defs_tpl(i_path, &t.as_cairo()))
            .join("\n")
    }
}
