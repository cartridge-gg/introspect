use super::{IEnum, IStruct};
use crate::i_type::{IExtract, ITys};
use crate::params::GenericParams;
use crate::{IntrospectError, IntrospectResult, Item, ItemTrait, Ty};
pub trait IntrospectItemTrait {
    type ModuleType;
    fn kind(&self) -> &str;
    fn child_types(&self) -> Vec<&Ty>;
    fn child_defs(&self, i_path: &str) -> String {
        self.child_types().collect_child_defs(i_path)
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

impl IExtract for IItem {
    type SyntaxType = Item;
    type Error = IntrospectError;
    fn iextract(item: &mut Item) -> IntrospectResult<IItem> {
        match item {
            Item::Struct(s) => IStruct::iextract(s).map(IItem::Struct),
            Item::Enum(e) => IEnum::iextract(e).map(IItem::Enum),
        }
    }
}
