use crate::items::TypeDefVariant;
use crate::{Attribute, IAttribute, Result, Ty};

pub trait IExtract<T, S> {
    fn iextract(&self, module: &mut S) -> Result<T>;
    fn iextracts(&self, modules: &mut [S]) -> Result<Vec<T>> {
        modules.iter_mut().map(|m| self.iextract(m)).collect()
    }
}

pub trait IExtractWith<T, S, C> {
    fn iextract_with(&self, module: &mut S, context: &C) -> Result<T>;
    fn iextracts_with(&self, modules_with: &mut [(S, C)]) -> Result<Vec<T>> {
        modules_with
            .iter_mut()
            .map(|(m, c)| self.iextract_with(m, c))
            .collect()
    }
}

pub struct DefaultIExtractor;

pub enum AttributeVariant {
    Emit(IAttribute),
    Macro(MacroAttribute),
    Ignore(Attribute),
}

pub enum MacroAttribute {
    Raw,
    Encoded(String),
}

impl DefaultIExtractor {
    pub fn new() -> Self {
        DefaultIExtractor {}
    }

    pub fn parse_type_def(&self, ty: &Ty, attributes: &[MacroAttribute]) -> TypeDefVariant {
        TypeDefVariant::Default
    }

    pub fn extract_attributes(
        &self,
        attributes: Vec<Attribute>,
    ) -> Result<(Vec<Attribute>, Vec<IAttribute>, Vec<MacroAttribute>)> {
        let mut macro_attributes = Vec::new();
        let mut iattributes = Vec::new();
        let mut other_attributes = Vec::new();
        for attr in attributes {
            let parsed = self.parse_attribute(attr)?;
            for p in parsed {
                match p {
                    AttributeVariant::Macro(m) => macro_attributes.push(m),
                    AttributeVariant::Emit(i) => iattributes.push(i),
                    AttributeVariant::Ignore(o) => other_attributes.push(o),
                }
            }
        }
        Ok((other_attributes, iattributes, macro_attributes))
    }

    pub fn parse_attribute(&self, attribute: Attribute) -> Result<Vec<AttributeVariant>> {
        match (attribute.name.as_str(), &attribute.args) {
            ("i_raw", None) => Ok(vec![AttributeVariant::Macro(MacroAttribute::Raw)]),
            ("encoded", Some(_)) => attribute
                .single_unnamed_arg()
                .map(|arg| vec![AttributeVariant::Macro(MacroAttribute::Encoded(arg))]),
            ("encoded" | "i_raw", _) => attribute.format_err(),
            _ => Ok(vec![AttributeVariant::Ignore(attribute)]),
        }
    }
}
