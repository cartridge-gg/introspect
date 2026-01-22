use introspect_types::{ByteArrayEncodedDef, Bytes31EncodedDef, TypeDef};

use super::ToTypeDefVariant;
use crate::i_type::{AttributeParser, AttributeVariant, TypeDefVariant};
use crate::ty::CairoPrimitiveType;
use crate::{Attribute, AttributesTrait, IntrospectError, IntrospectResult, Ty};

#[derive(Clone, Debug)]
pub enum TypeMod {
    Raw,
    Encoded(String),
}

pub trait TypeModMemberTrait {
    fn get_mut_type_mod(&mut self) -> &mut Option<TypeMod>;
    fn set_type_mod(&mut self, type_mod: TypeMod) -> IntrospectResult<()> {
        match self.get_mut_type_mod().replace(type_mod) {
            Some(_) => Err(IntrospectError::MultipleTypeModifiers),
            None => Ok(()),
        }
    }
    fn extract_type_mod_return_empty<T>(
        &mut self,
        attribute: &Attribute,
    ) -> Option<IntrospectResult<Vec<T>>> {
        self.extract_type_mod(attribute)
            .map(|res| res.map(|_| Vec::new()))
    }
    fn extract_type_mod(&mut self, attribute: &Attribute) -> Option<IntrospectResult<()>> {
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
}

pub trait TypeModTrait: Sized {
    fn get_type_mod(self) -> Option<TypeMod>;
    fn get_type_def(self, ty: &Ty) -> IntrospectResult<TypeDefVariant> {
        match self.get_type_mod() {
            Some(TypeMod::Raw) => match ty.get_core_type() {
                Some(CairoPrimitiveType::ByteArray) => {
                    Ok(TypeDefVariant::TypeDef(TypeDef::ByteArray))
                }
                Some(CairoPrimitiveType::Bytes31) => Ok(TypeDefVariant::TypeDef(TypeDef::Bytes31)),
                _ => Err(IntrospectError::UnsupportedRawType),
            },
            Some(TypeMod::Encoded(encoded)) => match ty.get_core_type() {
                Some(CairoPrimitiveType::ByteArray) => {
                    Ok(ByteArrayEncodedDef::new(encoded).to_type_def_variant())
                }
                Some(CairoPrimitiveType::Bytes31) => {
                    Ok(Bytes31EncodedDef::new(encoded).to_type_def_variant())
                }
                _ => Err(IntrospectError::UnsupportedEncodedType),
            },
            _ => Ok(TypeDefVariant::Default),
        }
    }
    fn get_type_def_option(self, ty: &Option<Ty>) -> IntrospectResult<TypeDefVariant> {
        match ty {
            Some(t) => self.get_type_def(t),
            None => Ok(TypeDefVariant::Default),
        }
    }
}

impl TypeModTrait for Option<TypeMod> {
    fn get_type_mod(self) -> Option<TypeMod> {
        self
    }
}

impl TypeModMemberTrait for Option<TypeMod> {
    fn get_mut_type_mod(&mut self) -> &mut Option<TypeMod> {
        self
    }
}

impl<T> AttributeParser<Option<TypeMod>> for T
where
    T: AttributesTrait,
{
    type Error = IntrospectError;
    fn parse_attribute(
        &self,
        _item: &mut T,
        type_mod: &mut Option<TypeMod>,
        attribute: Attribute,
    ) -> IntrospectResult<Vec<AttributeVariant>> {
        match type_mod.extract_type_mod_return_empty(&attribute) {
            None => attribute.into(),
            Some(res) => res,
        }
    }
}
