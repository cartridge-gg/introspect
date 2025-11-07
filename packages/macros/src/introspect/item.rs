use crate::introspect::IntrospectImpl;
use crate::{Enum, IntrospectError, Result, Struct};
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::kind::SyntaxKind;
use salsa::Database;

pub enum IntrospectItem<'db> {
    Struct(Struct<'db>),
    Enum(Enum<'db>),
}

impl<'db> IntrospectImpl for IntrospectItem<'db> {
    fn to_introspect_impl(&mut self) -> String {
        match self {
            IntrospectItem::Struct(struct_item) => struct_item.to_introspect_impl(),
            IntrospectItem::Enum(enum_item) => enum_item.to_introspect_impl(),
        }
    }
    fn to_introspect_ref_impl(&mut self) -> String {
        match self {
            IntrospectItem::Struct(struct_item) => struct_item.to_introspect_ref_impl(),
            IntrospectItem::Enum(enum_item) => enum_item.to_introspect_ref_impl(),
        }
    }
}

impl<'db> ToString for IntrospectItem<'db> {
    fn to_string(&self) -> String {
        match self {
            IntrospectItem::Struct(struct_item) => struct_item.to_string(),
            IntrospectItem::Enum(enum_item) => enum_item.to_string(),
        }
    }
}

pub fn get_introspection_type<'db>(
    db: &'db dyn Database,
    file: SyntaxNode<'db>,
) -> Result<IntrospectItem<'db>> {
    for child in file.get_children(db)[0].get_children(db) {
        let kind = (&child).kind(db);
        match kind {
            SyntaxKind::ItemStruct => {
                return Ok(IntrospectItem::Struct(Struct::from_syntax_node(db, *child)));
            }
            SyntaxKind::ItemEnum => {
                return Ok(IntrospectItem::Enum(Enum::from_syntax_node(db, *child)));
            }
            _ => continue,
        }
    }
    Err(IntrospectError::NoItem())
}
