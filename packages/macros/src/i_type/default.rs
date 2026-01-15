use introspect_types::{ByteArrayEDef, Bytes31EDef, TypeDef};

use super::ToTypeDefVariant;
use crate::i_type::attribute::MacroAttributeTrait;
use crate::i_type::extraction::IExtractor;
use crate::i_type::{AttributeParser, AttributeVariant, ExtractTypeDef, TypeDefVariant};
use crate::ty::CairoCoreType;
use crate::{Attribute, AttributeCallType, AttributesTrait, IntrospectError, Result, Ty};

pub struct DefaultIExtractor {
    pub attribute_type: AttributeCallType,
}

impl IExtractor for DefaultIExtractor {
    type Error = IntrospectError;
    fn get_attribute_call_type(&self) -> &AttributeCallType {
        &self.attribute_type
    }
    fn derive_call_error(&self) -> IntrospectError {
        IntrospectError::DeriveCallNotSupported
    }
}

#[derive(Clone, Debug)]
pub enum TypeMod {
    Raw,
    Encoded(String),
}

pub trait TypeModTrait
where
    Self: Sized,
{
    fn get_type_mod(&self) -> Option<TypeMod>;
    fn set_type_mod(&mut self, type_mod: TypeMod) -> Result<()>;
    fn parse_attribute(&mut self, attribute: &Attribute) -> Option<Result<()>> {
        match attribute.name.as_str() {
            "raw" => match &attribute.args {
                None => Some(self.set_type_mod(TypeMod::Raw)),
                _ => Some(attribute.format_err()),
            },
            "encoded" => Some(
                attribute
                    .single_unnamed_arg()
                    .and_then(|e| self.set_type_mod(TypeMod::Encoded(e))),
            ),
            _ => None,
        }
    }
    fn get_type_def(&self, ty: &Ty) -> Result<TypeDefVariant> {
        match self.get_type_mod() {
            Some(TypeMod::Raw) => match ty.get_core_type() {
                Some(CairoCoreType::ByteArray) => Ok(TypeDefVariant::TypeDef(TypeDef::ByteArray)),
                Some(CairoCoreType::Bytes31) => Ok(TypeDefVariant::TypeDef(TypeDef::Bytes31)),
                _ => Err(IntrospectError::UnsupportedRawType),
            },
            Some(TypeMod::Encoded(encoded)) => match ty.get_core_type() {
                Some(CairoCoreType::ByteArray) => {
                    Ok(ByteArrayEDef::new(encoded).to_type_def_variant())
                }
                Some(CairoCoreType::Bytes31) => Ok(Bytes31EDef::new(encoded).to_type_def_variant()),
                _ => Err(IntrospectError::UnsupportedEncodedType),
            },
            _ => Ok(TypeDefVariant::Default),
        }
    }
}

impl TypeModTrait for Option<TypeMod> {
    fn set_type_mod(&mut self, type_mod: TypeMod) -> Result<()> {
        match self {
            Some(_) => Err(IntrospectError::MultipleTypeModifiers),
            None => {
                *self = Some(type_mod);
                Ok(())
            }
        }
    }
    fn get_type_mod(&self) -> Option<TypeMod> {
        self.clone()
    }
}

impl DefaultIExtractor {
    pub fn new(attribute_type: AttributeCallType) -> Self {
        DefaultIExtractor { attribute_type }
    }
}

impl<T> AttributeParser<T, Option<TypeMod>> for DefaultIExtractor
where
    T: AttributesTrait,
{
    type Error = IntrospectError;
    fn parse_attribute(
        &self,
        _item: &mut T,
        type_mod: &mut Option<TypeMod>,
        attribute: Attribute,
    ) -> Result<Vec<AttributeVariant>> {
        match type_mod.parse_attribute(&attribute) {
            None => Ok(attribute.into()),
            Some(res) => res.map(|_| Vec::new()),
        }
    }
}
