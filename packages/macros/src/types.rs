use crate::{Enum, IntrospectError, Result, Struct};
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::kind::SyntaxKind;
use salsa::Database;

pub enum IntrospectType<'db> {
    Struct(Struct<'db>),
    Enum(Enum<'db>),
}

pub fn get_introspection_type<'db>(
    db: &'db dyn Database,
    file: SyntaxNode<'db>,
) -> Result<IntrospectType<'db>> {
    let item = file.get_children(db)[0].get_children(db)[0];
    let kind = item.kind(db);
    match kind {
        SyntaxKind::ItemStruct => Ok(IntrospectType::Struct(Struct::from_syntax_node(db, item))),
        SyntaxKind::ItemEnum => Ok(IntrospectType::Enum(Enum::from_syntax_node(db, item))),
        _ => Err(IntrospectError::UnsupportedItem(kind.to_string())),
    }
}
