use super::{IEnum, IExtract, IStruct};
use crate::i_type::{IAttributesTrait, INameTrait};
use crate::item::ItemTrait;
use crate::{IntrospectError, IntrospectResult};
use cairo_lang_macro::TokenStream;
use cairo_syntax_parser::item::item_from_token_stream;
use cairo_syntax_parser::{GenericParam, GenericParamsTrait, Item};

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
