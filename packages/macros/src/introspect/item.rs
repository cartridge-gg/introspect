// use crate::IItem;
// use crate::introspect::IntrospectImpl;

// impl IntrospectImpl for IItem {
//     fn to_introspect_impl<const IS_REF: bool>(&self, i_path: &str) -> String {
//         match self {
//             IItem::Struct(struct_item) => struct_item.to_introspect_impl::<IS_REF>(i_path),
//             IItem::Enum(enum_item) => enum_item.to_introspect_impl::<IS_REF>(i_path),
//         }
//     }
// }

use std::{fmt::Write, sync::Arc};

use cairo_syntax_parser::{CairoWrite, Visibility};

use crate::{IEnum, IStruct};

pub enum IntrospectItem {
    Struct(IStruct),
    Enum(IEnum),
}

pub struct IntrospectImpl {
    visibility: Arc<Visibility>,
    name: String,
    generic_types: Vec<_>,
    item: IntrospectItem,
}

impl CairoWrite for IntrospectImpl {
    fn cfmt<W: Write>(&self, buf: &mut W) -> std::fmt::Result {}
}

trait IntrospectImplFormat{
    fn cfmt_introspect_impl<W: Write>(&self,  buf: &mut W)
}
        