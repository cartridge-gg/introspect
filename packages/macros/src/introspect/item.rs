use crate::introspect::IntrospectImpl;
use crate::items::IntrospectItemTrait;
use crate::params::GenericParams;
use crate::{AsCairo, Enum, IntrospectError, Result, Struct, TryFromAst};
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::kind::SyntaxKind;
use salsa::Database;

pub enum IntrospectItem {
    Struct(Struct),
    Enum(Enum),
}

impl IntrospectImpl for IntrospectItem {
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

impl AsCairo for IntrospectItem {
    fn as_cairo(&self) -> String {
        match self {
            IntrospectItem::Struct(struct_item) => struct_item.as_cairo(),
            IntrospectItem::Enum(enum_item) => enum_item.as_cairo(),
        }
    }
}

pub fn get_introspection_type<'db>(
    db: &'db dyn Database,
    file: SyntaxNode<'db>,
) -> Result<IntrospectItem> {
    for child in file.get_children(db)[0].get_children(db) {
        let kind = (&child).kind(db);
        match kind {
            SyntaxKind::ItemStruct => {
                return Ok(IntrospectItem::Struct(Struct::try_from_syntax_node(
                    db, *child,
                )?));
            }
            SyntaxKind::ItemEnum => {
                return Ok(IntrospectItem::Enum(Enum::try_from_syntax_node(
                    db, *child,
                )?));
            }
            _ => continue,
        }
    }
    Err(IntrospectError::NoItem())
}

impl IntrospectItemTrait for IntrospectItem {
    fn kind(&self) -> &str {
        match self {
            IntrospectItem::Struct(s) => s.kind(),
            IntrospectItem::Enum(e) => e.kind(),
        }
    }
    fn name(&self) -> &str {
        match self {
            IntrospectItem::Struct(s) => s.name(),
            IntrospectItem::Enum(e) => e.name(),
        }
    }
    fn generic_params(&self) -> &GenericParams {
        match self {
            IntrospectItem::Struct(s) => s.generic_params(),
            IntrospectItem::Enum(e) => e.generic_params(),
        }
    }
    fn child_defs(&self) -> Vec<String> {
        match self {
            IntrospectItem::Struct(s) => s.child_defs(),
            IntrospectItem::Enum(e) => e.child_defs(),
        }
    }
}
