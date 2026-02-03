use super::{IEnum, IExtract, IStruct};
use crate::item::ItemTrait;
use crate::{IntrospectError, IntrospectResult};
use cairo_lang_macro::TokenStream;
use cairo_syntax_parser::item::item_from_token_stream;
use cairo_syntax_parser::{GenericParam, GenericParamsTrait, Item, NameTrait};

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

impl NameTrait for IntrospectItem {
    fn name(&self) -> &str {
        match self {
            IntrospectItem::Struct(s) => &s.name(),
            IntrospectItem::Enum(e) => &e.name(),
        }
    }
    fn set_name(&mut self, new_name: String) {
        match self {
            IntrospectItem::Struct(s) => s.set_name(new_name),
            IntrospectItem::Enum(e) => e.set_name(new_name),
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

impl ItemTrait for IntrospectItem {
    fn type_selector(&self) -> &'static str {
        match self {
            IntrospectItem::Struct(s) => s.type_selector(),
            IntrospectItem::Enum(e) => e.type_selector(),
        }
    }
}

pub trait IFieldTrait {
    fn field(&self) -> &str;
    fn name(&self) -> &str;
    fn ty(&self) -> &str;
}

pub trait IFieldsTrait {
    type Field: IFieldTrait;
    fn fields(&self) -> &[Self::Field];
    fn field_fields(&self) -> Vec<&str> {
        self.fields()
            .iter()
            .map(IFieldTrait::field)
            .collect::<Vec<&str>>()
    }
}
