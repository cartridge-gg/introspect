use super::{IEnum, IExtract, IStruct};
use crate::{IAttributesTrait, INameTrait, IntrospectError, IntrospectResult};
use cairo_lang_macro::TokenStream;
use cairo_syntax_parser::item::item_from_token_stream;
use cairo_syntax_parser::{CairoWriteSlice, GenericParam, GenericParamsTrait, Item};
use std::fmt::{Result as FmtResult, Write};

pub enum IntrospectItem {
    Struct(IStruct),
    Enum(IEnum),
}

impl IntrospectItem {
    pub fn from_token_stream(token_stream: TokenStream) -> IntrospectResult<Self> {
        let item = item_from_token_stream(token_stream);
        match item {
            Item::Struct(mut s) => IStruct::iextract(&mut s).map(IntrospectItem::Struct),
            Item::Enum(mut e) => IEnum::iextract(&mut e).map(IntrospectItem::Enum),
            _ => Err(IntrospectError::UnsupportedItem(item.kind().to_string())),
        }
    }
}

impl INameTrait for IntrospectItem {
    fn name(&self) -> &str {
        match self {
            IntrospectItem::Struct(s) => &s.name(),
            IntrospectItem::Enum(e) => &e.name(),
        }
    }
}

impl GenericParamsTrait for IntrospectItem {
    fn generic_params(&self) -> &Option<Vec<GenericParam>> {
        match self {
            IntrospectItem::Struct(s) => s.generic_params(),
            IntrospectItem::Enum(e) => e.generic_params(),
        }
    }
}

impl IAttributesTrait for IntrospectItem {
    fn iattributes(&self) -> &[crate::IAttribute] {
        match self {
            IntrospectItem::Struct(s) => s.iattributes(),
            IntrospectItem::Enum(e) => e.iattributes(),
        }
    }
}

impl ItemTrait for IntrospectItem {
    fn type_selector(&self) -> &'static str {
        match self {
            IntrospectItem::Struct(s) => s.type_selector(),
            IntrospectItem::Enum(e) => e.type_selector(),
        }
    }
}

pub trait ItemTrait: GenericParamsTrait + INameTrait {
    fn type_selector(&self) -> &'static str;
    fn write_name<W: Write>(&self, buf: &mut W) -> FmtResult {
        buf.write_str(self.name())
    }
    fn instantiated_name(&self) -> String {
        let mut name = String::new();
        self.write_instantiated_name(&mut name).unwrap();
        name
    }
    fn write_instantiated_name<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.write_name(buf)?;
        self.cwrite_generic_types(buf)
    }
    fn write_name_call<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.write_name(buf)?;
        self.cwrite_generic_types_call(buf)
    }
    fn write_generics_with_traits<W: Write>(&self, buf: &mut W, traits: &[&str]) -> FmtResult {
        let generic_types = self.generic_types();
        if let Some(generic_types) = generic_types {
            buf.write_char('<')?;
            generic_types.cwrite_csv(buf)?;
            for t in traits {
                generic_types
                    .iter()
                    .map(|g| write!(buf, ", +{t}<{g}>"))
                    .collect::<FmtResult>()?;
            }
            buf.write_char('>')?;
        }
        Ok(())
    }
    fn generics_with_traits(&self, traits: &[&str]) -> String {
        let mut buf = String::new();
        self.write_generics_with_traits(&mut buf, traits).unwrap();
        buf
    }
    fn generics_call(&self) -> String {
        let mut buf = String::new();
        self.cwrite_generic_types_call(&mut buf).unwrap();
        buf
    }
}
