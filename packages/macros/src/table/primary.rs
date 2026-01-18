use crate::type_def::CairoElementDef;
use crate::{AsCairo, AsCairoBytes, IAttribute, Ty};
use introspect_types::PrimaryTypeDef;

#[derive(Clone, Debug)]
pub struct PrimaryDef {
    pub name: String,
    pub member: Option<String>,
    pub attributes: Vec<IAttribute>,
    pub ty: Ty,
    pub type_def: PrimaryTypeDefVariant,
}

#[derive(Clone, Debug)]
pub enum PrimaryTypeDefVariant {
    Default,
    TypeDef(PrimaryTypeDef),
    Fn(String),
}

impl PrimaryTypeDefVariant {
    pub fn type_def(&self, ty: &Ty, i_path: &str) -> String {
        match self {
            PrimaryTypeDefVariant::Default => {
                format!("{i_path}::primary_type_def::<{}>()", ty.as_cairo())
            }
            PrimaryTypeDefVariant::TypeDef(type_def) => type_def.as_element_def(i_path),
            PrimaryTypeDefVariant::Fn(call) => call.clone(),
        }
    }
}

impl CairoElementDef for PrimaryTypeDef {
    fn as_element_def(&self, i_path: &str) -> String {
        match &self {
            PrimaryTypeDef::Bytes31E(encoding) => format!(
                "{i_path}::PrimaryTypeDef::{}({})",
                self.item_name(),
                encoding.as_cairo_byte_array()
            ),
            _ => as_unit_primary_type_def(i_path, self.item_name()),
        }
    }
}

pub fn as_unit_primary_type_def(i_path: &str, variant: &str) -> String {
    format!("{i_path}::PrimaryTypeDef::{}", variant)
}
