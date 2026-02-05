use crate::i_type::TypeDefVariant;
use crate::type_def::CairoElementDef;
use crate::{AsCairoBytes, IAttribute, IntrospectError, Ty};
use cairo_syntax_parser::CairoWrite;
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

impl TryFrom<TypeDefVariant> for PrimaryTypeDefVariant {
    type Error = IntrospectError;
    fn try_from(value: TypeDefVariant) -> Result<Self, Self::Error> {
        match value {
            TypeDefVariant::Default => Ok(PrimaryTypeDefVariant::Default),
            TypeDefVariant::TypeDef(type_def) => {
                let primary_type_def = type_def
                    .try_into()
                    .map_err(|_| IntrospectError::UnsupportedPrimaryType)?;
                Ok(PrimaryTypeDefVariant::TypeDef(primary_type_def))
            }
            TypeDefVariant::Fn(call) => Ok(PrimaryTypeDefVariant::Fn(call)),
        }
    }
}

impl PrimaryTypeDefVariant {
    pub fn type_def(&self, ty: &Ty, i_path: &str) -> String {
        match self {
            PrimaryTypeDefVariant::Default => {
                format!(
                    "{i_path}::primary_type_def::<{}>()",
                    CairoWrite::<String>::to_cairo(ty)
                )
            }
            PrimaryTypeDefVariant::TypeDef(type_def) => type_def.as_element_def(i_path),
            PrimaryTypeDefVariant::Fn(call) => call.clone(),
        }
    }
}

impl CairoElementDef for PrimaryTypeDef {
    fn as_element_def(&self, i_path: &str) -> String {
        match &self {
            PrimaryTypeDef::Bytes31Encoded(e) => format!(
                "{i_path}::PrimaryTypeDef::{}({})",
                self.item_name(),
                e.encoding.as_cairo_byte_array()
            ),
            _ => as_unit_primary_type_def(i_path, self.item_name()),
        }
    }
}

pub fn as_unit_primary_type_def(i_path: &str, variant: &str) -> String {
    format!("{i_path}::PrimaryTypeDef::{}", variant)
}
