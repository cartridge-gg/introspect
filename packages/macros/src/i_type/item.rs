use super::{IEnum, IStruct};
use crate::i_type::{DefaultIExtractor, IExtract, ITys};
use crate::params::GenericParams;
use crate::{IntrospectError, Item, ItemTrait, IntrospectResult, Ty};
pub trait IntrospectItemTrait {
    type ModuleType;
    fn kind(&self) -> &str;
    fn child_types(&self) -> Vec<&Ty>;
    fn child_defs(&self) -> String {
        self.child_types().child_defs()
    }
}

pub enum IItem {
    Struct(IStruct),
    Enum(IEnum),
}

impl ItemTrait for IItem {
    fn name(&self) -> &str {
        match self {
            IItem::Struct(s) => s.name(),
            IItem::Enum(e) => e.name(),
        }
    }
    fn generic_params(&self) -> &GenericParams {
        match self {
            IItem::Struct(s) => s.generic_params(),
            IItem::Enum(e) => e.generic_params(),
        }
    }
}

impl IntrospectItemTrait for IItem {
    type ModuleType = Item;
    fn kind(&self) -> &str {
        match self {
            IItem::Struct(s) => s.kind(),
            IItem::Enum(e) => e.kind(),
        }
    }
    fn child_types(&self) -> Vec<&Ty> {
        match self {
            IItem::Struct(s) => s.child_types(),
            IItem::Enum(e) => e.child_types(),
        }
    }
}

impl IExtract<IItem> for DefaultIExtractor {
    type SyntaxType = Item;
    type Error = IntrospectError;
    fn iextract(&self, item: &mut Item) -> IntrospectResult<IItem> {
        match item {
            Item::Struct(s) => self.iextract(s).map(IItem::Struct),
            Item::Enum(e) => self.iextract(e).map(IItem::Enum),
        }
    }
}
