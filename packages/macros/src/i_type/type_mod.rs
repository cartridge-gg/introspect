use super::ToTypeDefVariant;
use crate::i_type::{AttributeParser, AttributeVariant, TypeDefVariant};
use crate::ty::CairoPrimitiveType;
use crate::{Attribute, AttributesTrait, IntrospectError, IntrospectResult, Ty};
use introspect_rust_macros::macro_attributes;
use introspect_types::{ByteArrayEncodedDef, Bytes31EncodedDef, TypeDef};
use std::mem;

#[derive(Clone, Debug, Default)]
pub enum TypeMod {
    #[default]
    None,
    Raw,
    Encoded(String),
}

pub trait TypeModMemberTrait {
    fn get_mut_type_mod(&mut self) -> &mut TypeMod;
    fn set_type_mod(&mut self, type_mod: TypeMod) -> IntrospectResult<()> {
        match mem::replace(self.get_mut_type_mod(), type_mod) {
            TypeMod::None => Ok(()),
            _ => Err(IntrospectError::MultipleTypeModifiers),
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
    fn get_type_mod(self) -> TypeMod;
    fn get_type_def(self, ty: &Ty) -> IntrospectResult<TypeDefVariant> {
        match self.get_type_mod() {
            TypeMod::Raw => match ty.get_primitive_type() {
                Some(CairoPrimitiveType::ByteArray) => {
                    Ok(TypeDefVariant::TypeDef(TypeDef::ByteArray))
                }
                Some(CairoPrimitiveType::Bytes31) => Ok(TypeDefVariant::TypeDef(TypeDef::Bytes31)),
                _ => Err(IntrospectError::UnsupportedRawType),
            },
            TypeMod::Encoded(encoded) => match ty.get_primitive_type() {
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

impl TypeModTrait for TypeMod {
    fn get_type_mod(self) -> TypeMod {
        self
    }
}

impl TypeModMemberTrait for TypeMod {
    fn get_mut_type_mod(&mut self) -> &mut TypeMod {
        self
    }
}

impl<T> AttributeParser<T> for TypeMod
where
    T: AttributesTrait,
{
    type Error = IntrospectError;
    fn parse_attribute(
        &mut self,
        _module: &mut T,
        attribute: Attribute,
    ) -> IntrospectResult<Vec<AttributeVariant>> {
        match self.extract_type_mod_return_empty(&attribute) {
            None => attribute.into(),
            Some(res) => res,
        }
    }
}

#[derive(Default)]
#[macro_attributes]
pub struct TypeModAndName {
    #[skip]
    pub type_mod: TypeMod,
    pub name: String,
}

impl TypeModTrait for TypeModAndName {
    fn get_type_mod(self) -> TypeMod {
        self.type_mod
    }
}

impl TypeModMemberTrait for TypeModAndName {
    fn get_mut_type_mod(&mut self) -> &mut TypeMod {
        &mut self.type_mod
    }
}

impl<SyntaxType: AttributesTrait> AttributeParser<SyntaxType> for TypeModAndName {
    type Error = IntrospectError;
    fn parse_attribute(
        &mut self,
        _module: &mut SyntaxType,
        attribute: Attribute,
    ) -> IntrospectResult<Vec<AttributeVariant>> {
        if let Some(r) = self.extract_type_mod_return_empty(&attribute) {
            return r.map_err(From::from);
        }
        match attribute.name.as_str() {
            "name" => self.set_name_return_empty(attribute.single_unnamed_arg()?),
            _ => attribute.into(),
        }
    }
}
